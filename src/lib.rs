//! S-ONNX模型编译器
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod codegen;
pub mod error;
pub mod utils;

// 全局导出核心类型
pub use lexer::Token;
pub use parser::AST;
pub use semantic::SymbolTable;
pub use codegen::TAC;
pub use error::{CompilerError, Position};