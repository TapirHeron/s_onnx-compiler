use super::{SymbolTable, SemanticError};
use crate::parser::AST;
use std::collections::HashSet;

/// 语义检查器 - 实现文档要求的3类语义检查
pub struct SemanticChecker {
    ast: AST,
    sym_table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl SemanticChecker {
    /// 创建新的语义检查器
    pub fn new(ast: AST) -> Self {
        SemanticChecker {
            ast,
            sym_table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    /// 执行语义检查
    pub fn check(&mut self) -> Result<AST, SemanticError> {
        self.visit_ast(&self.ast.clone());
        if !self.errors.is_empty() {
            return Err(self.errors.remove(0));
        }
        Ok(self.ast.clone())
    }

    /// 递归遍历AST进行检查
    fn visit_ast(&mut self, ast: &AST) {
        match ast {
            AST::ModelProto { graph, .. } => self.visit_ast(graph),
            AST::Graph { name: _, nodes, inputs, outputs, initializers, .. } => {
                // 1. 先插入所有输入张量
                for input in inputs {
                    self.visit_value_info(input, false);
                }
                // 2. 插入所有初始化器
                if let Some(inits) = initializers {
                    for init in inits {
                        self.visit_initializer(init);
                    }
                }
                // 3. 插入所有输出张量
                for output in outputs {
                    self.visit_value_info(output, true);
                }
                // 4. 检查所有节点
                for node in nodes {
                    self.visit_node(node);
                }
            }
            _ => {}
        }
    }

    /// 检查ValueInfo(输入/输出)
    fn visit_value_info(&mut self, vi: &AST, is_output: bool) {
        if let AST::ValueInfo { name, elem_type, shape, pos: _ } = vi {
            let res = if is_output {
                self.sym_table.insert_output_tensor(name, elem_type, shape)
            } else {
                self.sym_table.insert_tensor(name, elem_type, shape)
            };
            if let Err(e) = res {
                self.errors.push(SemanticError::NamingConflict(e));
            }
        }
    }

    /// 检查初始化器
    fn visit_initializer(&mut self, init: &AST) {
        if let AST::Initializer { name, data_type, dims, raw_data: _, .. } = init {
            if let Err(e) = self.sym_table.insert_initializer(name, data_type, dims) {
                self.errors.push(SemanticError::NamingConflict(e));
            }
        }
    }

    /// 检查节点 - 核心语义检查
    fn visit_node(&mut self, node: &AST) {
        if let AST::Node { op_type, name, inputs, outputs, attributes: _, pos: _ } = node {
            // 检查1: 节点名称重名
            if let Err(e) = self.sym_table.insert_node(name, node) {
                self.errors.push(SemanticError::NamingConflict(e));
                return;
            }

            // 检查2: 输入张量是否已定义
            let mut input_types = HashSet::new();
            for input in inputs {
                if !self.sym_table.is_tensor_defined(input) {
                    self.errors.push(SemanticError::UndefinedReference(input.clone()));
                    return;
                }
                // 获取输入类型并收集
                if let Some(t) = self.sym_table.get_tensor_type(input) {
                    input_types.insert(t.to_string());
                }
            }

            // 检查3: 同一操作符输入类型一致
            if input_types.len() > 1 && inputs.len() >= 2 {
                let types = input_types.into_iter().collect::<Vec<_>>();
                self.errors.push(SemanticError::OpInputTypeMismatch(
                    op_type.clone(),
                    inputs[0].clone(),
                    inputs[1].clone(),
                    types[0].clone(),
                    types[1].clone(),
                ));
                return;
            }

            // 检查4: 输出张量是否已定义(且唯一)
            let input_type = self.sym_table.get_tensor_type(&inputs[0]).unwrap_or("unknown");
            for output in outputs {
                if !self.sym_table.is_tensor_defined(output) {
                    self.errors.push(SemanticError::UndefinedReference(output.clone()));
                    return;
                }
                // 检查5: 模型输入输出类型匹配
                let output_type = self.sym_table.get_tensor_type(output).unwrap_or("unknown");
                if input_type != output_type {
                    self.errors.push(SemanticError::ModelIOTypeMismatch(
                        op_type.clone(),
                        input_type.to_string(),
                        output_type.to_string(),
                    ));
                    return;
                }
            }
        }
    }
}