# 11 - CLI: clap + anyhow + indicatif

> 01 章 CLI 是手撸 `match`。这章用 **clap derive** 一把梭，配 anyhow / tracing / indicatif。

## 前置

- [01-fundamentals](../01-fundamentals)
- [03-error-handling](../03-error-handling)

## 本章目标

### clap derive 风格
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "calc CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Cmd {
    Add { a: i64, b: i64 },
    Divide { a: i64, b: i64 },
}
```
- 自动生成 `--help` / `--version` / 错误提示
- subcommand 嵌套
- `#[arg(env = "FOO")]`、`default_value`、`value_enum`
- 参数校验：`value_parser = ...`
- 自动 shell completion 生成（`clap_complete`）

### anyhow + 错误展示
- 应用顶层用 `anyhow::Result<()>`
- `.context("doing X failed")` 给错误链加上下文
- 错误打印用 `Debug` 格式：`{:#}` 显示链

### tracing 结构化日志
- `tracing::{info, warn, error, debug, trace}!`
- `tracing_subscriber::fmt()` 初始化
- spans：`#[instrument]` 给函数自动加 span
- log 级别由 `RUST_LOG` 控制

### 进度 / 表格
- `indicatif`：进度条、spinner
- `comfy-table` / `tabled`：终端表格
- `dialoguer`：交互输入
- `colored` / `console`：彩色输出

### 配置
- `config` crate：YAML/TOML/JSON 多源配置
- `figment`：分层配置另一选择
- 路径：`directories-next`（XDG）

## 推荐 crate

- 必备：`clap` (derive 特性)、`anyhow`、`tracing` + `tracing-subscriber`
- 锦上添花：`indicatif`、`dialoguer`、`comfy-table`

## 计划要写

- `src/`: 把 01 章的 calc 改写成 clap 版，加 `--verbose`、`--config`
- `src/`: 一个"下载多个 URL"的 CLI，配 indicatif 多进度条
- `tests/`: `assert_cmd` 端到端测 CLI

## 自测

- clap derive vs builder API 选哪个？
- `anyhow::Result` 在库代码可以用吗？为什么不推荐？
- `tracing::info!` 和 `println!` 区别？
- 如何给 CLI 加 bash/zsh 补全？

---

**TODO**: 待补充完整代码与示例。
