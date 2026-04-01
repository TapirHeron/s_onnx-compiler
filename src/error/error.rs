

use thiserror::Error;
use crate::{lexer::LexError, parser::ParseError, semantic::SemanticError, codegen::CodeGenError};

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("文件打开错误: {0}")]
    FileOpen(String),
    #[error("词法错误: {0}")]
    Lex(#[from] LexError),
    #[error("语法错误: {0}")]
    Parse(#[from] ParseError),
    #[error("语义错误: {0}")]
    Semantic(#[from] SemanticError),
    #[error("代码生成错误: {0}")]
    CodeGen(#[from] CodeGenError),
}