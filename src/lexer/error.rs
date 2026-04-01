use super::super::error::Position;
use thiserror::Error;

/// 词法错误 - 匹配文档错误分类
#[derive(Error, Debug)]
pub enum LexError {
    #[error("文件读取失败: {0}")]
    FileError(String),
    #[error("非法字符: '{0}' @ {1}")]
    InvalidChar(char, Position),
    #[error("无效整数: {0} @ {1}")]
    InvalidInteger(String, Position),
    #[error("未闭合字符串 @ {1}")]
    UnclosedString(String, Position),
    #[error("无效字节数据: {0} @ {1}")]
    InvalidBytes(String, Position),
    #[error("转义字符错误: {0} @ {1}")]
    InvalidEscape(String, Position),
}