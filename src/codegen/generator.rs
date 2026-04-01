use super::{TAC, CodeGenError};
use crate::parser::AST;
use std::collections::HashMap;

/// 三地址码生成器
pub struct CodeGenerator {
    ast: AST,
    tac: Vec<TAC>,
    tensor_temp: HashMap<String, String>,
    temp_counter: u32,
}

impl CodeGenerator {
    pub fn new(ast: AST) -> Self {
        Self {
            ast,
            tac: Vec::new(),
            tensor_temp: HashMap::new(),
            temp_counter: 1,
        }
    }

    /// 生成 TAC 入口
    pub fn generate(&mut self) -> Result<Vec<TAC>, CodeGenError> {
        self.tac.push(TAC::Comment("BEGIN S-ONNX TAC".into()));
        self.walk(&self.ast)?;
        self.tac.push(TAC::Comment("END S-ONNX TAC".into()));
        Ok(self.tac.clone())
    }

    /// 后序遍历 AST
    fn walk(&mut self, node: &AST) -> Result<(), CodeGenError> {
        match node {
            AST::ModelProto { graph, .. } => {
                self.walk(graph)?;
            }

            AST::Graph { name, inputs, outputs, nodes, initializers, .. } => {
                self.tac.push(TAC::Comment(format!("Graph: {}", name)));

                // 输入张量
                for input in inputs {
                    self.walk(input)?;
                }

                // 初始化器
                if let Some(inits) = initializers {
                    for init in inits {
                        self.walk(init)?;
                    }
                }

                // 计算节点
                for node in nodes {
                    self.walk(node)?;
                }

                // 输出张量
                for output in outputs {
                    self.walk(output)?;
                }
            }

            // 输入 → TAC Input
            AST::ValueInfo { name, elem_type, shape, .. } => {
                let temp = self.new_temp();
                self.tensor_temp.insert(name.clone(), temp.clone());
                let shape_str = self.shape_to_strs(shape);
                self.tac.push(TAC::Input {
                    result: temp,
                    name: name.clone(),
                    data_type: elem_type.clone(),
                    shape: shape_str,
                });
            }

            // 节点 → TAC Operation
            AST::Node { op_type, inputs, outputs, attributes, .. } => {
                let mut ops = Vec::new();
                for inp in inputs {
                    let t = self.tensor_temp.get(inp)
                        .ok_or_else(|| CodeGenError::MissingASTAttribute(format!("input {} not found", inp)))?;
                    ops.push(t.clone());
                }

                let out_temp = self.new_temp();
                for out in outputs {
                    self.tensor_temp.insert(out.clone(), out_temp.clone());
                }

                let attrs = self.convert_attrs(attributes);
                self.tac.push(TAC::Operation {
                    result: out_temp,
                    op_type: op_type.clone(),
                    operands: ops,
                    attributes: attrs,
                });
            }

            // 初始化器 → TAC Initializer
            AST::Initializer { name, data_type, dims, raw_data, .. } => {
                let temp = self.new_temp();
                self.tensor_temp.insert(name.clone(), temp.clone());
                self.tac.push(TAC::Initializer {
                    result: temp,
                    name: name.clone(),
                    data_type: data_type.clone(),
                    shape: dims.clone(),
                    raw_data: raw_data.clone(),
                });
            }

            // 输出 → TAC Output
            AST::Output { name, .. } => {
                let t = self.tensor_temp.get(name)
                    .ok_or_else(|| CodeGenError::MissingASTAttribute(format!("output {} not found", name)))?;
                self.tac.push(TAC::Output {
                    name: name.clone(),
                    operand: t.clone(),
                });
            }

            _ => {}
        }
        Ok(())
    }

    /// 生成新临时变量 T1, T2, T3...
    fn new_temp(&mut self) -> String {
        let t = format!("T{}", self.temp_counter);
        self.temp_counter += 1;
        t
    }

    /// shape 转字符串列表
    fn shape_to_strs(&self, shape: &[AST]) -> Vec<String> {
        shape.iter().filter_map(|d| {
            if let AST::Dim { dim_value: Some(v), .. } = d {
                Some(v.to_string())
            } else {
                None
            }
        }).collect()
    }

    /// 转换 attribute
    fn convert_attrs(&self, attrs: &Option<Vec<AST>>) -> Option<Vec<(String, String)>> {
        attrs.as_ref().map(|list| {
            list.iter().filter_map(|a| {
                if let AST::Attribute { name, value, .. } = a {
                    Some((name.clone(), value.clone()))
                } else {
                    None
                }
            }).collect()
        })
    }
}