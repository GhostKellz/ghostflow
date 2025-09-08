use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: NodeCategory,
    pub version: String,
    pub inputs: Vec<NodePort>,
    pub outputs: Vec<NodePort>,
    pub parameters: Vec<NodeParameter>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeCategory {
    Trigger,
    Action,
    Transform,
    ControlFlow,
    Integration,
    Ai,
    Data,
    Utility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePort {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub data_type: DataType,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeParameter {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub param_type: ParameterType,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub options: Option<Vec<ParameterOption>>,
    pub validation: Option<ParameterValidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterOption {
    pub value: serde_json::Value,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Any,
    String,
    Number,
    Boolean,
    Object,
    Array,
    Binary,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Select,
    MultiSelect,
    Object,
    Array,
    Secret,
    File,
    Code,
}

pub use crate::flow::ParameterType as FlowParameterType;