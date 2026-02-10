use crate::edge::Edge;
use crate::node_builder::NodeInstance;
use crate::node_builder::{ NodeTypeCatalog, ParamDef, ParamSource, ParamType, RenderHint };
use serde::Serialize;

fn render_non_table_rhs(value: &toml::Value) -> Result<String, String> {
    // Leverage the TOML serializer for correct quoting/number formatting.
    // We serialize a dummy table `v = <value>` and strip the left-hand side.
    let mut singleton = toml::map::Map::new();
    singleton.insert("v".to_string(), value.clone());
    let rendered = toml::to_string(&toml::Value::Table(singleton)).map_err(|e| e.to_string())?;
    let line = rendered
        .lines()
        .next()
        .ok_or_else(|| "failed to render value".to_string())?;
    line.strip_prefix("v = ")
        .map(|s| s.to_string())
        .ok_or_else(|| "unexpected rendered format".to_string())
}

fn render_inline_table(table: &toml::map::Map<String, toml::Value>) -> Result<String, String> {
    let mut keys: Vec<&String> = table.keys().collect();
    keys.sort();
    let mut parts: Vec<String> = Vec::with_capacity(keys.len());
    for key in keys {
        let value = &table[key];
        let rhs = match value {
            toml::Value::Table(inner) => render_inline_table(inner)?,
            other => render_non_table_rhs(other)?,
        };
        parts.push(format!("{} = {}", key, rhs));
    }
    Ok(format!("{{ {} }}", parts.join(", ")))
}

fn render_assignment(key: &str, value: &toml::Value) -> Result<String, String> {
    let rhs = match value {
        toml::Value::Table(table) => render_inline_table(table)?,
        other => render_non_table_rhs(other)?,
    };
    Ok(format!("{} = {}\n", key, rhs))
}
#[derive(Debug, Serialize)]
pub struct Graph {
    pub nodes: Vec<NodeInstance>,
    pub edges: Vec<Edge>,
}

fn incoming_edge_labels(graph: &Graph, node_id: usize) -> Vec<String> {
    let mut incoming: Vec<(u64, String)> = graph.edges
        .iter()
        .filter(|e| e.to == (node_id as u64))
        .map(|e| (e.id, e.label.clone()))
        .collect();
    incoming.sort_by_key(|(id, _)| *id);
    incoming
        .into_iter()
        .map(|(_, l)| l)
        .collect()
}

fn outgoing_edge_labels(graph: &Graph, node_id: usize) -> Vec<String> {
    let mut outgoing: Vec<(u64, String)> = graph.edges
        .iter()
        .filter(|e| e.from == (node_id as u64))
        .map(|e| (e.id, e.label.clone()))
        .collect();
    outgoing.sort_by_key(|(id, _)| *id);
    outgoing
        .into_iter()
        .map(|(_, l)| l)
        .collect()
}

fn value_from_source(
    graph: &Graph,
    node: &NodeInstance,
    source: &ParamSource
) -> Option<toml::Value> {
    match source {
        ParamSource::NodeLabel => Some(toml::Value::String(node.label.clone())),
        ParamSource::IncomingEdgeLabels => {
            let labels = incoming_edge_labels(graph, node.id);
            Some(toml::Value::Array(labels.into_iter().map(toml::Value::String).collect()))
        }
        ParamSource::OutgoingEdgeLabels => {
            let labels = outgoing_edge_labels(graph, node.id);
            Some(toml::Value::Array(labels.into_iter().map(toml::Value::String).collect()))
        }
        ParamSource::IncomingEdgeLabel { index } => {
            let labels = incoming_edge_labels(graph, node.id);
            let idx = index.unwrap_or(0);
            labels.get(idx).cloned().map(toml::Value::String)
        }
        ParamSource::OutgoingEdgeLabel { index } => {
            let labels = outgoing_edge_labels(graph, node.id);
            let idx = index.unwrap_or(0);
            labels.get(idx).cloned().map(toml::Value::String)
        }
    }
}

fn apply_render_hint(value: toml::Value, def: Option<&ParamDef>) -> toml::Value {
    let Some(def) = def else {
        return value;
    };

    // If template explicitly tells us how to render, do that.
    if let Some(render) = &def.render {
        return match render {
            RenderHint::Scalar =>
                match value {
                    toml::Value::Array(mut items) if items.len() == 1 => items.remove(0),
                    other => other,
                }
            RenderHint::List =>
                match value {
                    toml::Value::Array(_) => value,
                    other => toml::Value::Array(vec![other]),
                }
        };
    }

    // Default behavior: if the template defines a list of len 1, render as scalar.
    if matches!(def.kind, ParamType::List) && def.len == Some(1) {
        return match value {
            toml::Value::Array(mut items) if items.len() == 1 => items.remove(0),
            other => other,
        };
    }

    value
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: NodeInstance) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_node(&self, id: usize) -> Option<&NodeInstance> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn get_edge(&self, id: u64) -> Option<&Edge> {
        self.edges.iter().find(|edge| edge.id == id)
    }

    pub fn get_new_node_id(&self) -> usize {
        self.nodes
            .iter()
            .map(|node| node.id)
            .max()
            .unwrap_or(0) + 1
    }

    /// Serialize this graph into a TOML document driven entirely by `catalog`.
    ///
    /// The template controls:
    /// - root table name (`format.root`)
    /// - which params exist per type
    /// - how to derive missing params (`params.<k>.source`)
    /// - how to render values (`params.<k>.render`, or default len=1 list => scalar)
    pub fn to_template_toml_value(&self, catalog: &NodeTypeCatalog) -> toml::Value {
        let root_key = catalog.format
            .as_ref()
            .map(|f| f.root.as_str())
            .unwrap_or("units");

        let mut root = toml::map::Map::new();
        let mut root_table = toml::map::Map::new();

        // Deterministic output: preserve editing intent by ordering by node id.
        let mut indices: Vec<usize> = (0..self.nodes.len()).collect();
        indices.sort_by_key(|i| self.nodes[*i].id);

        for i in indices {
            let node = &self.nodes[i];
            let mut table = toml::map::Map::new();

            if let Some(type_def) = catalog.nodes.types.get(node.type_.as_str()) {
                for (key, def) in &type_def.params {
                    let value = if let Some(existing) = node.values.get(key) {
                        Some(existing.clone())
                    } else if let Some(source) = &def.source {
                        value_from_source(self, node, source)
                    } else {
                        None
                    };

                    if let Some(value) = value {
                        let rendered = apply_render_hint(value, Some(def));
                        table.insert(key.clone(), rendered);
                    }
                }
            } else {
                // Unknown type: only serialize what exists on the node.
                for (k, v) in &node.values {
                    table.insert(k.clone(), v.clone());
                }
            }

            let unit_value = toml::Value::Table(table);
            let entry = root_table
                .entry(node.type_.clone())
                .or_insert_with(|| toml::Value::Array(Vec::new()));
            if let toml::Value::Array(arr) = entry {
                arr.push(unit_value);
            }
        }

        root.insert(root_key.to_string(), toml::Value::Table(root_table));
        toml::Value::Table(root)
    }

    pub fn to_template_toml_string(&self, catalog: &NodeTypeCatalog) -> Result<String, String> {
        let root_key = catalog.format
            .as_ref()
            .map(|f| f.root.as_str())
            .unwrap_or("units");

        let doc = self.to_template_toml_value(catalog);
        let root_table = doc
            .get(root_key)
            .and_then(|v| v.as_table())
            .ok_or_else(|| format!("missing {} table", root_key))?;

        let mut out = String::new();
        out.push_str(&format!("[{}]\n\n", root_key));

        // Stable ordering: type key sorted, then insertion order within arrays.
        let mut type_keys: Vec<&String> = root_table.keys().collect();
        type_keys.sort();

        for type_key in type_keys {
            let Some(arr) = root_table.get(type_key).and_then(|v| v.as_array()) else {
                continue;
            };

            // Optional per-type ordering; otherwise sort keys.
            let order = catalog.nodes.types.get(type_key.as_str()).and_then(|t| t.order.clone());

            for entry in arr {
                let Some(table) = entry.as_table() else {
                    continue;
                };
                out.push_str(&format!("[[{}.{}]]\n", root_key, type_key));

                let mut keys: Vec<&String> = table.keys().collect();
                match &order {
                    Some(order) => {
                        keys.sort_by_key(|k| {
                            order
                                .iter()
                                .position(|o| o == k.as_str())
                                .unwrap_or(usize::MAX)
                        });
                    }
                    None => keys.sort(),
                }

                for key in keys {
                    let value = &table[key];
                    out.push_str(&render_assignment(key, value)?);
                }
                out.push('\n');
            }
        }

        Ok(out)
    }

    // Back-compat wrapper (old name, now fully template-driven)
    pub fn to_units_toml_string(&self, catalog: &NodeTypeCatalog) -> Result<String, String> {
        self.to_template_toml_string(catalog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_builder::NodeTypeCatalog;

    fn default_catalog() -> NodeTypeCatalog {
        let raw = include_str!("../templates/units.toml");
        toml::from_str(raw).expect("template catalog parses")
    }

    #[test]
    fn units_toml_matches_ltp_shape_for_len1_io_and_coords() {
        let catalog = default_catalog();
        let mut g = Graph::new();

        let mut n1 = NodeInstance::new(1, "cstr".to_string(), "lane1.t1".to_string());
        let mut coords = toml::map::Map::new();
        coords.insert("x".to_string(), toml::Value::Float(0.0));
        coords.insert("y".to_string(), toml::Value::Float(0.0));
        n1.values.insert("coords".to_string(), toml::Value::Table(coords));
        g.add_node(n1);

        let n2 = NodeInstance::new(2, "sensor".to_string(), "lane1.t1_sensor".to_string());
        g.add_node(n2);

        g.add_edge(Edge {
            id: 1,
            from: 1,
            to: 2,
            label: "lane1_t1_out".to_string(),
        });

        let out = g.to_template_toml_string(&catalog).unwrap();
        let parsed: toml::Value = toml::from_str(&out).expect("output is valid toml");
        let root_key = catalog.format
            .as_ref()
            .map(|f| f.root.as_str())
            .unwrap_or("units");
        let root = parsed
            .get(root_key)
            .and_then(|v| v.as_table())
            .expect("root table");

        let cstr_entries = root
            .get("cstr")
            .and_then(|v| v.as_array())
            .expect("cstr array");
        let c0 = cstr_entries[0].as_table().expect("cstr table");

        // name is derived from node_label via template
        assert_eq!(
            c0.get("name").and_then(|v| v.as_str()),
            Some("lane1.t1")
        );
        // out is derived from outgoing_edge_label and rendered as scalar (per template)
        let out_val = c0.get("out");
        assert_eq!(
            out_val.and_then(|v| v.as_str()),
            Some("lane1_t1_out"),
            "unexpected out value: {out_val:?}\nTOML:\n{out}"
        );
        // coords roundtrips
        let coords = c0
            .get("coords")
            .and_then(|v| v.as_table())
            .expect("coords table");
        assert!(coords.get("x").unwrap().as_float().unwrap() == 0.0);

        let sensor_entries = root
            .get("sensor")
            .and_then(|v| v.as_array())
            .expect("sensor array");
        let s0 = sensor_entries[0].as_table().expect("sensor table");
        let in_val = s0.get("in");
        assert_eq!(
            in_val.and_then(|v| v.as_str()),
            Some("lane1_t1_out"),
            "unexpected in value: {in_val:?}\nTOML:\n{out}"
        );
    }
}
