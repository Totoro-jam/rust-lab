# 06 - Modules, Crates & Cargo

> 学完这章，你能搭出能上 crates.io 的项目骨架。

## 前置

- [01-fundamentals](../01-fundamentals)

## 本章目标

### 模块
- `mod foo;` 与文件布局
  - `src/foo.rs` 或 `src/foo/mod.rs`（推荐前者，2018+ 风格）
- 子模块、嵌套：`src/foo/bar.rs` 配 `mod bar;` in `foo.rs`
- 可见性：默认私有；`pub` / `pub(crate)` / `pub(super)` / `pub(in path)`
- `use` 导入：`use std::collections::HashMap;`
- `pub use` 重导出（控制公开 API 表面）
- 路径：绝对 `crate::foo` / 相对 `self::foo` / 父级 `super::foo`

### Crate
- library vs binary：`lib.rs` vs `main.rs`，可以同 crate 都有
- 多 binary：`src/bin/xxx.rs` 自动暴露
- example：`examples/xxx.rs`，`cargo run --example xxx`
- benches：`benches/xxx.rs` + nightly 或 criterion

### Workspace
- 根 `Cargo.toml` 的 `[workspace]`
- 多包共享 `Cargo.lock` 和 `target/`
- 路径依赖 `path = "../foo"`

### Feature flags
- `[features]` 定义可选功能
- 条件编译 `#[cfg(feature = "x")]`
- 默认特性 `default = ["..."]`
- 别滥用，会导致组合爆炸

### 发布
- `cargo publish` 流程：版本号、`README.md`、`Cargo.toml` 的 `description` / `license` / `repository`
- semver：版本号怎么定
- `cargo doc --open` 生成文档

### Cargo 常用命令
- `cargo new --lib foo`
- `cargo build --release`
- `cargo test`
- `cargo run -- args`
- `cargo check`（不链接，最快验证）
- `cargo fmt` / `cargo clippy`
- `cargo tree` 看依赖
- `cargo update` 更新 Cargo.lock

## 推荐工具

- `cargo-edit`（`cargo add` / `cargo rm`）
- `cargo-watch -x check -x test`
- `cargo-outdated` 查过时依赖

## 计划要写

- `src/`: 一个 lib + 2 个 bin + 1 个 example
- 配 workspace 演示多包
- README 配 feature flags demo

## 自测

- `lib.rs` 和 `main.rs` 能同时存在吗？关系？
- `pub(crate)` 和 `pub` 区别？
- workspace 的 lock 文件在哪？
- feature flag 的 `default` 列表里能加私有依赖吗？

---

**TODO**: 待补充完整代码与示例。
