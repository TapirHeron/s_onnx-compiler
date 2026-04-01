#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // 32个关键字
    ModelProto,
    Graph, Name, Node, Input, Output,
    OpType, Attribute,
    Initializer,
    DocString,
    Domain,
    ModelVersion,
    ProducerName,
    ProducerVersion,
    Rtype,
    TensorType,
    IrVersion,
    ElemType,
    Shape,
    Dim,
    Dims,
    RawData,
    OpsetImport,
    DimValue,
    DimParam,
    DataType,
    Version,
    Value, 
    Int, 
    Float, 
    String, 
    Bool,

    // 6个专用符号
    LBracket,  // [
    RBracket,  // ]
    LCurly,    // {
    RCurly,    // }
    Comma,     // ,
    Equal,     // =

    // 字面量
    Ident(String),       // 标识符
    Integer(i64),        // 整数(INTEGER)
    StringLit(String),   // 字符串(STRING)
    BytesLit(String),    // 字节数据(BYTES)

    // 文件结束
    Eof,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // 关键字
            Token::ModelProto => write!(f, "关键字：ModelProto"),
            Token::Graph => write!(f, "关键字：Graph"),
            Token::Name => write!(f, "关键字：Name"),
            Token::Node => write!(f, "关键字：Node"),
            Token::Input => write!(f, "关键字：Input"),
            Token::Output => write!(f, "关键字：Output"),
            Token::OpType => write!(f, "关键字：OpType"),
            Token::Attribute => write!(f, "关键字：Attribute"),
            Token::Initializer => write!(f, "关键字：Initializer"),
            Token::DocString => write!(f, "关键字：DocString"),
            Token::Domain => write!(f, "关键字：Domain"),
            Token::ModelVersion => write!(f, "关键字：ModelVersion"),
            Token::ProducerName => write!(f, "关键字：ProducerName"),
            Token::ProducerVersion => write!(f, "关键字：ProducerVersion"),
            Token::Rtype => write!(f, "关键字：Rtype"),
            Token::TensorType => write!(f, "关键字：TensorType"),
            Token::IrVersion => write!(f, "关键字：IrVersion"),
            Token::ElemType => write!(f, "关键字：ElemType"),
            Token::Shape => write!(f, "关键字：Shape"),
            Token::Dim => write!(f, "关键字：Dim"),
            Token::Dims => write!(f, "关键字：Dims"),
            Token::RawData => write!(f, "关键字：RawData"),
            Token::OpsetImport => write!(f, "关键字：OpsetImport"),
            Token::DimValue => write!(f, "关键字：DimValue"),
            Token::DimParam => write!(f, "关键字：DimParam"),
            Token::DataType => write!(f, "关键字：DataType"),
            Token::Version => write!(f, "关键字：Version"),
            Token::Value => write!(f, "关键字：Value"),
            Token::Int => write!(f, "关键字：Int"),
            Token::Float => write!(f, "关键字：Float"),
            Token::String => write!(f, "关键字：String"),
            Token::Bool => write!(f, "关键字：Bool"),

            // 专用符号
            Token::LBracket => write!(f, "符号：["),
            Token::RBracket => write!(f, "符号：]"),
            Token::LCurly => write!(f, "符号：{{"),
            Token::RCurly => write!(f, "符号：}}"),
            Token::Comma => write!(f, "符号：,"),
            Token::Equal => write!(f, "符号：="),

            // 字面量
            Token::Ident(s) => write!(f, "标识符：{}", s),
            Token::Integer(n) => write!(f, "整数：{}", n),
            Token::StringLit(s) => write!(f, "字符串：\"{}\"", s),
            Token::BytesLit(s) => write!(f, "字节数据：{}", s),

            // 文件结束
            Token::Eof => write!(f, "结束符：Eof"),
        }
    }
}

impl Token {
    /// 判断是否为关键字
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::ModelProto | Token::Graph | Token::Name | Token::Node | Token::Input
            | Token::Output | Token::OpType | Token::Attribute | Token::Initializer
            | Token::DocString | Token::Domain | Token::ModelVersion | Token::ProducerName
            | Token::ProducerVersion | Token::Rtype | Token::TensorType | Token::IrVersion
            | Token::ElemType | Token::Shape | Token::Dim | Token::Dims | Token::RawData
            | Token::OpsetImport | Token::DimValue | Token::DimParam | Token::DataType
            | Token::Version | Token::Value | Token::Int | Token::Float | Token::String | Token::Bool
        )
    }
}

