use crate::{
    error::{CompilerError, Position},
    parser::AST,
};
use std::fmt;
use crate::codegen::CodeGenError;

// ============================================
// 🔥 TAC 定义：完全匹配你测试用的 API！
// ============================================
#[derive(Debug, Clone, PartialEq)]
pub enum TAC {
    Input {
        name: String,
        ty: String,
        shape: Vec<usize>,
        pos: Position,
    },
    Initializer {
        name: String,
        value: Vec<f64>,
        shape: Vec<usize>,
        pos: Position,
    },
    Operation {
        op: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        pos: Position,
    },
    Output {
        name: String,
        pos: Position,
    },
}

// ============================================
// 代码生成器核心类
// ============================================
#[derive(Debug)]
pub struct CodeGenerator {
    ast: AST,
    instructions: Vec<TAC>,
}

impl CodeGenerator {
    /// 完全匹配你的调用：CodeGenerator::new(checked_ast)
    pub fn new(ast: AST) -> Self {
        Self {
            ast,
            instructions: Vec::new(),
        }
    }

    /// 完全匹配你的调用：codegen.generate() -> Result<Vec<TAC>, CompilerError>
    pub fn generate(&mut self) -> Result<Vec<TAC>, CompilerError> {
        self.visit_model(&self.ast.clone())?;
        Ok(self.instructions.clone())
    }

    // 遍历 ModelProto
    fn visit_model(&mut self, ast: &AST) -> Result<(), CompilerError> {
        if let AST::ModelProto { graph, .. } = ast {
            self.visit_graph(graph)?;
        }
        Ok(())
    }

    // 遍历 Graph → 生成 Input / Initializer / Node / Output
    fn visit_graph(&mut self, graph: &AST) -> Result<(), CompilerError> {
        match graph {
            AST::Graph { inputs, initializers, nodes, outputs, .. } => {
                // 1. 生成 Input TAC
                for input in inputs {
                    self.visit_input(input)?;
                }

                // 2. 生成 Initializer TAC
                if let Some(inits) = initializers {
                    for init in inits {
                        self.visit_initializer(init)?;
                    }
                }

                // 3. 生成 Operation TAC（算子）
                for node in nodes {
                    self.visit_node(node)?;
                }

                // 4. 生成 Output TAC
                for output in outputs {
                    self.visit_output(output)?;
                }
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "Graph".to_string(),
                format!("{:?}", graph),
            ))),
        }
        Ok(())
    }

    // 生成 Input 指令
    fn visit_input(&mut self, input: &AST) -> Result<(), CompilerError> {
        match input {
            AST::ValueInfo { name, elem_type, shape, pos } => {
                // 将 Vec<AST> (Dim) 转换为 Vec<usize>
                let dim_values: Vec<usize> = shape.iter()
                    .filter_map(|dim| {
                        if let AST::Dim { dim_value: Some(v), .. } = dim {
                            Some(*v as usize)
                        } else {
                            None
                        }
                    })
                    .collect();
                
                self.instructions.push(TAC::Input {
                    name: name.clone(),
                    ty: elem_type.clone(),
                    shape: dim_values,
                    pos: pos.clone(),
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "ValueInfo".to_string(),
                format!("{:?}", input),
            ))),
        }
        Ok(())
    }

    // 生成 Initializer 指令
    fn visit_initializer(&mut self, init: &AST) -> Result<(), CompilerError> {
        match init {
            AST::Initializer { name, dims, raw_data, pos, .. } => {
                // 解析 raw_data 字符串为 f64 向量
                let data: Vec<f64> = raw_data
                    .split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .collect();
                
                let shape: Vec<usize> = dims.iter().map(|d| *d as usize).collect();
                
                self.instructions.push(TAC::Initializer {
                    name: name.clone(),
                    value: data,
                    shape,
                    pos: pos.clone(),
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "Initializer".to_string(),
                format!("{:?}", init),
            ))),
        }
        Ok(())
    }

    // 生成 Operation 指令
    fn visit_node(&mut self, node: &AST) -> Result<(), CompilerError> {
        match node {
            AST::Node { op_type, inputs, outputs, pos, .. } => {
                self.instructions.push(TAC::Operation {
                    op: op_type.clone(),
                    inputs: inputs.clone(),
                    outputs: outputs.clone(),
                    pos: pos.clone(),
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "Node".to_string(),
                format!("{:?}", node),
            ))),
        }
        Ok(())
    }

    // 生成 Output 指令
    fn visit_output(&mut self, output: &AST) -> Result<(), CompilerError> {
        match output {
            AST::ValueInfo { name, pos, .. } => {
                self.instructions.push(TAC::Output {
                    name: name.clone(),
                    pos: pos.clone(),
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "ValueInfo".to_string(),
                format!("{:?}", output),
            ))),
        }
        Ok(())
    }
}

// 可选：方便调试打印
impl fmt::Display for TAC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TAC::Input { name, shape, .. } => write!(f, "INPUT {} {:?}", name, shape),
            TAC::Initializer { name, shape, .. } => write!(f, "INIT {} {:?}", name, shape),
            TAC::Operation { op, inputs, outputs, .. } => {
                write!(f, "OP {} <- {:?} -> {:?}", op, inputs, outputs)
            }
            TAC::Output { name, .. } => write!(f, "OUTPUT {}", name),
        }
    }
}