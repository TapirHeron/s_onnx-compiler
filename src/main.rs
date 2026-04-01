use clap::Parser;
use s_onnx_compiler::*;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about = "S-ONNX Compiler - 编译原理实验", long_about = None)]
struct Args {
    /// S-ONNX源码文件路径
    input: String,
    /// 三地址码输出文件路径
    #[arg(short, long)]
    output: Option<String>,
    /// 打印抽象语法树(AST)
    #[arg(short, long)]
    print_ast: bool,
    /// 仅执行词法分析
    #[arg(long)]
    only_lex: bool,
    /// 仅执行词法+语法分析
    #[arg(long)]
    only_parse: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("=== S-ONNX Compiler 开始编译: {} ===", args.input);

    // 1. 词法分析
    let mut scanner = lexer::Scanner::new_from_file(&args.input)?;
    if args.only_lex {
        println!("✅ 词法分析结果:");
        loop {
            let token = scanner.next_token()?;
            println!("{}", token);
            if token == lexer::Token::Eof {
                break;
            }
        }
        process::exit(0);
    }

    // 2. 语法分析
    let mut parser = parser::Parser::new(scanner);
    let ast = parser.parse()?;
    println!("✅ 语法分析完成，AST构建成功");
    if args.print_ast {
        utils::print::print_ast(&ast);
    }
    if args.only_parse {
        process::exit(0);
    }

    // 3. 语义分析
    let mut checker = semantic::SemanticChecker::new(ast);
    let checked_ast = checker.check()?;
    println!("✅ 语义检查通过");

    // 4. 中间代码生成
    let mut codegen = codegen::CodeGenerator::new(checked_ast);
    let tac = codegen.generate()?;
    println!("✅ 三地址码(TAC)生成完成");

    // 打印并保存TAC
    utils::print::print_tac(&tac);
    if let Some(out_path) = args.output {
        utils::file::save_tac(&tac, &out_path)?;
        println!("✅ TAC已保存至: {}", out_path);
    }

    println!("=== 编译完成 ===");
    Ok(())
}