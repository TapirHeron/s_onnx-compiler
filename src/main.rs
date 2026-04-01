
use s_onnx_compiler::{lexer, parser, semantic, codegen};
use std::fs;
use s_onnx_compiler::CompilerError;

fn main() -> Result<(), CompilerError> {
    // ==============================
    // 直接读取你的测试用例文件
    // ==============================
    let path = "test.txt"; // 把你的 ModelProto 测试用例保存为 test.txt
    let source = fs::read_to_string(path)
        .map_err(|_| CompilerError::FileOpen(path.into()))?;

    println!("==================================");
    println!("  S-ONNX 编译器开始运行");
    println!("==================================");

    // ==============================
    // 1. 词法分析
    // ==============================
    println!("\n[1/4] 词法分析...");
    let mut scanner = lexer::Scanner::new(&source, path);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        if token == lexer::Token::Eof {
            break;
        }
        tokens.push(token.clone());
        println!("  {}", token);
    }
    println!("✅ 词法分析完成，共 {} 个Token", tokens.len());

    // ==============================
    // 2. 语法分析
    // ==============================
    println!("\n[2/4] 语法分析...");
    let scanner = lexer::Scanner::new(&source, path);
    let mut parser = parser::Parser::new(scanner);
    let ast = parser.parse()?;
    println!("✅ 语法分析完成，AST 构建成功");

    // 打印 AST
    println!("\n=== 抽象语法树 AST ===");
    ast.print(0);

    // ==============================
    // 3. 语义检查
    // ==============================
    println!("\n[3/4] 语义检查...");
    let mut checker = semantic::SemanticChecker::new(ast.clone());
    let checked_ast = checker.check()?;
    println!("✅ 语义检查通过");

    // ==============================
    // 4. 三地址码生成
    // ==============================
    println!("\n[4/4] 三地址码生成...");
    let mut codegen = codegen::CodeGenerator::new(checked_ast);
    let tac_list = codegen.generate()?;

    println!("\n=== 生成的三地址码 TAC ===");
    for inst in tac_list {
        println!("{}", inst);
    }

    println!("\n==================================");
    println!("  编译全部完成 ✅");
    println!("==================================");

    Ok(())
}