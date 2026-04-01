use super::{Token, LexError};
use crate::error::Position;
use std::fs::read_to_string;
use std::char;

/// 词法扫描器 - 基于DFA实现
pub struct Scanner {
    source: Vec<char>,
    pos: usize,
    pub(crate) line: usize,
    pub(crate) col: usize,
    file: &'static str,
}

impl Scanner {
    /// 从文件创建扫描器
    pub fn new_from_file(path: &str) -> Result<Self, LexError> {
        let content = read_to_string(path)
            .map_err(|e| LexError::FileError(format!("{}: {}", path, e)))?;
        let static_path = Box::leak(String::from(path).into_boxed_str());
        Ok(Self::new(&content, static_path))
    }

    /// 从字符串创建扫描器
    pub fn new(source: &str, file: &'static str) -> Self {
        Scanner {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            file,
        }
    }

    /// 获取下一个Token - 实现getToken()接口
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();
        if self.is_eof() {
            return Ok(Token::Eof);
        }

        let c = self.peek();
        match c {
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier_or_keyword(),
            '0'..='9' => self.read_integer_or_bytes(),
            '"' => self.read_string(),
            '[' => self.consume_and_return(Token::LBracket),
            ']' => self.consume_and_return(Token::RBracket),
            '{' => self.consume_and_return(Token::LCurly),
            '}' => self.consume_and_return(Token::RCurly),
            ',' => self.consume_and_return(Token::Comma),
            '=' => self.consume_and_return(Token::Equal),
            _ => Err(LexError::InvalidChar(c, self.pos())),
        }
    }

    // 工具方法
    fn peek(&self) -> char {
        self.source[self.pos]
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn consume(&mut self) {
        if self.peek() == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.pos += 1;
    }

    fn consume_n(&mut self, n: usize) {
        for _ in 0..n {
            self.consume();
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    pub(crate) fn pos(&self) -> Position {
        Position::new(self.line, self.col).with_file(self.file)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() {
            self.consume();
        }
    }

    fn consume_and_return(&mut self, token: Token) -> Result<Token, LexError> {
        self.consume();
        Ok(token)
    }

    /// 读取标识符或关键字 - 关键字不区分大小写
    fn read_identifier_or_keyword(&mut self) -> Result<Token, LexError> {
        let start = self.pos;
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_' || self.peek() == '-') {
            self.consume();
        }
        let s: String = self.source[start..self.pos].iter().collect();
        let s_lower = s.to_lowercase();

        // 匹配32个关键字，不区分大小写
        Ok(match s_lower.as_str() {
            "modelproto" => Token::ModelProto,
            "graph" => Token::Graph,
            "name" => Token::Name,
            "node" => Token::Node,
            "input" => Token::Input,
            "output" => Token::Output,
            "op_type" => Token::OpType,
            "attribute" => Token::Attribute,
            "initializer" => Token::Initializer,
            "doc_string" => Token::DocString,
            "domain" => Token::Domain,
            "model_version" => Token::ModelVersion,
            "producer_name" => Token::ProducerName,
            "producer_version" => Token::ProducerVersion,
            "type" => Token::Rtype, // 避免与Rust关键字冲突
            "tensor_type" => Token::TensorType,
            "ir_version" => Token::IrVersion,
            "elem_type" => Token::ElemType,
            "shape" => Token::Shape,
            "dim" => Token::Dim,
            "dims" => Token::Dims,
            "raw_data" => Token::RawData,
            "opset_import" => Token::OpsetImport,
            "dim_value" => Token::DimValue,
            "dim_param" => Token::DimParam,
            "data_type" => Token::DataType,
            "version" => Token::Version,
            "value" => Token::Value,
            "int" => Token::Int,
            "float" => Token::Float,
            "string" => Token::String,
            "bool" => Token::Bool,
            _ => Token::Ident(s),
        })
    }

    /// 读取整数或字节数据 (0 | [1-9][0-9]*) [lL]?
    fn read_integer_or_bytes(&mut self) -> Result<Token, LexError> {
        let start = self.pos;
        let first = self.peek();
        let start_pos = self.pos();

        // 处理 0 开头的情况
        if first == '0' {
            self.consume();

            // 检查是否有 L/l 后缀
            if !self.is_eof() && matches!(self.peek(), 'l' | 'L') {
                self.consume();
                return Ok(Token::Integer(0));
            }

            // 如果 0 后面还跟着数字，尝试匹配字节数据（如 01b, 02b）
            if !self.is_eof() && self.peek().is_ascii_digit() {
                return self.read_bytes(start);
            }

            // 单独的 0 是合法的
            return Ok(Token::Integer(0));
        }

        // 非 0 开头，必须是 1-9 后跟任意数字
        while !self.is_eof() && self.peek().is_ascii_digit() {
            self.consume();
        }

        // 处理可选的 L/l 后缀
        if !self.is_eof() && matches!(self.peek(), 'l' | 'L') {
            self.consume();
        }

        let s: String = self.source[start..self.pos].iter().collect();
        let num = s.parse()
            .map_err(|_| LexError::InvalidInteger(s.clone(), start_pos))?;
        Ok(Token::Integer(num))
    }
    /// 读取字符串 - 遵循文档正则: "(ESCAPE_SEQUENCE | (~\\|~"))*"
    fn read_string(&mut self) -> Result<Token, LexError> {
        self.consume(); // 跳过开头的"
        let mut content = String::new();

        while !self.is_eof() && self.peek() != '"' {
            let c = self.peek();
            if c == '\\' {
                // 处理转义字符
                self.consume();
                if self.is_eof() {
                    return Err(LexError::UnclosedString(content, self.pos()));
                }
                let esc = self.peek();
                let decoded = match esc {
                    'b' => '\x08',
                    't' => '\t',
                    'n' => '\n',
                    'f' => '\x0c',
                    'r' => '\r',
                    '"' => '"',
                    '\'' => '\'',
                    '\\' => '\\',
                    _ => return Err(LexError::InvalidEscape(format!("\\{}", esc), self.pos())),
                };
                content.push(decoded);
                self.consume();
            } else {
                content.push(c);
                self.consume();
            }
        }

        if self.is_eof() {
            return Err(LexError::UnclosedString(content, self.pos()));
        }

        self.consume(); // 跳过结尾的"
        Ok(Token::StringLit(content))
    }

    /// 读取字节数据 [0-9A-Fa-f]+b
    fn read_bytes(&mut self, start: usize) -> Result<Token, LexError> {
        while !self.is_eof() && self.peek().is_ascii_hexdigit() {
            if self.peek() == 'b' {
                break;
            }
            self.consume();
        }

        if self.is_eof() || self.peek() != 'b' {
            let s: String = self.source[start..self.pos].iter().collect();
            return Err(LexError::InvalidBytes(s, self.pos()));
        }

        self.consume(); // 跳过b
        let s: String = self.source[start..self.pos].iter().collect();
        Ok(Token::BytesLit(s))
    }
}