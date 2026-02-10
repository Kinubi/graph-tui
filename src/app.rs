use crossterm::event::{ KeyCode, KeyEvent };
use std::collections::HashMap;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Graph,
    GraphEditor,
    NodeEditor,
    EdgeEditor,
    Exiting,
}

#[derive(Debug)]
pub enum NodeEditorMode {
    Type,
    Label,
    Param,
}

#[derive(Debug)]
pub enum EdgeEditorMode {
    Label,
    InOuts(InOut),
}

#[derive(Debug)]
pub enum InOut {
    From,
    To,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Node(NodeEditorMode),
    Edge(EdgeEditorMode),
}

use crate::graph::Graph;
use crate::node_builder::{
    NodeInstance,
    NodeTypeCatalog,
    NodeTypeDef,
    NodeTypesSection,
    ParamDef,
    ParamType,
    ValueType,
};
#[derive(Debug)]
pub struct App {
    pub graph: Graph,
    pub node_catalog: NodeTypeCatalog,
    pub node_type_keys: Vec<String>,
    pub node_type_index: usize,
    pub exit: bool,
    pub label: String,
    pub in_outs: [u64; 2],
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub node_edit: Option<NodeEditState>,
}

impl App {
    pub fn new() -> Self {
        Self::new_with_catalog(load_node_catalog_default())
    }

    pub fn new_with_catalog(node_catalog: NodeTypeCatalog) -> Self {
        let node_type_keys = node_type_keys(&node_catalog);
        Self {
            graph: Graph::new(),
            node_catalog,
            node_type_keys,
            node_type_index: 0,
            exit: false,
            label: String::new(),
            in_outs: [0, 0],
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            node_edit: None,
        }
    }

    pub fn update(&mut self) {
        todo!();
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        // Handle screen navigation and business logic
        match self.current_screen {
            CurrentScreen::Main => {
                match key.code {
                    KeyCode::Char('g') | KeyCode::Char('G') => {
                        self.current_screen = CurrentScreen::Graph;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }
            }
            CurrentScreen::Graph => {
                match key.code {
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
            }
            CurrentScreen::GraphEditor => {
                match key.code {
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.start_node_editor();
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        self.label.clear();
                        self.in_outs = [0, 0];
                        self.currently_editing = Some(
                            CurrentlyEditing::Edge(EdgeEditorMode::Label)
                        );
                        self.current_screen = CurrentScreen::EdgeEditor;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.current_screen = CurrentScreen::Graph;
                    }
                    _ => {}
                }
            }
            CurrentScreen::NodeEditor => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.label.clear();
                        self.currently_editing = None;
                        self.current_screen = CurrentScreen::GraphEditor;
                        self.node_edit = None;
                    }
                    KeyCode::Enter => {
                        match &self.currently_editing {
                            Some(CurrentlyEditing::Node(NodeEditorMode::Type)) => {
                                self.select_current_type();
                            }
                            _ => {
                                self.advance_node_editor();
                            }
                        }
                    }
                    KeyCode::Up => {
                        if
                            matches!(
                                self.currently_editing,
                                Some(CurrentlyEditing::Node(NodeEditorMode::Type))
                            )
                        {
                            self.move_type_selection(-1);
                        }
                    }
                    KeyCode::Down => {
                        if
                            matches!(
                                self.currently_editing,
                                Some(CurrentlyEditing::Node(NodeEditorMode::Type))
                            )
                        {
                            self.move_type_selection(1);
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(CurrentlyEditing::Node(mode)) = &self.currently_editing {
                            match mode {
                                NodeEditorMode::Type => {}
                                NodeEditorMode::Label => {
                                    self.label.pop();
                                }
                                NodeEditorMode::Param => {
                                    if let Some(edit) = &mut self.node_edit {
                                        edit.buffer.pop();
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        self.currently_editing = None;
                        self.label.clear();
                        self.current_screen = CurrentScreen::GraphEditor;
                        self.node_edit = None;
                    }
                    KeyCode::Char(value) => {
                        if let Some(CurrentlyEditing::Node(mode)) = &self.currently_editing {
                            match mode {
                                NodeEditorMode::Type => {}
                                NodeEditorMode::Label => {
                                    self.label.push(value);
                                }
                                NodeEditorMode::Param => {
                                    if let Some(edit) = &mut self.node_edit {
                                        edit.buffer.push(value);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            CurrentScreen::EdgeEditor => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.label.clear();
                        self.in_outs = [0, 0];
                        self.currently_editing = None;
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Enter => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::To))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.add_edge();
                                    self.label.clear();
                                    self.in_outs = [0, 0];
                                    self.currently_editing = None;
                                    self.current_screen = CurrentScreen::GraphEditor;
                                }
                            }
                        }
                    }
                    KeyCode::Tab => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::To))
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::Label)
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.label.pop();
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.in_outs[0] = 0;
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::Label)
                                    );
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.in_outs[1] = 0;
                                    self.currently_editing = Some(
                                        CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        self.currently_editing = None;
                        self.label.clear();
                        self.in_outs = [0, 0];
                        self.current_screen = CurrentScreen::GraphEditor;
                    }
                    KeyCode::Char(value) => {
                        if let Some(CurrentlyEditing::Edge(mode)) = &self.currently_editing {
                            match mode {
                                EdgeEditorMode::Label => {
                                    self.label.push(value);
                                }
                                EdgeEditorMode::InOuts(InOut::From) => {
                                    self.in_outs[0] = value.to_digit(10).unwrap_or(0) as u64;
                                }
                                EdgeEditorMode::InOuts(InOut::To) => {
                                    self.in_outs[1] = value.to_digit(10).unwrap_or(0) as u64;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            CurrentScreen::Exiting => {
                match key.code {
                    KeyCode::Char('y') => {
                        self.exit = true;
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn add_node(&mut self) {
        let id = self.graph.get_new_node_id();
        let type_name = self.current_type_name().unwrap_or("unknown");
        let instance = NodeInstance::new(id, type_name.to_string(), self.label.clone());
        self.graph.add_node(instance);
    }

    fn start_node_editor(&mut self) {
        self.label.clear();
        self.node_edit = None;
        self.node_type_index = 0;
        if self.node_type_keys.is_empty() {
            self.currently_editing = Some(CurrentlyEditing::Node(NodeEditorMode::Label));
        } else {
            self.currently_editing = Some(CurrentlyEditing::Node(NodeEditorMode::Type));
        }
        self.current_screen = CurrentScreen::NodeEditor;
    }

    fn advance_node_editor(&mut self) {
        match &self.currently_editing {
            Some(CurrentlyEditing::Node(NodeEditorMode::Label)) => {
                if let Some(edit) = &self.node_edit {
                    if edit.has_params() {
                        self.currently_editing = Some(
                            CurrentlyEditing::Node(NodeEditorMode::Param)
                        );
                        return;
                    }
                }
                self.finalize_node_edit();
            }
            Some(CurrentlyEditing::Node(NodeEditorMode::Param)) => {
                if self.commit_current_param() {
                    if let Some(edit) = &mut self.node_edit {
                        edit.advance();
                        if edit.is_done() {
                            self.finalize_node_edit();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn commit_current_param(&mut self) -> bool {
        let Some(edit) = &mut self.node_edit else {
            return true;
        };
        let Some(param_def) = edit.current_def() else {
            return true;
        };
        if edit.buffer.trim().is_empty() {
            edit.error = None;
            return true;
        }
        let result = parse_param_value(&edit.buffer, param_def);
        match result {
            Ok(value) => {
                if edit.current_key() == Some("name") {
                    if let toml::Value::String(name) = &value {
                        self.label = name.clone();
                    }
                }
                edit.set_value(value);
                edit.buffer.clear();
                edit.error = None;
                true
            }
            Err(message) => {
                edit.error = Some(message);
                false
            }
        }
    }

    fn finalize_node_edit(&mut self) {
        let id = self.graph.get_new_node_id();
        let instance = if let Some(edit) = &self.node_edit {
            let mut instance = NodeInstance::new(id, edit.type_name.clone(), self.label.clone());
            instance.values = edit.values.clone();
            instance
        } else {
            let type_name = self.current_type_name().unwrap_or("unknown");
            NodeInstance::new(id, type_name.to_string(), self.label.clone())
        };
        self.graph.add_node(instance);
        self.label.clear();
        self.current_screen = CurrentScreen::GraphEditor;
        self.currently_editing = None;
        self.node_edit = None;
    }

    pub fn current_type_name(&self) -> Option<&str> {
        self.node_type_keys.get(self.node_type_index).map(|name| name.as_str())
    }

    fn move_type_selection(&mut self, delta: i32) {
        if self.node_type_keys.is_empty() {
            return;
        }
        let len = self.node_type_keys.len() as i32;
        let mut next = (self.node_type_index as i32) + delta;
        if next < 0 {
            next = len - 1;
        }
        if next >= len {
            next = 0;
        }
        self.node_type_index = next as usize;
    }

    fn select_current_type(&mut self) {
        if let Some(type_name) = self.current_type_name() {
            if let Some(def) = self.node_catalog.nodes.types.get(type_name) {
                self.node_edit = Some(NodeEditState::new(type_name.to_string(), def));
            }
        }
        self.currently_editing = Some(CurrentlyEditing::Node(NodeEditorMode::Label));
    }

    pub fn add_edge(&mut self) {
        let from = self.in_outs[0];
        let to = self.in_outs[1];
        let id = (self.graph.edges.len() as u64) + 1;
        let label = self.label.clone();
        self.graph.add_edge(crate::edge::Edge {
            id,
            from,
            to,
            label: label.clone(),
        });
        self.apply_edge_to_node_io(from, to, &label);
    }

    pub fn on_tick(&mut self) {
        self.update();
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn print_nodes(&self) {
        for node in &self.graph.nodes {
            println!("Node {}: {}", node.id, node.label);
        }
    }
}

fn apply_list_value(node: &mut NodeInstance, key: &str, value: &str) {
    if value.is_empty() {
        return;
    }
    let entry = toml::Value::String(value.to_string());
    match node.values.get_mut(key) {
        Some(toml::Value::Array(items)) => {
            if !items.iter().any(|item| item.as_str() == Some(value)) {
                items.push(entry);
            }
        }
        Some(toml::Value::String(existing)) => {
            if existing != value {
                let items = vec![toml::Value::String(existing.clone()), entry];
                node.values.insert(key.to_string(), toml::Value::Array(items));
            }
        }
        _ => {
            node.values.insert(key.to_string(), toml::Value::Array(vec![entry]));
        }
    }
}

fn update_node_io(nodes: &mut [NodeInstance], node_id: u64, key: &str, stream: &str) {
    let Ok(node_id) = usize::try_from(node_id) else {
        return;
    };
    if let Some(node) = nodes.iter_mut().find(|node| node.id == node_id) {
        apply_list_value(node, key, stream);
    }
}

impl App {
    fn apply_edge_to_node_io(&mut self, from: u64, to: u64, label: &str) {
        update_node_io(&mut self.graph.nodes, from, "out", label);
        update_node_io(&mut self.graph.nodes, to, "in", label);
    }
}

#[derive(Debug)]
pub struct NodeEditState {
    pub type_name: String,
    pub param_keys: Vec<String>,
    pub index: usize,
    pub buffer: String,
    pub values: HashMap<String, toml::Value>,
    pub error: Option<String>,
    params: HashMap<String, ParamDef>,
}

impl NodeEditState {
    pub fn new(type_name: String, def: &NodeTypeDef) -> Self {
        let mut param_keys: Vec<String> = def.params
            .keys()
            .filter(|key| {
                let key = key.as_str();
                key != "name" && key != "in" && key != "out" && key != "ins" && key != "outs"
            })
            .cloned()
            .collect();
        param_keys.sort();
        Self {
            type_name,
            param_keys,
            index: 0,
            buffer: String::new(),
            values: HashMap::new(),
            error: None,
            params: def.params.clone(),
        }
    }

    pub fn has_params(&self) -> bool {
        !self.param_keys.is_empty()
    }

    pub fn current_key(&self) -> Option<&str> {
        self.param_keys.get(self.index).map(|key| key.as_str())
    }

    pub fn current_def(&self) -> Option<&ParamDef> {
        let key = self.current_key()?;
        self.params.get(key)
    }

    pub fn set_value(&mut self, value: toml::Value) {
        if let Some(key) = self.current_key() {
            self.values.insert(key.to_string(), value);
        }
    }

    pub fn advance(&mut self) {
        if self.index < self.param_keys.len() {
            self.index += 1;
        }
    }

    pub fn is_done(&self) -> bool {
        self.index >= self.param_keys.len()
    }
}

fn parse_param_value(raw: &str, def: &ParamDef) -> Result<toml::Value, String> {
    match def.kind {
        ParamType::String => Ok(toml::Value::String(raw.to_string())),
        ParamType::Float => {
            let value = raw.parse::<f64>().map_err(|_| "expected float".to_string())?;
            Ok(toml::Value::Float(value))
        }
        ParamType::Bool => {
            let value = raw.parse::<bool>().map_err(|_| "expected true/false".to_string())?;
            Ok(toml::Value::Boolean(value))
        }
        ParamType::List => {
            let value = match parse_toml_value(raw) {
                Ok(value) if value.is_array() => value,
                _ => parse_list_from_raw(raw, def)?,
            };
            if let Some(expected) = def.len {
                let len = value
                    .as_array()
                    .map(|arr| arr.len())
                    .unwrap_or(0);
                if len != expected {
                    return Err(format!("expected list length {}", expected));
                }
            }
            validate_value_type(&value, def.value_type.as_ref())?;
            Ok(value)
        }
        ParamType::Table => {
            let value = match parse_toml_value(raw) {
                Ok(value) if value.is_table() => value,
                _ => parse_table_from_raw(raw, def)?,
            };
            Ok(value)
        }
    }
}

fn parse_toml_value(raw: &str) -> Result<toml::Value, String> {
    let wrapped = format!("value = {}", raw);
    let value: toml::Value = toml::from_str(&wrapped).map_err(|err| err.to_string())?;
    value
        .get("value")
        .cloned()
        .ok_or_else(|| "missing value".to_string())
}

fn parse_list_from_raw(raw: &str, def: &ParamDef) -> Result<toml::Value, String> {
    let parts = split_list_parts(raw);
    if parts.is_empty() {
        return Err("expected list".to_string());
    }
    let mut values = Vec::new();
    for part in parts {
        values.push(parse_list_entry(part, def.value_type.as_ref())?);
    }
    Ok(toml::Value::Array(values))
}

fn split_list_parts(raw: &str) -> Vec<&str> {
    if raw.contains(',') {
        raw.split(',')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .collect()
    } else {
        raw.split_whitespace().collect()
    }
}

fn parse_table_from_raw(raw: &str, def: &ParamDef) -> Result<toml::Value, String> {
    let trimmed = raw.trim();
    if let Some(table) = parse_xy_table(trimmed, def) {
        return Ok(table);
    }
    let wrapped = if trimmed.starts_with('{') && trimmed.ends_with('}') {
        format!("value = {}", trimmed)
    } else {
        format!("value = {{ {} }}", trimmed)
    };
    let value: toml::Value = toml::from_str(&wrapped).map_err(|err| err.to_string())?;
    value
        .get("value")
        .cloned()
        .ok_or_else(|| "missing value".to_string())
}

fn parse_xy_table(raw: &str, def: &ParamDef) -> Option<toml::Value> {
    if def.value_type.as_ref()? != &ValueType::Float {
        return None;
    }
    let parts = split_list_parts(raw);
    if parts.len() != 2 {
        return None;
    }
    let x = parts[0].parse::<f64>().ok()?;
    let y = parts[1].parse::<f64>().ok()?;
    let mut table = toml::map::Map::new();
    table.insert("x".to_string(), toml::Value::Float(x));
    table.insert("y".to_string(), toml::Value::Float(y));
    Some(toml::Value::Table(table))
}

fn parse_list_entry(raw: &str, value_type: Option<&ValueType>) -> Result<toml::Value, String> {
    match value_type {
        Some(ValueType::String) => Ok(toml::Value::String(unquote(raw))),
        Some(ValueType::Float) => {
            let value = raw.parse::<f64>().map_err(|_| "expected float".to_string())?;
            Ok(toml::Value::Float(value))
        }
        Some(ValueType::Bool) => {
            let value = raw.parse::<bool>().map_err(|_| "expected true/false".to_string())?;
            Ok(toml::Value::Boolean(value))
        }
        Some(ValueType::Any) | None => {
            parse_toml_value(raw).or_else(|_| Ok(toml::Value::String(unquote(raw))))
        }
    }
}

fn unquote(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn validate_value_type(value: &toml::Value, value_type: Option<&ValueType>) -> Result<(), String> {
    let Some(value_type) = value_type else {
        return Ok(());
    };
    if matches!(value_type, ValueType::Any) {
        return Ok(());
    }
    let Some(values) = value.as_array() else {
        return Err("expected list".to_string());
    };
    let valid = values.iter().all(|entry| {
        match value_type {
            ValueType::String => entry.is_str(),
            ValueType::Float => entry.is_float() || entry.is_integer(),
            ValueType::Bool => entry.is_bool(),
            ValueType::Any => true,
        }
    });
    if valid {
        Ok(())
    } else {
        Err("list values do not match type".to_string())
    }
}

pub fn load_node_catalog_from_path(path: &str) -> Result<NodeTypeCatalog, String> {
    let raw = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    toml::from_str(&raw).map_err(|err| err.to_string())
}

fn load_node_catalog_default() -> NodeTypeCatalog {
    let raw = include_str!("../templates/units.toml");
    toml::from_str(raw).unwrap_or_else(|_| NodeTypeCatalog {
        format: None,
        nodes: NodeTypesSection { types: HashMap::new() },
    })
}

fn node_type_keys(catalog: &NodeTypeCatalog) -> Vec<String> {
    let mut keys: Vec<String> = catalog.nodes.types.keys().cloned().collect();
    keys.sort();
    keys
}

pub fn write_graph_to_path(
    path: &str,
    graph: &Graph,
    catalog: &NodeTypeCatalog
) -> Result<(), String> {
    let string_result = graph.to_units_toml_string(catalog);
    match string_result {
        Ok(string) => {
            let result = std::fs::write(&path, &string);
            match result {
                Ok(()) => {
                    return Ok(());
                }
                Err(error) => {
                    return Err(error.to_string());
                }
            }
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }
}
