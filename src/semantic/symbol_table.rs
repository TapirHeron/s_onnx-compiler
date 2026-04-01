use std::collections::{HashMap, HashSet};
use crate::parser::AST;

/// 符号表 - 用于语义检查，存储张量/节点/初始化器信息
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    // 节点名称 -> 节点AST
    nodes: HashMap<String, AST>,
    // 张量名称 -> 类型+形状
    tensors: HashMap<String, (String, Vec<AST>)>,
    // 初始化器名称 -> 类型+维度
    initializers: HashMap<String, (String, Vec<i64>)>,
    // 输出张量名称(确保唯一)
    output_tensors: HashSet<String>,
}

impl SymbolTable {
    /// 创建新的符号表
    pub fn new() -> Self {
        Self::default()
    }

    /// 插入节点 - 检查重名
    pub fn insert_node(&mut self, name: &str, node: &AST) -> Result<(), String> {
        if self.nodes.contains_key(name) {
            return Err(format!("节点名称重复: {}", name));
        }
        self.nodes.insert(name.to_string(), node.clone());
        Ok(())
    }

    /// 插入输入/输出张量 - 检查重名
    pub fn insert_tensor(&mut self, name: &str, dtype: &str, shape: &[AST]) -> Result<(), String> {
        if self.tensors.contains_key(name) {
            return Err(format!("张量名称重复: {}", name));
        }
        self.tensors.insert(name.to_string(), (dtype.to_string(), shape.to_vec()));
        Ok(())
    }

    /// 插入输出张量(额外检查全局唯一)
    pub fn insert_output_tensor(&mut self, name: &str, dtype: &str, shape: &[AST]) -> Result<(), String> {
        self.insert_tensor(name, dtype, shape)?;
        if self.output_tensors.contains(name) {
            return Err(format!("输出张量名称重复: {}", name));
        }
        self.output_tensors.insert(name.to_string());
        Ok(())
    }

    /// 插入初始化器
    pub fn insert_initializer(&mut self, name: &str, dtype: &str, dims: &[i64]) -> Result<(), String> {
        if self.initializers.contains_key(name) {
            return Err(format!("初始化器名称重复: {}", name));
        }
        self.initializers.insert(name.to_string(), (dtype.to_string(), dims.to_vec()));
        // 初始化器也作为张量
        self.tensors.insert(name.to_string(), (dtype.to_string(), vec![]));
        Ok(())
    }

    /// 检查张量是否已定义(包括初始化器)
    pub fn is_tensor_defined(&self, name: &str) -> bool {
        self.tensors.contains_key(name)
    }

    /// 获取张量类型
    pub fn get_tensor_type(&self, name: &str) -> Option<&str> {
        self.tensors.get(name).map(|(d, _)| d.as_str())
    }

    /// 检查输出张量是否唯一
    pub fn is_output_unique(&self, name: &str) -> bool {
        !self.output_tensors.contains(name)
    }
}