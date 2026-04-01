use thiserror::Error;

/// 代码生成错误
#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("AST节点类型错误: 预期{0}, 实际{1}")]
    ASTNodeTypeError(String, String),

    #[error("缺少必要的AST节点属性: {0}")]
    MissingASTAttribute(String),

    #[error("生成三地址码失败: {0}")]
    TACGenFailed(String),
}