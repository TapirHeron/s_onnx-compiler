use crate::error::Position;
use std::fmt;

/// 抽象语法树(AST)
#[derive(Debug, Clone)]
pub enum AST {
    /// ModelProto -> "ModelProto" "{" model_body_def "}"
    ModelProto {
        ir_version: i64,
        producer_name: String,
        producer_version: String,
        domain: String,
        model_version: i64,
        doc_string: String,
        graph: Box<AST>,
        opset_import: Box<AST>,
        pos: Position,
    },

    /// graph -> "graph" "{" graph_body_def "}"
    Graph {
        name: String,
        nodes: Vec<AST>,
        inputs: Vec<AST>,
        outputs: Vec<AST>,
        initializers: Option<Vec<AST>>, // 可选
        pos: Position,
    },

    /// node -> "node" "{" node_def "}"
    Node {
        op_type: String,
        name: String,
        inputs: Vec<String>,    // input_arr
        outputs: Vec<String>,   // output_arr
        attributes: Option<Vec<AST>>, // 可选
        pos: Position,
    },

    /// attribute -> "attribute" "{" attribute_def "}"
    Attribute {
        name: String,
        value: String,
        pos: Position,
    },

    /// input/output -> "input"/"output" "{" value_info_def "}"
    ValueInfo {
        name: String,
        elem_type: String,
        shape: Vec<AST>,
        pos: Position,
    },

    /// initializer -> "initializer" "{" tensor_def "}"
    Initializer {
        name: String,
        data_type: String,
        dims: Vec<i64>,
        raw_data: String,
        pos: Position,
    },

    /// dim -> "dim" "{" dim_def "}"
    Dim {
        dim_value: Option<i64>,    // 二选一
        dim_param: Option<String>, // 二选一
        pos: Position,
    },

    /// opset_import -> "opset_import" "{" domain_def version_def "}"
    OpsetImport {
        domain: String,
        version: i64,
        pos: Position,
    },
}

impl AST {
    /// 获取节点位置
    pub fn pos(&self) -> &Position {
        match self {
            AST::ModelProto { pos, .. } => pos,
            AST::Graph { pos, .. } => pos,
            AST::Node { pos, .. } => pos,
            AST::Attribute { pos, .. } => pos,
            AST::ValueInfo { pos, .. } => pos,
            AST::Initializer { pos, .. } => pos,
            AST::Dim { pos, .. } => pos,
            AST::OpsetImport { pos, .. } => pos,
        }
    }

    /// 格式化打印AST(||--结合式)
    pub fn print(&self, indent: usize) {
        let mut output = String::new();
        self.print_to_string(&mut output, indent);
        print!("{}", output);
    }

    /// 将AST输出到字符串
    pub fn print_to_string(&self, output: &mut String, indent: usize) {
        let indent_str = "|  ".repeat(indent);
        match self {
            AST::ModelProto {
                ir_version,
                producer_name,
                producer_version,
                domain,
                model_version,
                doc_string,
                graph,
                opset_import,
            ..} => {
                output.push_str(&format!("{}|-- ModelProto\n", indent_str));
                output.push_str(&format!("{}|-- ir_version: {}\n", indent_str, ir_version));
                output.push_str(&format!("{}|-- producer_name: \"{}\"\n", indent_str, producer_name));
                output.push_str(&format!("{}|-- producer_version: \"{}\"\n", indent_str, producer_version));
                output.push_str(&format!("{}|-- domain: \"{}\"\n", indent_str, domain));
                output.push_str(&format!("{}|-- model_version: {}\n", indent_str, model_version));
                output.push_str(&format!("{}|-- doc_string: \"{}\"\n", indent_str, doc_string));
                graph.print_to_string(output, indent + 1);
                opset_import.print_to_string(output, indent + 1);
            }
            AST::Graph { name, nodes, inputs, outputs, initializers, .. } => {
                output.push_str(&format!("{}|-- graph\n", indent_str));
                output.push_str(&format!("{}|-- name: \"{}\"\n", indent_str, name));
                output.push_str(&format!("{}|-- inputs [{}]:\n", indent_str, inputs.len()));
                inputs.iter().for_each(|i| i.print_to_string(output, indent + 1));
                output.push_str(&format!("{}|-- nodes [{}]:\n", indent_str, nodes.len()));
                nodes.iter().for_each(|n| n.print_to_string(output, indent + 1));
                output.push_str(&format!("{}|-- outputs [{}]:\n", indent_str, outputs.len()));
                outputs.iter().for_each(|o| o.print_to_string(output, indent + 1));
                if let Some(init) = initializers {
                    output.push_str(&format!("{}|-- initializers [{}]:\n", indent_str, init.len()));
                    init.iter().for_each(|i| i.print_to_string(output, indent + 1));
                }
            }
            AST::Node { op_type, name, inputs, outputs, attributes, .. } => {
                output.push_str(&format!("{}|-- node\n", indent_str));
                output.push_str(&format!("{}|-- op_type: \"{}\"\n", indent_str, op_type));
                output.push_str(&format!("{}|-- name: \"{}\"\n", indent_str, name));
                output.push_str(&format!("{}|-- input: {:?}\n", indent_str, inputs));
                output.push_str(&format!("{}|-- output: {:?}\n", indent_str, outputs));
                if let Some(attrs) = attributes {
                    output.push_str(&format!("{}|-- attributes [{}]:\n", indent_str, attrs.len()));
                    attrs.iter().for_each(|a| a.print_to_string(output, indent + 1));
                }
            }
            AST::ValueInfo { name, elem_type, shape, .. } => {
                output.push_str(&format!("{}|-- ValueInfo\n", indent_str));
                output.push_str(&format!("{}|-- name: \"{}\"\n", indent_str, name));
                output.push_str(&format!("{}|-- elem_type: {}\n", indent_str, elem_type));
                output.push_str(&format!("{}|-- shape:\n", indent_str));
                shape.iter().for_each(|d| d.print_to_string(output, indent + 1));
            }
            AST::Dim { dim_value, dim_param, .. } => {
                output.push_str(&format!("{}|-- dim\n", indent_str));
                if let Some(v) = dim_value {
                    output.push_str(&format!("{}|-- dim_value: {}\n", indent_str, v));
                }
                if let Some(p) = dim_param {
                    output.push_str(&format!("{}|-- dim_param: \"{}\"\n", indent_str, p));
                }
            }
            AST::OpsetImport { domain, version, ..} => {
                output.push_str(&format!("{}|-- OpsetImport\n", indent_str));
                output.push_str(&format!("{}|-- domain: \"{}\"\n", indent_str, domain));
                output.push_str(&format!("{}|-- version: {}\n", indent_str, version));
            }
            _ => output.push_str(&format!("{}|-- {:?}\n", indent_str, self)),
        }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}