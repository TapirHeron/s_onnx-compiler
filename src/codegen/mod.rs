pub mod tac;
pub mod generator;
pub mod error;
pub use generator::{TAC, CodeGenerator};
pub use error::CodeGenError;