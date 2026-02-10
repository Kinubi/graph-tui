use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeSchema {
    pub type_: String,
    pub params: HashMap<String, ParamDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInstance {
    pub id: usize,
    pub type_: String,
    pub label: String,
    pub values: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeInstance {}

impl NodeInstance {
    pub fn new(id: usize, type_: String, label: String) -> Self {
        Self {
            id,
            type_,
            label,
            values: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeTypeCatalog {
    #[serde(default)]
    pub format: Option<FormatSpec>,
    pub nodes: NodeTypesSection,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormatSpec {
    /// Root table name for serialized output (e.g. "units").
    pub root: String,

    /// Extra top-level tables to include in the serialized TOML (e.g. [sim]).
    ///
    /// These are emitted verbatim from the template (no graph-derived data).
    #[serde(default)]
    pub tables: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeTypesSection {
    pub types: HashMap<String, NodeTypeDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeTypeDef {
    #[serde(default)]
    pub order: Option<Vec<String>>,
    pub params: HashMap<String, ParamDef>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParamDef {
    #[serde(rename = "type")]
    pub kind: ParamType,
    pub value_type: Option<ValueType>,
    pub len: Option<usize>,

    /// Optional hint to derive this value during serialization.
    #[serde(default)]
    pub source: Option<ParamSource>,

    /// Optional hint controlling rendering (e.g. scalar vs list).
    #[serde(default)]
    pub render: Option<RenderHint>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ParamSource {
    /// Use the node's label.
    NodeLabel,
    /// Use all incoming edge labels.
    IncomingEdgeLabels,
    /// Use all outgoing edge labels.
    OutgoingEdgeLabels,
    /// Use the Nth incoming edge label (default index 0).
    IncomingEdgeLabel {
        index: Option<usize>,
    },
    /// Use the Nth outgoing edge label (default index 0).
    OutgoingEdgeLabel {
        index: Option<usize>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RenderHint {
    /// Render list-of-one as scalar.
    Scalar,
    /// Always render as list (wrap scalars).
    List,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    String,
    Float,
    Bool,
    List,
    Table,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    String,
    Float,
    Bool,
    Any,
}
