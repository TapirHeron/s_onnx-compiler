use s_onnx_compiler::*;
use pretty_assertions::assert_eq;
use s_onnx_compiler::lexer::LexError;
use s_onnx_compiler::parser::ParseError;
use s_onnx_compiler::semantic::SemanticError;

// 测试用例1: 官方测试用例1完整解析
#[test]
fn test_official_test_case1() {
    let source =
    r#"ModelProto {
        ir_version = 8
        producer_name = "onnx-example"
        producer_version = "1.0"
        domain = "example_domain"
        model_version = 1
        doc_string = "This is an example ONNX model."
        graph {
            name= "test-model"
            node {
                op_type= "Pad"
                name= "test-node"
                input=["X","pads","value"]
                output=[ "Y" ]
                attribute {
                    name= "mode"
                    value = "33"
                }
            }
            input {
                name= "X"
                type {
                    tensor_type {
                        elem_type= int
                            shape {
                                dim {
                                    dim_value= 3
                                }
                                dim {
                                    dim_value= 2
                                }
                            }
                        }
                    }
                }
            input {
                name="pads"
                type {
                    tensor_type {
                        elem_type= int
                        shape {
                            dim {
                                dim_value= 1
                            }
                            dim {
                                dim_value= 4
                            }
                        }
                    }
                }
            }
            input {
                name= "value"
                type {
                    tensor_type {
                        elem_type= int
                        shape {
                            dim {
                                dim_value= 1
                            }
                        }
                    }
                }
            }
            output {
                name="Y"
                type {
                    tensor_type {
                        elem_type= int
                        shape {
                            dim {
                                dim_value=3
                            }
                            dim {
                                dim_value=4
                            }
                        }
                    }
                }
            }
            initializer {
                name= "conv.bias"
                data_type=int
                dims=1 2 3 4
                raw_data=000000000000b
            }
        }
        opset_import {
            domain = "ex"
            version=15
        }
    }"#;

    // 1. 词法分析
    let mut scanner = lexer::Scanner::new(source, "test_case1.txt");
    let mut tokens = Vec::new();
    loop {
        let token = scanner.next_token().unwrap();
        if token == Token::Eof {
            break;
        }
        tokens.push(token);
    }
    assert!(!tokens.is_empty(), "词法分析未生成任何Token");

    // 2. 语法分析
    let scanner = lexer::Scanner::new(source, "test_case1.txt");
    let mut parser = parser::Parser::new(scanner);
    let ast = parser.parse().expect("语法分析失败");

    // 验证AST结构
    if let AST::ModelProto { graph, opset_import, .. } = &ast {
        // 验证graph
        if let AST::Graph { name, nodes, inputs, outputs, initializers, .. } = &**graph {
            assert_eq!(name, "test-model", "Graph名称不匹配");
            assert_eq!(nodes.len(), 1, "Node数量不匹配");
            assert_eq!(inputs.len(), 3, "Input数量不匹配");
            assert_eq!(outputs.len(), 1, "Output数量不匹配");
            assert!(initializers.is_some(), "缺少Initializer");
            assert_eq!(initializers.as_ref().unwrap().len(), 1, "Initializer数量不匹配");
        } else {
            panic!("AST Graph节点类型错误");
        }

        // 验证opset_import
        if let AST::OpsetImport { domain, version, .. } = &**opset_import {
            assert_eq!(domain, "ex", "OpsetImport domain不匹配");
            assert_eq!(version, &15, "OpsetImport version不匹配");
        } else {
            panic!("AST OpsetImport节点类型错误");
        }
    } else {
        panic!("AST根节点不是ModelProto");
    }

    // 3. 语义分析
    let mut checker = semantic::SemanticChecker::new(ast);
    let checked_ast = checker.check().expect("语义检查失败");

    // 4. 代码生成
    let mut codegen = codegen::CodeGenerator::new(checked_ast);
    let tac = codegen.generate().expect("TAC生成失败");
    for inst in &tac {
        println!("{:?}", inst);
    }
    assert!(!tac.is_empty(), "未生成任何TAC指令");
    // 验证TAC指令类型
    let mut has_input = false;
    let mut has_operation = false;
    let mut has_initializer = false;
    let mut has_output = false;
    for inst in &tac {
        match inst {
            TAC::Input { .. } => has_input = true,
            TAC::Operation { .. } => has_operation = true,
            TAC::Initializer { .. } => has_initializer = true,
            TAC::Output { .. } => has_output = true,
            TAC::Comment(_) => todo!(),
        }
    }
    assert!(has_input, "缺少Input类型TAC指令");
    assert!(has_operation, "缺少Operation类型TAC指令");
    assert!(has_initializer, "缺少Initializer类型TAC指令");
    assert!(has_output, "缺少Output类型TAC指令");

    println!("✅ 官方测试用例1全流程测试通过!");
}

// 测试用例2: 语义错误-节点名称重复(测试用例9)
#[test]
fn test_semantic_error_duplicate_node_name() {
    let source = r#"ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel9."
    graph{
        name = "DuplicateNodeNameGraph"
        node{
            op_type = "Add"
            name = "DuplicateNode"
            input = ["input1"]
            output = ["output1"]
        }
        node{
            op_type = "Add"
            name = "DuplicateNode"
            input = ["input2"]
            output = ["output2"]
        }
input{
            name = "input1"
            type{
                tensor_type{
                    elem_type = float
                    shape{
                        dim{
                            dim_value = 1
                        }
                    }
                }
            }
        }
        input{
            name = "input2"
            type{
                tensor_type{
                    elem_type = float
                    shape{
                        dim{
                            dim_value = 1
                        }
                    }
                }
            }
        }
        output{
            name = "output1"
            type{
                tensor_type{
                    elem_type = int
                    shape{
                        dim{
                            dim_value = 1
                        }
                    }
                }
            }
            }
output{
            name = "output2"
            type{
                tensor_type{
                    elem_type = float
                    shape{
                        dim{
                            dim_value = 1
                        }
                    }
                }
            }
        }
    }
    opset_import{
        domain = "ai.onnx"
        version = 11
    }
}"#;

    // 1. 词法+语法分析通过
    let scanner = lexer::Scanner::new(source, "test_case9.txt");
    let mut parser = parser::Parser::new(scanner);
    let ast = parser.parse().expect("语法分析失败");

    // 2. 语义分析应该报错(节点名称重复)
    let mut checker = semantic::SemanticChecker::new(ast);
    let result = checker.check();
    assert!(result.is_err(), "应该检测到节点名称重复的语义错误");
    match result.err().unwrap() {
        SemanticError::NamingConflict(msg) => {
            assert!(msg.contains("节点名称重复"), "错误信息不正确: {}", msg);
            assert!(msg.contains("DuplicateNode"), "错误信息未包含重复节点名: {}", msg);
        }
        _ => panic!("语义错误类型不正确"),
    }

    println!("✅ 语义错误-节点名称重复测试通过!");
}

// 测试用例3: 语法错误-缺少必要符号(测试用例10简化版)
#[test]
fn test_syntax_error_missing_symbol() {
    let source = r#"ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
    model_version = 1
    doc_string = "This is testmodel10."
    graph {
        name = "SyntaxErrorGraph"
        node{
            op_type = "Add"
            name = "AddNode"
            input = ["input1", "input2"]
            output = ["output1"]
        }
    opset_import{
        domain = "ai.onnx"
        version = 11
    }
}"#;

    // 语法分析应该报错(缺少}))
    let scanner = lexer::Scanner::new(source, "test_case10.txt");
    let mut parser = parser::Parser::new(scanner);
    let result = parser.parse();
    assert!(result.is_err(), "应该检测到语法错误");
    match result.err().unwrap() {
        ParseError::MissingSymbol(sym, _) => {
            assert_eq!(Token::RCurly, sym, "错误信息应提示缺少}}");
        }
        ParseError::UnexpectedToken(token, _) => {
            assert_eq!(token, Token::OpsetImport, "应该在opset_import处检测到意外Token");
        }
        _ => panic!("语法错误类型不正确"),
    }

    println!("✅ 语法错误-缺少必要符号测试通过!");
}

// 测试用例4: 词法错误-未闭合字符串
#[test]
fn test_lex_error_unclosed_string() {
    let source = r#"ModelProto{
        ir_version = 8
        producer_name = "onnx-example"
        producer_version = "1.0"
        domain = "example_domain"
        model_version = 1
        doc_string = "This is an example ONNX model.
        graph {
           name= "test-model"
        }
    }"#;

    // 词法分析应该报错(未闭合字符串)
    let mut scanner = lexer::Scanner::new(source, "lex_error_test.txt");
    let mut tokens = Vec::new();
    let mut has_error = false;
    loop {
        match scanner.next_token() {
            Ok(Token::Eof) => break,
            Ok(token) => tokens.push(token),
            Err(LexError::UnclosedString(_, _)) => {
                has_error = true;
                break;
            }
            Err(e) => {
                panic!("Unexpected lex error: {}", e);
            }
        }
    }
    println!("tokens: {:?}", tokens);
    assert!(has_error, "应该检测到未闭合字符串的词法错误");

    println!("✅ 词法错误-未闭合字符串测试通过!");
}