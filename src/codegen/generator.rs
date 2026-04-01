use super::{TAC, CodeGenError};
use crate::parser::AST;
use std::collections::HashMap;

/// 三地址码 (TAC) 生成器
/// 后序遍历 AST，严格按照 S-ONNX 文法生成规范中间代码
pub struct CodeGenerator {
    ast: AST,
    tac: Vec<TAC>,
    // 张量名 → 临时变量名 (T1, T2, T3...)
    temp_map: HashMap<String, String>,
    temp_idx: u32,
}

impl CodeGenerator {
    pub fn new(ast: AST) -> Self {
        CodeGenerator {
            ast,
            tac: Vec::new(),
            temp_map: HashMap::new(),
            temp_idx: 1,
        }
    }

    /// 生成 TAC 入口函数
    pub fn generate(&mut self) -> Result<Vec<TAC>, CodeGenError> {
        self.tac.push(TAC::Comment("S-ONNX 模型三地址码开始".to_string()));
        self.traverse(&self.ast.clone())?;
        self.tac.push(TAC::Comment("S-ONNX 模型三地址码结束".to_string()));
        Ok(self.tac.clone())
    }

    /// 后序遍历 AST
    fn traverse(&mut self, node: &AST) -> Result<(), CodeGenError> {
        match node {
            // 根节点
            AST::ModelProto { graph, .. } => {
                self.traverse(graph)?;
            }

            // 图结构：先输入 → 初始化器 → 节点 → 输出
            AST::Graph { name, inputs, outputs, nodes, initializers, .. } => {
                self.tac.push(TAC::Comment(format!("图名称: {}", name)));

                // 1. 输入张量
                for input in inputs {
                    self.traverse(input)?;
                }

                // 2. 初始化器
                if let Some(inits) = initializers {
                    for init in inits {
                        self.traverse(init)?;
                    }
                }

                // 3. 计算节点
                for node in nodes {
                    self.traverse(node)?;
                }

                // 4. 输出张量
                for output in outputs {
                    self.traverse(output)?;
                }
            }

            // 输入节点 → Input TAC
            AST::ValueInfo { name, elem_type, shape, .. } => {
                let temp = self.new_temp();
                self.temp_map.insert(name.clone(), temp.clone());
                let shape_vec = self.convert_shape(shape);

                self.tac.push(TAC::Input {
                    result: temp,
                    name: name.clone(),
                    data_type: elem_type.clone(),
                    shape: shape_vec,
                });
            }

            // 计算节点 → Operation TAC
            AST::Node { op_type, inputs, outputs, attributes, .. } => {
                // 获取输入临时变量
                let mut operands = Vec::new();
                for input_name in inputs {
                    let tmp = self.temp_map.get(input_name)
                        .ok_or_else(|| CodeGenError::ASTNodeTypeError("Operation TAC".into(), input_name.clone()))?;
                    operands.push(tmp.clone());
                }

                // 生成输出临时变量
                let out_temp = self.new_temp();
                for out_name in outputs {
                    self.temp_map.insert(out_name.clone(), out_temp.clone());
                }

                // 转换属性
                let attrs = self.convert_attributes(attributes);

                self.tac.push(TAC::Operation {
                    result: out_temp,
                    op_type: op_type.clone(),
                    operands,
                    attributes: attrs,
                });
            }

            // 初始化器 → Initializer TAC
            AST::Initializer { name, data_type, dims, raw_data, .. } => {
                let temp = self.new_temp();
                self.temp_map.insert(name.clone(), temp.clone());

                self.tac.push(TAC::Initializer {
                    result: temp,
                    name: name.clone(),
                    data_type: data_type.clone(),
                    shape: dims.clone(),
                    raw_data: raw_data.clone(),
                });
            }

            // 忽略不需要生成 TAC 的节点
            AST::Dim { .. }
            | AST::Attribute { .. }
            | AST::OpsetImport { .. } => {}
        }

        Ok(())
    }

    /// 生成新临时变量 T1, T2, T3...
    fn new_temp(&mut self) -> String {
        let t = format!("T{}", self.temp_idx);
        self.temp_idx += 1;
        t
    }

    /// 把 shape 节点转成字符串数组
    fn convert_shape(&self, shape: &[AST]) -> Vec<String> {
        shape.iter()
            .filter_map(|d| {
                if let AST::Dim { dim_value: Some(v), .. } = d {
                    Some(v.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    /// 把属性节点转成键值对
    fn convert_attributes(&self, attrs: &Option<Vec<AST>>) -> Option<Vec<(String, String)>> {
        attrs.as_ref().map(|list| {
            list.iter()
                .filter_map(|a| {
                    if let AST::Attribute { name, value, .. } = a {
                        Some((name.clone(), value.clone()))
                    } else {
                        None
                    }
                })
                .collect()
        })
    }
}