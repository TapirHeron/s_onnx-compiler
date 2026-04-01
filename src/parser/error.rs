use crate::{error::Position, lexer::Token};
use thiserror::Error;

/// 语法错误
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("预期Token: {0:?}, 实际得到: {1:?} @ {2}")]
    ExpectedToken(Token, Token, Position),

    #[error("预期整数字面量, 实际得到: {0:?} @ {1}")]
    ExpectedInteger(Token, Position),

    #[error("预期字符串字面量, 实际得到: {0:?} @ {1}")]
    ExpectedString(Token, Position),

    #[error("预期字节数据字面量, 实际得到: {0:?} @ {1}")]
    ExpectedBytes(Token, Position),

    #[error("预期标识符, 实际得到: {0:?} @ {1}")]
    ExpectedIdent(Token, Position),

    #[error("预期数据类型(int/float/string/bool), 实际得到: {0:?} @ {1}")]
    ExpectedDataType(Token, Position),

    #[error("语法结构不完整: 缺少{0} @ {1}")]
    MissingSymbol(Token, Position),

    #[error("意外的Token: {0:?} @ {1}")]
    UnexpectedToken(Token, Position),

    #[error("文法规则不匹配: {0} @ {1}")]
    GrammarMismatch(&'static str, Position),
}