# rust-lab

> 一个从零起手、循序渐进的 **Rust 教学仓库**。每一章都是可独立 `cargo run` 的小项目，配最常用的 crate。
>
> 设计哲学：**借用检查器是朋友 / 优先 `Result<T, E>` / 优先标准库迭代器 / `cargo clippy` 是必经**。

## 前置安装

- **Rust** ≥ 1.86（稳定版 toolchain）
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup default stable
  ```
  官方文档：[rustup.rs](https://github.com/rust-lang/rustup)

- **编辑器**：推荐装 [rust-analyzer](https://github.com/rust-lang/rust-analyzer)

- **可选工具**：
  ```bash
  cargo install cargo-watch cargo-nextest
  ```
  - [cargo-watch](https://github.com/watchexec/cargo-watch) — 文件变化自动重跑
  - [cargo-nextest](https://github.com/nextest-rs/nextest) — 更快的 test runner

## 学习路径

| #  | 章节 | 关键词 |
|----|------|--------|
| 01 | [fundamentals](./packages/01-fundamentals) | cargo、`fn`、`let`、`mut`、`Option`、`Result` |
| 02 | [ownership-borrowing](./packages/02-ownership-borrowing) | move / borrow / lifetime / `Box` / `Rc` / `RefCell` |
| 03 | [error-handling](./packages/03-error-handling) | `?` / `thiserror` / `anyhow` / `context()` |
| 04 | [traits-generics](./packages/04-traits-generics) | trait 定义、`impl Trait`、`dyn Trait`、bounds |
| 05 | [collections-iterators](./packages/05-collections-iterators) | `Vec` / `HashMap` / `HashSet` / `BTreeMap`、`Iterator` 链式 |
| 06 | [modules-cargo](./packages/06-modules-cargo) | mod、crate、workspace、feature flags、`pub(crate)` |
| 07 | [testing-bench](./packages/07-testing-bench) | `#[test]`、proptest、rstest、criterion bench |
| 08 | [async-tokio](./packages/08-async-tokio) | `async/await`、tokio、channels、`join!`、`select!` |
| 09 | [serde-json](./packages/09-serde-json) | `Serialize`/`Deserialize`、tagged enum、flatten |
| 10 | [http-axum-reqwest](./packages/10-http-axum-reqwest) | axum CRUD 服务端、tower middleware |
| 11 | [cli-clap-anyhow](./packages/11-cli-clap-anyhow) | clap derive 子命令、tracing、assert_cmd |
| 12 | [popular-crates](./packages/12-popular-crates) | rayon、regex、chrono、itertools、uuid、bytes |

## 怎么用

### 单个章节

```bash
cd packages/01-fundamentals
cargo test              # 跑测试
cargo run -- add 1 2    # 有 bin 的章节可以 run
cargo clippy --all-targets -- -D warnings
```

### 根目录跑全部测试

```bash
# 主 workspace（01–05, 07–12）
cargo test --workspace

# 第 6 章有独立 workspace（演示 workspace 概念），单独跑
cargo test --workspace --manifest-path packages/06-modules-cargo/Cargo.toml

# 一条命令全跑
cargo test --workspace && cargo test --workspace --manifest-path packages/06-modules-cargo/Cargo.toml
```

### 完整验证（推送前必跑）

```bash
cargo clippy --workspace --all-targets -- -D warnings \
  && cargo fmt --all -- --check \
  && cargo test --workspace \
  && cd packages/06-modules-cargo \
  && cargo clippy --workspace --all-targets -- -D warnings \
  && cargo fmt --all -- --check \
  && cargo test --workspace
```

## 项目结构

```
rust-lab/
├── Cargo.toml                    # 根 workspace
├── .github/workflows/ci.yml     # CI: clippy + fmt + test
├── packages/
│   ├── 01-fundamentals/          # 每章独立 cargo 包
│   ├── 02-ownership-borrowing/
│   ├── ...
│   ├── 06-modules-cargo/         # 独立 workspace（内含子 crate）
│   │   ├── Cargo.toml            #   workspace 根
│   │   └── crates/
│   │       ├── mathlib/          #   库 crate（feature flags）
│   │       └── mathcli/          #   二进制 crate
│   └── 12-popular-crates/
└── reference/                    # 横向参考资料
    ├── OWNERSHIP_RULES.md
    ├── STDLIB_MAP.md
    ├── PATTERNS.md
    ├── ANTIPATTERNS.md
    └── COMPARISON.md
```

## reference/

横向参考资料，与具体章节解耦：

- [`OWNERSHIP_RULES.md`](./reference/OWNERSHIP_RULES.md) — 所有权三原则与典型借用错误
- [`STDLIB_MAP.md`](./reference/STDLIB_MAP.md) — `std::` 怎么选
- [`PATTERNS.md`](./reference/PATTERNS.md) — Rusty 的常见模式
- [`ANTIPATTERNS.md`](./reference/ANTIPATTERNS.md) — 别这么写
- [`COMPARISON.md`](./reference/COMPARISON.md) — vs C++ / Go / TypeScript

---

相关仓库：[python-lab](https://github.com/Totoro-jam/python-lab) / [c-lab](https://github.com/Totoro-jam/c-lab) / [frontend-expert-roadmap](https://github.com/Totoro-jam/frontend-expert-roadmap) / [testing-lab](https://github.com/Totoro-jam/testing-lab)
