use std::fmt;
use itertools::Itertools;

/// 三地址码(TAC)
#[derive(Debug, Clone)]
pub enum TAC {
    /// 输入张量定义: <result> = Input(<name>, <data_type>, <shape>)
    Input {
        result: String,
        name: String,
        data_type: String,
        shape: Vec<String>,
    },

    /// 输出张量定义: Output(<name>, <operand>)
    Output {
        name: String,
        operand: String,
    },

    /// 权重初始化: <result> = Initializer(<name>, <data_type>, <shape>, <raw_data>)
    Initializer {
        result: String,
        name: String,
        data_type: String,
        shape: Vec<i64>,
        raw_data: String,
    },

    /// 操作符执行: <result> = <op_type> <operand1>, <operand2>, ... [attributes]
    Operation {
        result: String,
        op_type: String,
        operands: Vec<String>,
        attributes: Option<Vec<(String, String)>>,
    },

    /// 注释
    Comment(String),
}

impl fmt::Display for TAC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TAC::Input { result, name, data_type, shape } => {
                write!(f, "{} = Input(\"{}\", {}, {:?})", result, name, data_type, shape)
            }
            TAC::Output { name, operand } => {
                write!(f, "Output(\"{}\", {})", name, operand)
            }
            TAC::Initializer { result, name, data_type, shape, raw_data } => {
                write!(f, "{} = Initializer(\"{}\", {}, {:?}, {})", result, name, data_type, shape, raw_data)
            }
            TAC::Operation { result, op_type, operands, attributes } => {
                let ops = operands.join(", ");
                if let Some(attrs) = attributes {
                    let attr_str = attrs.iter().map(|(k, v)| format!("{}={}", k, v)).join(", ");
                    write!(f, "{} = {} {}, [{}]", result, op_type, ops, attr_str)
                } else {
                    write!(f, "{} = {} {}", result, op_type, ops)
                }
            }
            TAC::Comment(desc) => {
                write!(f, "// {}", desc)
            }
        }
    }
}