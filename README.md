# S-ONNX Compiler

一个基于 Rust 实现的简化版 ONNX 模型编译器，支持词法分析、语法分析、语义检查和三地址码生成。

## 项目结构

```
s_onnx_compiler/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── lib.rs               # 库入口
│   ├── lexer/               # 词法分析模块
│   │   ├── mod.rs
│   │   ├── scanner.rs       # DFA 扫描器实现
│   │   ├── token.rs         # Token 定义
│   │   └── error.rs         # 词法错误处理
│   ├── parser/              # 语法分析模块
│   │   ├── mod.rs
│   │   ├── recursive_descent.rs  # 递归下降解析器
│   │   ├── ast.rs           # 抽象语法树定义
│   │   └── error.rs         # 语法错误处理
│   ├── semantic/            # 语义分析模块
│   │   ├── mod.rs
│   │   ├── checker.rs       # 语义检查器
│   │   ├── symbol_table.rs  # 符号表
│   │   └── error.rs         # 语义错误处理
│   ├── codegen/             # 代码生成模块
│   │   ├── mod.rs
│   │   ├── generator.rs     # 代码生成器
│   │   ├── tac.rs           # 三地址码定义
│   │   └── error.rs         # 代码生成错误处理
│   ├── error/               # 错误处理模块
│   │   ├── mod.rs
│   │   ├── error.rs         # 统一错误类型
│   │   └── position.rs      # 位置信息
│   └── utils/               # 工具模块
│       ├── mod.rs
│       ├── file.rs
│       └── print.rs
├── tests/                   # 测试文件
│   └── compiler_test.rs
├── test.txt                 # 测试用例
├── Cargo.toml
└── Cargo.lock
```

## 功能特性

### ✅ 已实现
- **词法分析**：词法扫描器，支持关键字不区分大小写
- **语法分析**：递归下降解析器，生成 AST
- **语义检查**：类型检查、作用域验证、符号表管理
- **代码生成**：生成三地址码 (TAC)
- **错误处理**：完整的错误定位和友好的错误提示

### 📋 支持的 ONNX 元素
- `ModelProto`、`Graph`、`Node`
- `ValueInfo`（输入/输出）
- `Initializer`（张量初始化）
- `Attribute`（属性）
- `OpsetImport`（算子集导入）
- 数据类型：`int`、`float`、`string`、`bool`
- 字面量：整数、字符串、字节数据

## 安装与构建

### 环境要求
- Rust 1.70+（推荐最新版）
- Cargo 包管理器

### 构建项目
```bash
# 克隆项目
git clone https://github.com/TapirHeron/s_onnx-compiler.git
cd s_onnx_compiler

# 编译项目
cargo build

# 编译发布版本
cargo build --release
```

## 使用方法

### 基本用法
```bash
# 完整编译（默认），三地址码自动保存到 test.tac
cargo run --bin s-onnx-compiler -- test.txt

# 仅词法分析
cargo run --bin s-onnx-compiler -- test.txt lexer

# 词法 + 语法分析（输出 AST）
cargo run --bin s-onnx-compiler -- test.txt parser

# 词法 + 语法 + 语义检查
cargo run --bin s-onnx-compiler -- test.txt semantic

# 指定三地址码输出文件
cargo run --bin s-onnx-compiler -- test.txt codegen output.tac

# 保存到指定目录
cargo run --bin s-onnx-compiler -- test.txt codegen ./build/output.tac
```

### IDE 运行配置

在 RustRover/CLion 中：
1. 打开 **运行/调试配置**
2. 找到 **Run s_onnx_compiler**
3. 在 **命令** 栏添加参数：`--bin s-onnx-compiler -- test.txt lexer`

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_official_test_case1

# 运行测试并显示输出
cargo test -- --nocapture
```

## 示例输出

### 词法分析阶段
```
==================================
  S-ONNX 编译器开始运行 (阶段: lexer)
==================================

[1/4] 词法分析...
  ModelProto
  LCurly
  IrVersion
  Equal
  Integer(8)
  ...
✅ 词法分析完成，共 125 个Token
```

### 语法分析阶段
```
[2/4] 语法分析...
✅ 语法分析完成，AST 构建成功

=== 抽象语法树 AST ===
ModelProto {
  ir_version: 8
  producer_name: "onnx-example"
  graph {
    name: "test-model"
    inputs [3]:
      ValueInfo {
        name: "X"
        elem_type: int
        shape:
          dim {
            dim_value: 3
          }
          ...
      }
    ...
  }
}
```

## 文法规则

```
G[model]
model -> “ModelProto”“{”model_body_def “}”
model_body_def -> ir_version_def producer_name_def producer_version_def domain_def model_version_def  doc_string_def graph_def  opset_import_def
ir_version_def -> “ir_version” “=” INTEGER
producer_name_def -> “producer_name” “=” STRING
producer_version_def -> “producer_version” “=” STRING
domain_def -> “domain” “=” STRING
model_version_def -> “model_version” “=” INTEGER
doc_string_def -> “doc_string” “=” STRING
graph_def -> “graph” “{” graph_body_def “}”
graph_body_def -> name_def  node_list  input_list  output_list [initializer_list] 
name_def -> “name” “=” STRING
node_list ->  node_repeats {node_repeats}
node_repeats->“node” “{” node_def “}”
input_list -> input_repeats {input_repeats}
input_repeats-> “input” “{” value_info_def  “}”
output_list ->  output_repeats {output_repeats} 
output_repeats->“output” “{” value_info_def “}”
initializer_list->initializer_repeats  {initializer_repeats} 
initializer_repeats-> “initializer” “{”tensor_def  “}”
node_def -> op_type_def  name_def  (input_list | input_arr ) (output_list | output_arr)   [attribute_list]
op_type_def -> “op_type” “=” STRING
input_arr -> “input”“=”“[”STRING { “,”STRING }“]”
output_arr -> “output”“=”“[”STRING { “,”STRING }“]”
attribute_list-> attribute_repeats {attribute_repeats}
attribute_repeats -> “attribute” “{”attribute_def   “}”
attribute_def -> name_def  value_def 
value_def -> “value” “=” STRING
value_info_def -> name_def  type_def
type_def -> “type” “{” tensor_type_def “}”
tensor_type_def -> “tensor_type” “{” elem_type_def  shape_def “}”
elem_type_def -> “elem_type” “=”  (“int” | “float” | “string” | “bool”)
shape_def -> “shape” “{” dim_list “}”
dim_list ->  dim_repeats {dim_repeats}
dim_repeats -> “dim” “{” dim_def   “}”
dim_def -> (“dim_value” “=” INTEGER) | (“dim_param” “=” STRING)
tensor_def -> name_def  data_type_def  dims_def  raw_data_def
data_type_def -> “data_type” “=” (“int” | “float” | “string” | “bool”)
dims_def ->  “dims” “=” INTEGER  {INTEGER}
raw_data_def -> “raw_data” “=” BYTES
opset_import_def -> “opset_import” “{” domain_def  version_def “}”
version_def -> “version” “=” INTEGER
```

## 错误处理

编译器提供详细的错误信息，包括：
- 错误类型（词法/语法/语义/代码生成）
- 错误位置（文件、行号、列号）
- 错误描述和建议

示例：
```
语法错误: 期望 Token 'RCurly'，但找到 'Eof' at test.txt:87:5
```

## 技术栈

- **语言**：Rust
- **解析器**：递归下降（手写）
- **错误处理**：thiserror
- **测试框架**：Rust 标准测试

## 许可证

本项目采用 [MIT License](LICENSE) 许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 联系方式

如有问题或建议，欢迎通过 Issue 联系。
