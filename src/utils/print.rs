use crate::{parser::AST, codegen::TAC};

/// 格式化打印工具
pub mod print {
    use super::*;

    /// 打印AST(缩进式，父子级关系清晰)
    pub fn print_ast(ast: &AST) {
        println!("\n=== 抽象语法树(AST) ===");
        ast.print(0);
        println!("\n=== AST打印完成 ===");
    }

    /// 打印三地址码(TAC)
    pub fn print_tac(tac: &[TAC]) {
        println!("\n=== 三地址码(TAC) ===");
        for (idx, inst) in tac.iter().enumerate() {
            println!("{:4}: {}", idx + 1, inst);
        }
        println!("\n=== TAC打印完成 ===");
    }
}