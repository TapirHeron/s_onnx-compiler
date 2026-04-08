use s_onnx_compiler::{lexer, parser, semantic, codegen, utils};
use std::fs;
use std::path::Path;
use s_onnx_compiler::CompilerError;
use utils::file;
fn main() -> Result<(), CompilerError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <文件路径> [阶段] [输出文件]", args[0]);
        eprintln!("阶段选项:");
        eprintln!("  lexer    - 仅输出词法分析结果");
        eprintln!("  parser   - 输出词法和语法分析结果(AST)");
        eprintln!("  semantic - 输出到语义检查阶段");
        eprintln!("  codegen  - 完整编译(默认)");
        eprintln!("\n输出文件: 可选，保存三地址码的路径。如不指定则保存到源文件同目录下的 <源文件名>.tac");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let stage = if args.len() > 2 { args[2].to_lowercase() } else { "codegen".to_string() };
    let output_file = if args.len() > 3 { args[3].clone() } else { String::new() };

    let static_path: &'static str = Box::leak(file_path.clone().into_boxed_str());
    let source = fs::read_to_string(file_path)
        .map_err(|e| CompilerError::FileOpen(format!("{}: {}", file_path, e)))?;

    println!("==================================");
    println!("  S-ONNX 编译器开始运行 (阶段: {})", stage);
    println!("==================================");

    // ==============================
    // 1. 词法分析
    // ==============================
    println!("\n[1/4] 词法分析...");
    let mut scanner = lexer::Scanner::new(&source, static_path);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        if token == lexer::Token::Eof {
            break;
        }
        tokens.push(token.clone());
        if stage == "lexer" {
            println!("  {}", token);
        }
    }
    println!("✅ 词法分析完成，共 {} 个Token", tokens.len());
    if stage == "lexer" {
        return Ok(());
    }

    // ==============================
    // 2. 语法分析
    // ==============================
    println!("\n[2/4] 语法分析...");
    let scanner = lexer::Scanner::new(&source, static_path);
    let mut parser = parser::Parser::new(scanner);
    let ast = parser.parse()?;
    println!("✅ 语法分析完成，AST 构建成功");

    if stage == "parser" {
        println!("\n=== 抽象语法树 AST ===");
        ast.print(0);
        return Ok(());
    }

    // ==============================
    // 3. 语义检查
    // ==============================
    println!("\n[3/4] 语义检查...");
    let mut checker = semantic::SemanticChecker::new(ast.clone());
    let checked_ast = checker.check()?;
    println!("✅ 语义检查通过");

    if stage == "semantic" {
        return Ok(());
    }

    // ==============================
    // 4. 三地址码生成
    // ==============================
    println!("\n[4/4] 三地址码生成...");
    let mut codegen = codegen::CodeGenerator::new(checked_ast);
    let tac_list = codegen.generate()?;

    println!("\n=== 生成的三地址码 TAC ===");
    let tac_content: Vec<String> = tac_list.iter().map(|inst| inst.to_string()).collect();
    for line in &tac_content {
        println!("{}", line);
    }

    // 保存三地址码到文件
    if stage == "codegen" {
        let output_path = if output_file.is_empty() {
            let path = Path::new(file_path);
            let file_stem = path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "output".to_string());

            // 使用 PathBuf 自动保留原目录，只替换文件名
            let mut output = path.to_path_buf();
            output.set_file_name(format!("{}.tac", file_stem));
            output.to_string_lossy().to_string()
        } else {
            output_file
        };


        match file::save_tac(&tac_list, &output_path) {
            Ok(_) => println!("\n✅ 三地址码已保存到: {}", output_path),
            Err(e) => eprintln!("\n❌ 保存失败: {}", e),
        }
    }

    println!("\n==================================");
    println!("  编译全部完成 ✅");
    println!("==================================");

    Ok(())
}
