use thiserror::Error;

/// 语义错误
#[derive(Error, Debug, Clone)]
pub enum SemanticError {
    #[error("命名冲突: {0}")]
    NamingConflict(String),

    #[error("未定义即使用: {0} 未声明")]
    UndefinedReference(String),

    #[error("张量类型不匹配: {0} 预期{1}, 实际{2}")]
    TypeMismatch(String, String, String),

    #[error("操作符输入类型不一致: {0} 的输入{1}和{2}类型分别为{3}和{4}")]
    OpInputTypeMismatch(String, String, String, String, String),

    #[error("模型输入输出类型不匹配: 操作{0}输入为{1}, 输出为{2}")]
    ModelIOTypeMismatch(String, String, String),
}