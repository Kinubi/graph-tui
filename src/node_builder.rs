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
    pub nodes: NodeTypesSection,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeTypesSection {
    pub types: HashMap<String, NodeTypeDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeTypeDef {
    pub params: HashMap<String, ParamDef>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParamDef {
    #[serde(rename = "type")]
    pub kind: ParamType,
    pub value_type: Option<ValueType>,
    pub len: Option<usize>,
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
