#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub file: &'static str,
}

impl Position {
    /// 创建新的位置信息
    pub fn new(line: usize, col: usize) -> Self {
        Position {
            line,
            col,
            file: "unknown",
        }
    }

    /// 设置文件名称
    pub fn with_file(mut self, file: &'static str) -> Self {
        self.file = file;
        self
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "文件:{} 行:{} 列:{}", self.file, self.line, self.col)
    }
}