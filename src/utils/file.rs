use crate::codegen::TAC;
use anyhow::Result;
use std::fs::{read_to_string, write};
pub use file::{read_source, save_tac};
/// 文件工具 - 读取S-ONNX源码、保存TAC
pub mod file {
    use super::*;

    /// 读取S-ONNX源码文件
    pub fn read_source(path: &str) -> Result<String> {
        read_to_string(path)
            .map_err(|e| anyhow::anyhow!("读取文件失败: {} - {}", path, e))
    }

    /// 保存三地址码到文件
    pub fn save_tac(tac: &Vec<TAC>, path: &str) -> Result<()> {
        let content = tac.iter()
            .map(|inst| inst.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write(path, content)
            .map_err(|e| anyhow::anyhow!("保存TAC失败: {} - {}", path, e))?;
        Ok(())
    }
}
