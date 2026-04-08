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

    /// 格式化打印AST(缩进式)
    pub fn print(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
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
                println!("{}ModelProto {{", indent_str);
                println!("{}  ir_version: {}", indent_str, ir_version);
                println!("{}  producer_name: \"{}\"", indent_str, producer_name);
                println!("{}  producer_version: \"{}\"", indent_str, producer_version);
                println!("{}  domain: \"{}\"", indent_str, domain);
                println!("{}  model_version: \"{}\"", indent_str, model_version);
                println!("{}  doc_string: \"{}\"", indent_str, doc_string);
                graph.print(indent + 2);
                opset_import.print(indent + 2);
                println!("{}}}", indent_str);
            }
            AST::Graph { name, nodes, inputs, outputs, initializers, .. } => {
                println!("{}graph {{", indent_str);
                println!("{}  name: \"{}\"", indent_str, name);
                println!("{}  inputs [{}]:", indent_str, inputs.len());
                inputs.iter().for_each(|i| i.print(indent + 3));
                println!("{}  nodes [{}]:", indent_str, nodes.len());
                nodes.iter().for_each(|n| n.print(indent + 3));
                println!("{}  outputs [{}]:", indent_str, outputs.len());
                outputs.iter().for_each(|o| o.print(indent + 3));
                if let Some(init) = initializers {
                    println!("{}  initializers [{}]:", indent_str, init.len());
                    init.iter().for_each(|i| i.print(indent + 3));
                }
                println!("{}}}", indent_str);
            }
            AST::Node { op_type, name, inputs, outputs, attributes, .. } => {
                println!("{}node {{", indent_str);
                println!("{}  op_type: \"{}\"", indent_str, op_type);
                println!("{}  name: \"{}\"", indent_str, name);
                println!("{}  input: {:?}", indent_str, inputs);
                println!("{}  output: {:?}", indent_str, outputs);
                if let Some(attrs) = attributes {
                    println!("{}  attributes [{}]:", indent_str, attrs.len());
                    attrs.iter().for_each(|a| a.print(indent + 3));
                }
                println!("{}}}", indent_str);
            }
            AST::ValueInfo { name, elem_type, shape, .. } => {
                println!("{}ValueInfo {{", indent_str);
                println!("{}  name: \"{}\"", indent_str, name);
                println!("{}  elem_type: {}", indent_str, elem_type);
                println!("{}  shape:", indent_str);
                shape.iter().for_each(|d| d.print(indent + 3));
                println!("{}}}", indent_str);
            }
            AST::Dim { dim_value, dim_param, .. } => {
                println!("{}dim {{", indent_str);
                if let Some(v) = dim_value {
                    println!("{}  dim_value: {}", indent_str, v);
                }
                if let Some(p) = dim_param {
                    println!("{}  dim_param: \"{}\"", indent_str, p);
                }
                println!("{}}}", indent_str);
            }
            AST::OpsetImport { domain, version, ..} => {
                println!("{}OpsetImport {{", indent_str);
                println!("{}  domain: \"{}\"", indent_str, domain);
                println!("{}  version: {}", indent_str, version);
                println!("{}}}", indent_str);
            }
            _ => println!("{}{:?}", indent_str, self),
        }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}