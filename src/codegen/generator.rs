use crate::{error::CompilerError, parser::AST, TAC};
use crate::codegen::CodeGenError;

// ============================================
// 代码生成器核心类
// ============================================
#[derive(Debug)]
pub struct CodeGenerator {
    ast: AST,
    instructions: Vec<TAC>,
    temp_counter: usize,
}

impl CodeGenerator {
    /// 创建代码生成器
    pub fn new(ast: AST) -> Self {
        Self {
            ast,
            instructions: Vec::new(),
            temp_counter: 1,
        }
    }

    /// 生成三地址码
    pub fn generate(&mut self) -> Result<Vec<TAC>, CompilerError> {
        self.visit_model(&self.ast.clone())?;
        Ok(self.instructions.clone())
    }

    /// 生成临时变量名
    fn gen_temp(&mut self) -> String {
        let name = format!("T{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    // ============================================
    // AST 遍历方法
    // ============================================

    fn visit_model(&mut self, ast: &AST) -> Result<(), CompilerError> {
        if let AST::ModelProto { graph, .. } = ast {
            self.visit_graph(graph)?;
        }
        Ok(())
    }

    fn visit_graph(&mut self, graph: &AST) -> Result<(), CompilerError> {
        match graph {
            AST::Graph { inputs, initializers, nodes, outputs, .. } => {
                // 1. 输入张量定义
                for input in inputs {
                    self.visit_input(input)?;
                }

                // 2. 权重初始化
                if let Some(inits) = initializers {
                    for init in inits {
                        self.visit_initializer(init)?;
                    }
                }

                // 3. 算子执行
                for node in nodes {
                    self.visit_node(node)?;
                }

                // 4. 输出张量定义
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

    // 生成 Input 指令: T0 = Input("X", int, [3, 2])
    fn visit_input(&mut self, input: &AST) -> Result<(), CompilerError> {
        match input {
            AST::ValueInfo { name, elem_type, shape, .. } => {
                let result = self.gen_temp();
                let dim_values: Vec<String> = shape.iter()
                    .filter_map(|dim| {
                        if let AST::Dim { dim_value: Some(v), .. } = dim {
                            Some(v.to_string())
                        } else if let AST::Dim { dim_param: Some(p), .. } = dim {
                            Some(p.clone())
                        } else {
                            None
                        }
                    })
                    .collect();

                self.instructions.push(TAC::Input {
                    result,
                    name: name.clone(),
                    data_type: elem_type.clone(),
                    shape: dim_values,
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "ValueInfo".to_string(),
                format!("{:?}", input),
            ))),
        }
        Ok(())
    }

    // 生成 Initializer 指令: T3 = Initializer("conv.bias", int, [1,2,3,4], raw_data=000000000000b)
    fn visit_initializer(&mut self, init: &AST) -> Result<(), CompilerError> {
        match init {
            AST::Initializer { name, data_type, dims, raw_data, .. } => {
                let result = self.gen_temp();
                let shape = dims.clone();

                self.instructions.push(TAC::Initializer {
                    result,
                    name: name.clone(),
                    data_type: data_type.clone(),
                    shape,
                    raw_data: format!("row_data={}", raw_data),
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "Initializer".to_string(),
                format!("{:?}", init),
            ))),
        }
        Ok(())
    }

    // 生成 Operation 指令: T4 = Pad T0, T1, T2, [mode=33]
    fn visit_node(&mut self, node: &AST) -> Result<(), CompilerError> {
        match node {
            AST::Node { op_type, inputs, outputs, attributes, .. } => {
                // 操作数直接使用输入名称
                let operands = inputs.clone();

                // 提取属性
                let attrs: Option<Vec<(String, String)>> = attributes.as_ref().map(|attr_list| {
                    attr_list.iter().filter_map(|attr| {
                        if let AST::Attribute { name, value, .. } = attr {
                            Some((name.clone(), value.clone()))
                        } else {
                            None
                        }
                    }).collect()
                });

                // 为每个输出生成操作指令
                for _output_name in outputs {
                    let result = self.gen_temp();
                    self.instructions.push(TAC::Operation {
                        result,
                        op_type: op_type.clone(),
                        operands: operands.clone(),
                        attributes: attrs.clone(),
                    });
                }
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "Node".to_string(),
                format!("{:?}", node),
            ))),
        }
        Ok(())
    }

    // 生成 Output 指令: Output("Y", T4)
    fn visit_output(&mut self, output: &AST) -> Result<(), CompilerError> {
        match output {
            AST::ValueInfo { name, .. } => {
                // 查找最后一个 Operation 的结果变量
                let operand = self.find_last_result();
                self.instructions.push(TAC::Output {
                    name: name.clone(),
                    operand,
                });
            }
            _ => return Err(CompilerError::CodeGen(CodeGenError::ASTNodeTypeError(
                "ValueInfo".to_string(),
                format!("{:?}", output),
            ))),
        }
        Ok(())
    }

    /// 查找最后一条 Operation 的结果变量
    fn find_last_result(&self) -> String {
        for inst in self.instructions.iter().rev() {
            if let TAC::Operation { result, .. } = inst {
                return result.clone();
            }
        }
        "unknown".to_string()
    }
}
