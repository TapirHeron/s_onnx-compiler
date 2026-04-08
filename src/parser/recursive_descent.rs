use super::{AST, ParseError};
use crate::lexer::{Scanner, Token};
use crate::error::Position;

/// 递归下降解析器
pub struct Parser {
    scanner: Scanner,
    current: Token,
}

impl Parser {
    /// 创建新的解析器
    pub fn new(mut scanner: Scanner) -> Self {
        let current = scanner.next_token().unwrap_or(Token::Eof);
        Parser { scanner, current }
    }

    /// 核心解析方法 - 入口：model -> ModelProto { ... }
    pub fn parse(&mut self) -> Result<AST, ParseError> {
        let ast = self.parse_model_proto()?;
        // 解析完成后必须是 Eof
        if self.current != Token::Eof {
            return Err(ParseError::UnexpectedToken(self.current.clone(), self.pos()));
        }
        Ok(ast)
    }

    // 工具方法
    fn next(&mut self) {
        self.current = self.scanner.next_token().unwrap_or(Token::Eof);
    }

    fn pos(&self) -> Position {
        match &self.current {
            Token::Eof => Position::new(self.scanner.line, self.scanner.col),
            _ => self.scanner.pos(),
        }
    }

    /// 预期指定 Token，匹配则消费，否则报错
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current == expected {
            self.next();
            Ok(())
        } else {
            Err(ParseError::ExpectedToken(expected, self.current.clone(), self.pos()))
        }
    }

    /// 预期整数字面量
    fn expect_integer(&mut self) -> Result<i64, ParseError> {
        match &self.current {
            Token::Integer(n) => {
                let val = *n;
                self.next();
                Ok(val)
            }
            _ => Err(ParseError::ExpectedInteger(self.current.clone(), self.pos())),
        }
    }

    /// 预期字符串字面量
    fn expect_string(&mut self) -> Result<String, ParseError> {
        match &self.current {
            Token::StringLit(s) => {
                let val = s.clone();
                self.next();
                Ok(val)
            }
            _ => Err(ParseError::ExpectedString(self.current.clone(), self.pos())),
        }
    }

    /// 预期字节数据字面量
    fn expect_bytes(&mut self) -> Result<String, ParseError> {
        match &self.current {
            Token::BytesLit(s) => {
                let val = s.clone();
                self.next();
                Ok(val)
            }
            _ => Err(ParseError::ExpectedBytes(self.current.clone(), self.pos())),
        }
    }

    /// 预期标识符
    // fn expect_ident(&mut self) -> Result<String, ParseError> {
    //     match &self.current {
    //         Token::Ident(s) => {
    //             let val = s.clone();
    //             self.next();
    //             Ok(val)
    //         }
    //         _ => Err(ParseError::ExpectedIdent(self.current.clone(), self.pos())),
    //     }
    // }

    /// 预期数据类型 (int/float/string/bool)
    fn expect_data_type(&mut self) -> Result<String, ParseError> {
        let dtype = match &self.current {
            Token::Int => "int".to_string(),
            Token::Float => "float".to_string(),
            Token::String => "string".to_string(),
            Token::Bool => "bool".to_string(),
            _ => return Err(ParseError::ExpectedDataType(self.current.clone(), self.pos())),
        };
        self.next();
        Ok(dtype)
    }

    // 文法规则实现

    /// model -> "ModelProto" "{" model_body_def "}"
    fn parse_model_proto(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::ModelProto)?;
        self.expect(Token::LCurly)?;

        // model_body_def -> ir_version_def producer_name_def producer_version_def domain_def model_version_def doc_string_def graph_def opset_import_def
        let ir_version = self.parse_ir_version_def()?;
        let producer_name = self.parse_producer_name_def()?;
        let producer_version = self.parse_producer_version_def()?;
        let domain = self.parse_domain_def()?;
        let model_version = self.parse_model_version_def()?;
        let doc_string = self.parse_doc_string_def()?;
        let graph = self.parse_graph_def()?;
        let opset_import = self.parse_opset_import_def()?;

        self.expect(Token::RCurly)?;

        Ok(AST::ModelProto {
            ir_version,
            producer_name,
            producer_version,
            domain,
            model_version,
            doc_string,
            graph: Box::new(graph),
            opset_import: Box::new(opset_import),
            pos: self.pos(),
        })
    }

    /// ir_version_def -> "ir_version" "=" INTEGER
    fn parse_ir_version_def(&mut self) -> Result<i64, ParseError> {
        self.expect(Token::IrVersion)?;
        self.expect(Token::Equal)?;
        self.expect_integer()
    }

    /// producer_name_def -> "producer_name" "=" STRING
    fn parse_producer_name_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::ProducerName)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// producer_version_def -> "producer_version" "=" STRING
    fn parse_producer_version_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::ProducerVersion)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// domain_def -> "domain" "=" STRING
    fn parse_domain_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::Domain)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// model_version_def -> "model_version" "=" INTEGER
    fn parse_model_version_def(&mut self) -> Result<i64, ParseError> {
        self.expect(Token::ModelVersion)?;
        self.expect(Token::Equal)?;
        self.expect_integer()
    }

    /// doc_string_def -> "doc_string" "=" STRING
    fn parse_doc_string_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::DocString)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// graph_def -> "graph" "{" graph_body_def "}"
    fn parse_graph_def(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Graph)?;
        self.expect(Token::LCurly)?;

        // graph_body_def -> name_def node_list input_list output_list [initializer_list]
        let name = self.parse_name_def()?;
        let nodes = self.parse_node_list()?;
        let inputs = self.parse_input_list()?;
        let outputs = self.parse_output_list()?;
        let initializers = if self.current == Token::Initializer {
            Some(self.parse_initializer_list()?)
        } else {
            None
        };

        let r_curly_result = self.expect(Token::RCurly);
        if r_curly_result.is_err() {
            return Err(ParseError::MissingSymbol(Token::RCurly, self.pos()));
        }
        Ok(AST::Graph {
            name,
            nodes,
            inputs,
            outputs,
            initializers,
            pos: self.pos(),
        })
    }

    /// name_def -> "name" "=" STRING
    fn parse_name_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::Name)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// node_list -> node_repeats {node_repeats}
    fn parse_node_list(&mut self) -> Result<Vec<AST>, ParseError> {
        let mut nodes = Vec::new();
        while self.current == Token::Node && self.current != Token::Eof {
            nodes.push(self.parse_node_repeats()?);
        }
        Ok(nodes)
    }

    /// node_repeats -> "node" "{" node_def "}"
    fn parse_node_repeats(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Node)?;
        self.expect(Token::LCurly)?;
        let node = self.parse_node_def()?;
        self.expect(Token::RCurly)?;
        Ok(node)
    }

    /// node_def -> op_type_def name_def (input_list | input_arr) (output_list | output_arr) [attribute_list]
    fn parse_node_def(&mut self) -> Result<AST, ParseError> {
        let op_type = self.parse_op_type_def()?;
        let name = self.parse_name_def()?;
        // 优先匹配 input_arr (input = [..])
        let inputs = if self.current == Token::Input {
            self.parse_input_arr()?
        } else {
            Vec::new()
        };
        // 优先匹配 output_arr (output = [..])
        let outputs = if self.current == Token::Output {
            self.parse_output_arr()?
        } else {
            Vec::new()
        };
        // 可选的 attribute_list
        let attributes = if self.current == Token::Attribute {
            Some(self.parse_attribute_list()?)
        } else {
            None
        };

        Ok(AST::Node {
            op_type,
            name,
            inputs,
            outputs,
            attributes,
            pos: self.pos(),
        })
    }

    /// op_type_def -> "op_type" "=" STRING
    fn parse_op_type_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::OpType)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// input_arr -> "input" "=" "[" STRING { "," STRING } "]"
    fn parse_input_arr(&mut self) -> Result<Vec<String>, ParseError> {
        self.expect(Token::Input)?;
        self.expect(Token::Equal)?;
        self.expect(Token::LBracket)?;

        let mut inputs = Vec::new();
        if self.current != Token::RBracket {
            inputs.push(self.expect_string()?);
            while self.current == Token::Comma {
                self.next();
                inputs.push(self.expect_string()?);
            }
        }

        self.expect(Token::RBracket)?;
        Ok(inputs)
    }

    /// output_arr -> "output" "=" "[" STRING { "," STRING } "]"
    fn parse_output_arr(&mut self) -> Result<Vec<String>, ParseError> {
        self.expect(Token::Output)?;
        self.expect(Token::Equal)?;
        self.expect(Token::LBracket)?;

        let mut outputs = Vec::new();
        if self.current != Token::RBracket {
            outputs.push(self.expect_string()?);
            while self.current == Token::Comma {
                self.next();
                outputs.push(self.expect_string()?);
            }
        }

        self.expect(Token::RBracket)?;
        Ok(outputs)
    }

    /// attribute_list -> attribute_repeats {attribute_repeats}
    fn parse_attribute_list(&mut self) -> Result<Vec<AST>, ParseError> {
        let mut attrs = Vec::new();
        while self.current == Token::Attribute && self.current != Token::Eof {
            attrs.push(self.parse_attribute_repeats()?);
        }
        Ok(attrs)
    }

    /// attribute_repeats -> "attribute" "{" attribute_def "}"
    fn parse_attribute_repeats(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Attribute)?;
        self.expect(Token::LCurly)?;
        let attr = self.parse_attribute_def()?;
        self.expect(Token::RCurly)?;
        Ok(attr)
    }

    /// attribute_def -> name_def value_def
    fn parse_attribute_def(&mut self) -> Result<AST, ParseError> {
        let name = self.parse_name_def()?;
        let value = self.parse_value_def()?;
        Ok(AST::Attribute {
            name,
            value,
            pos: self.pos(),
        })
    }

    /// value_def -> "value" "=" STRING
    fn parse_value_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::Value)?;
        self.expect(Token::Equal)?;
        self.expect_string()
    }

    /// input_list -> input_repeats {input_repeats}
    fn parse_input_list(&mut self) -> Result<Vec<AST>, ParseError> {
        let mut inputs = Vec::new();
        while self.current == Token::Input && self.current != Token::Eof {
            inputs.push(self.parse_input_repeats()?);
        }
        Ok(inputs)
    }

    /// input_repeats -> "input" "{" value_info_def "}"
    fn parse_input_repeats(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Input)?;
        self.expect(Token::LCurly)?;
        let vi = self.parse_value_info_def()?;
        self.expect(Token::RCurly)?;
        Ok(vi)
    }

    /// output_list -> output_repeats {output_repeats}
    fn parse_output_list(&mut self) -> Result<Vec<AST>, ParseError> {
        let mut outputs = Vec::new();
        while self.current == Token::Output && self.current != Token::Eof {
            outputs.push(self.parse_output_repeats()?);
        }
        Ok(outputs)
    }

    /// output_repeats -> "output" "{" value_info_def "}"
    fn parse_output_repeats(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Output)?;
        self.expect(Token::LCurly)?;
        let vi = self.parse_value_info_def()?;
        self.expect(Token::RCurly)?;
        Ok(vi)
    }

    /// value_info_def -> name_def type_def
    fn parse_value_info_def(&mut self) -> Result<AST, ParseError> {
        let name = self.parse_name_def()?;
        let (elem_type, shape) = self.parse_type_def()?;
        Ok(AST::ValueInfo {
            name,
            elem_type,
            shape,
            pos: self.pos(),
        })
    }

    /// type_def -> "type" "{" tensor_type_def "}"
    fn parse_type_def(&mut self) -> Result<(String, Vec<AST>), ParseError> {
        self.expect(Token::Rtype)?;
        self.expect(Token::LCurly)?;
        let (et, s) = self.parse_tensor_type_def()?;
        self.expect(Token::RCurly)?;
        Ok((et, s))
    }

    /// tensor_type_def -> "tensor_type" "{" elem_type_def shape_def "}"
    fn parse_tensor_type_def(&mut self) -> Result<(String, Vec<AST>), ParseError> {
        self.expect(Token::TensorType)?;
        self.expect(Token::LCurly)?;
        let et = self.parse_elem_type_def()?;
        let s = self.parse_shape_def()?;
        self.expect(Token::RCurly)?;
        Ok((et, s))
    }

    /// elem_type_def -> "elem_type" "=" (int | float | string | bool)
    fn parse_elem_type_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::ElemType)?;
        self.expect(Token::Equal)?;
        self.expect_data_type()
    }

    /// shape_def -> "shape" "{" dim_list "}"
    fn parse_shape_def(&mut self) -> Result<Vec<AST>, ParseError> {
        self.expect(Token::Shape)?;
        self.expect(Token::LCurly)?;
        let dims = self.parse_dim_list()?;
        self.expect(Token::RCurly)?;
        Ok(dims)
    }

    /// dim_list -> dim_repeats {dim_repeats}
    fn parse_dim_list(&mut self) -> Result<Vec<AST>, ParseError> {
        let mut dims = Vec::new();
        while self.current == Token::Dim && self.current != Token::Eof {
            dims.push(self.parse_dim_repeats()?);
        }
        Ok(dims)
    }

    /// dim_repeats -> "dim" "{" dim_def "}"
    fn parse_dim_repeats(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Dim)?;
        self.expect(Token::LCurly)?;
        let dim = self.parse_dim_def()?;
        self.expect(Token::RCurly)?;
        Ok(dim)
    }

    /// dim_def -> (dim_value = INTEGER) | (dim_param = STRING)
    fn parse_dim_def(&mut self) -> Result<AST, ParseError> {
        let dim = if self.current == Token::DimValue {
            self.expect(Token::DimValue)?;
            self.expect(Token::Equal)?;
            let val = self.expect_integer()?;
            AST::Dim {
                dim_value: Some(val),
                dim_param: None,
                pos: self.pos(),
            }
        } else if self.current == Token::DimParam {
            self.expect(Token::DimParam)?;
            self.expect(Token::Equal)?;
            let param = self.expect_string()?;
            AST::Dim {
                dim_value: None,
                dim_param: Some(param),
                pos: self.pos(),
            }
        } else {
            return Err(ParseError::GrammarMismatch("dim_def must be dim_value or dim_param", self.pos()));
        };
        Ok(dim)
    }

    /// initializer_list -> initializer_repeats {initializer_repeats}
    fn parse_initializer_list(&mut self) -> Result<Vec<AST>, ParseError> {
        let mut inits = Vec::new();
        while self.current == Token::Initializer && self.current != Token::Eof {
            inits.push(self.parse_initializer_repeats()?);
        }
        Ok(inits)
    }

    /// initializer_repeats -> "initializer" "{" tensor_def "}"
    fn parse_initializer_repeats(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::Initializer)?;
        self.expect(Token::LCurly)?;
        let init = self.parse_tensor_def()?;
        self.expect(Token::RCurly)?;
        Ok(init)
    }

    /// tensor_def -> name_def data_type_def dims_def raw_data_def
    fn parse_tensor_def(&mut self) -> Result<AST, ParseError> {
        let name = self.parse_name_def()?;
        let data_type = self.parse_data_type_def()?;
        let dims = self.parse_dims_def()?;
        let raw_data = self.parse_raw_data_def()?;

        Ok(AST::Initializer {
            name,
            data_type,
            dims,
            raw_data,
            pos: self.pos(),
        })
    }

    /// data_type_def -> "data_type" "=" (int | float | string | bool)
    fn parse_data_type_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::DataType)?;
        self.expect(Token::Equal)?;
        self.expect_data_type()
    }

    /// dims_def -> "dims" "=" INTEGER { INTEGER }
    fn parse_dims_def(&mut self) -> Result<Vec<i64>, ParseError> {
        self.expect(Token::Dims)?;
        self.expect(Token::Equal)?;

        let mut dims = Vec::new();
        dims.push(self.expect_integer()?);
        while matches!(&self.current, Token::Integer(_)) && self.current != Token::Eof {
            dims.push(self.expect_integer()?);
        }

        Ok(dims)
    }

    /// raw_data_def -> "raw_data" "=" BYTES
    fn parse_raw_data_def(&mut self) -> Result<String, ParseError> {
        self.expect(Token::RawData)?;
        self.expect(Token::Equal)?;
        self.expect_bytes()
    }

    /// opset_import_def -> "opset_import" "{" domain_def version_def "}"
    fn parse_opset_import_def(&mut self) -> Result<AST, ParseError> {
        self.expect(Token::OpsetImport)?;
        self.expect(Token::LCurly)?;
        let domain = self.parse_domain_def()?;
        let version = self.parse_version_def()?;
        self.expect(Token::RCurly)?;

        Ok(AST::OpsetImport {
            domain,
            version,
            pos: self.pos(),
        })
    }

    /// version_def -> "version" "=" INTEGER
    fn parse_version_def(&mut self) -> Result<i64, ParseError> {
        self.expect(Token::Version)?;
        self.expect(Token::Equal)?;
        self.expect_integer()
    }

}
