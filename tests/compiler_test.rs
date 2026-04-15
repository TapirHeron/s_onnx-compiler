use s_onnx_compiler::lexer::Scanner;
use s_onnx_compiler::parser::Parser;
use s_onnx_compiler::semantic::SemanticChecker;
use s_onnx_compiler::codegen::CodeGenerator;
use std::fs::File;
use std::io::Write;

fn run_test_case(test_name: &str, source: &str) {
    let mut output = String::new();
    
    output.push_str(&format!("\n{}\n", "=".repeat(80)));
    output.push_str(&format!("测试用例: {}\n", test_name));
    output.push_str(&format!("{}\n", "=".repeat(80)));

    // 1. 词法分析
    output.push_str("\n【词法分析模块】\n");
    let mut scanner = Scanner::new(source, "test_input");
    let mut tokens = Vec::new();
    loop {
        match scanner.next_token() {
            Ok(token) => {
                if token == s_onnx_compiler::Token::Eof {
                    break;
                }
                tokens.push(token.clone());
            }
            Err(e) => {
                output.push_str(&format!("词法分析错误: {:?}\n", e));
                save_output(test_name, &output);
                return;
            }
        }
    }
    output.push_str(&format!("Token数量: {}\n", tokens.len()));
    for (i, token) in tokens.iter().enumerate() {
        output.push_str(&format!("  [{}] {}\n", i + 1, token));
    }

    // 2. 语法分析
    output.push_str("\n【语法分析模块】\n");
    let mut parser = Parser::new(Scanner::new(source, "test_input"));
    match parser.parse() {
        Ok(ast) => {
            output.push_str("语法分析成功!\n");
            output.push_str("AST结构:\n");
            ast.print_to_string(&mut output, 0);

            // 3. 语义分析
            output.push_str("\n【语义分析模块】\n");
            let mut checker = SemanticChecker::new(ast.clone());
            match checker.check() {
                Ok(checked_ast) => {
                    output.push_str("语义分析成功!\n");

                    // 4. 中间代码生成
                    output.push_str("\n【中间代码生成模块】\n");
                    let mut generator = CodeGenerator::new(checked_ast);
                    match generator.generate() {
                        Ok(tac_list) => {
                            output.push_str("中间代码生成成功!\n");
                            output.push_str(&format!("三地址码指令数: {}\n", tac_list.len()));
                            for (i, tac) in tac_list.iter().enumerate() {
                                output.push_str(&format!("  [{}] {}\n", i + 1, tac));
                            }
                        }
                        Err(e) => {
                            output.push_str(&format!("中间代码生成错误: {:?}\n", e));
                        }
                    }
                }
                Err(e) => {
                    output.push_str(&format!("语义分析错误: {:?}\n", e));
                }
            }
        }
        Err(e) => {
            output.push_str(&format!("语法分析错误: {:?}\n", e));
        }
    }
    
    save_output(test_name, &output);
}

fn save_output(test_name: &str, content: &str) {
    // 从测试名称中提取第一个连续数字
    let test_num = test_name
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    
    let test_num = if test_num.is_empty() { "unknown" } else { &test_num };
    let filename = format!("test_{}.txt", test_num);
    let mut file = File::create(&filename).expect(&format!("无法创建文件: {}", filename));
    file.write_all(content.as_bytes()).expect(&format!("无法写入文件: {}", filename));
    println!("输出已保存到: {}", filename);
}

#[test]
fn test_case_1() {
    let source = r#"
    ModelProto{
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
    run_test_case("测试用例1 - Pad操作符模型", source);
}

#[test]
fn test_case_2() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel2."
    graph{
        name = "GraphWithInitializer"
        node{
            op_type = "MatMul"
            name = "MatMulNode"
            input = ["input1", "initializer1"]
            output = ["output1"]
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
                        dim{
                            dim_value = 2
                        }
                    }
                }
            }
        }
        output{
            name = "output1"
            type{
                tensor_type{
                    elem_type = float
                    shape{
                        dim{
                            dim_value = 1
                        }
                        dim{
                            dim_value = 1
                        }
                    }
                }
            }
        }
initializer{
            name = "initializer1"
            data_type = float
            dims = 2 1
            raw_data = 01020304b
        }
    }
    opset_import{
        domain = "ai.onnx"
        version = 11
    }
}"#;
    run_test_case("测试用例2 - MatMul带初始化器", source);
}

#[test]
fn test_case_3() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel3."
    graph{
        name = "MultiNodeGraph"
        node{
            op_type = "Mul"
            name = "MulNode"
            input = ["input1", "input2", "input3"]
            output = ["output1"]
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
        input{
            name = "input3"
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
    run_test_case("测试用例3 - 多输入Mul操作符", source);
}

#[test]
fn test_case_4() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel4."
    graph{
        name = "ComplexShapeGraph"
        node{
            op_type = "Conv"
            name = "ConvNode"
            input = ["input1", "initializer1"]
            output = ["output1"]
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
                        dim{
                            dim_value = 3
                        }
                        dim{
                            dim_value = 224
                        }
                        dim{
                            dim_value = 224
                        }
                    }
                }
            }
        }

        output{
            name = "output1"
            type{
                tensor_type{
                    elem_type = float
                    shape{
                        dim{
                            dim_value = 1
                        }
                        dim{
                            dim_value = 64
                        }
                        dim{
                            dim_value = 222
                        }
                        dim{
                            dim_value = 222
                        }
                    }
                }
            }
        }
        initializer{
            name = "initializer1"
            data_type = float
            dims = 64 3 3 3
            raw_data = 0102b
        }
    }
    opset_import{
        domain = "ai.onnx"
        version = 11
    }
}"#;
    run_test_case("测试用例4 - Conv卷积操作符", source);
}

#[test]
fn test_case_5() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel5."
    graph{
        name = "NodeWithStringAttrGraph"
        node{
            op_type = "CustomOp"
            name = "CustomNode"
            input = ["input1"]
            output = ["output1"]
            attribute{
                name = "customAttr"
                value = "SomeStringValue"
            }
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
        output{
            name = "output1"
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
    run_test_case("测试用例5 - 自定义操作符带字符串属性", source);
}

#[test]
fn test_case_6() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel6."
    graph{
        name = "MultiIOGraph"
        node{
            op_type = "Add"
            name = "AddNode"
            input = ["input1", "input2"]
            output = ["output1", "output2"]
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
    run_test_case("测试用例6 - 多输出Add操作符", source);
}

#[test]
fn test_case_7() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "Test"
    graph{
        node{
            op_type = "Add"
            name = "Node1"
            input = ["input1", "input2"]
            output = ["output1"]
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
    keyword = "value"
}"#;
    run_test_case("测试用例7 - 包含非法关键字", source);
}

#[test]
fn test_case_8() {
    let source = r#"
    ModelProto{
    ir_version = 1
    producer_name = "Test"
    graph{
        node{
            op_type = "Add"
            name = "Node1"
            input = ["input1", "input2"]
            output = ["output1"]
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
}
}"#;
    run_test_case("测试用例8 - 缺少opset_import且多余右括号", source);
}

#[test]
fn test_case_9() {
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
    run_test_case("测试用例9 - 节点重名和类型不匹配", source);
}

#[test]
fn test_case_10() {
    let source = r#"ModelProto{
    ir_version = 1
    producer_name = "TestProducer"
    producer_version = "1.0"
    domain = "test.onnx"
model_version = 1
doc_string = "This is testmodel10."
    graph{
        name = "SyntaxErrorGraph"
        node{
            op_type = "Add"
            name = "AddNode"
            input = ["input1", "input2"]
            output = ["output1"]
        }
    }
    opset_import{
        domain = "ai.onnx"
        version = 11
    }
}"#;
    run_test_case("测试用例10 - 缺少输入输出定义", source);
}
