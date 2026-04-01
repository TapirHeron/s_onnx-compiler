pub mod symbol_table;
pub mod checker;
pub mod error;
pub use symbol_table::SymbolTable;
pub use checker::SemanticChecker;
pub use error::SemanticError;