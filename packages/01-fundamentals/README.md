# 01 - Fundamentals: cargo / `fn` / `Option` / `Result`

> 第一章不教所有权、不教 trait、不教 async。先把"一个最小但**像样**的 Rust 项目"跑起来：cargo 管理、`cargo test` 跑测试、`Option` / `Result` 写正确的错误处理。

## 前置

- 装好 rustup：[rustup.rs](https://rustup.rs/)
- 编辑器装 `rust-analyzer`

## 本章目标

学完后你能：

- `cargo new` / `cargo build` / `cargo run` / `cargo test` / `cargo clippy`
- 区分 `let` / `let mut` / `const`
- 写函数、模块、`pub` / 私有
- 用 `Option<T>` 表示"可能没有"，用 `Result<T, E>` 表示"可能失败"
- 用 `?` 做错误传播
- 写 `match` / `if let`
- 写第一个 `#[test]`

## 如何运行

```bash
cd packages/01-fundamentals

# 编译并跑 CLI
cargo run -- add 1 2          # → 3
cargo run -- divide 10 2      # → 5
cargo run -- divide 1 0       # → 报错，exit 1
cargo run -- even -4          # → true

# 跑测试
cargo test

# 跑 lint（强烈推荐养成习惯）
cargo clippy --all-targets -- -D warnings
```

## 核心概念

### 1. `Option<T>`：可能没有值

```rust
fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

match first_word("hello world") {
    Some(w) => println!("{w}"),
    None => println!("空字符串"),
}
```

替代了"返回 -1 / null 表示找不到"的脏活。`None` 不是 null——编译器强制你处理。

### 2. `Result<T, E>`：可能失败

```rust
fn divide(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 {
        Err("除数为零")
    } else {
        Ok(a / b)
    }
}
```

调用方：

```rust
match divide(10, 0) {
    Ok(r) => println!("{r}"),
    Err(e) => eprintln!("error: {e}"),
}
```

### 3. `?` 运算符：早返回

```rust
fn parse_then_divide(a: &str, b: &str) -> Result<i64, MyErr> {
    let a: i64 = a.parse()?;       // 解析失败 → 立即返回错误
    let b: i64 = b.parse()?;
    divide(a, b)
}
```

`?` = "如果是 `Err`，立即返回；否则解包 `Ok` 的值"。

### 4. `match` 是强制穷尽的

加 enum 变体后所有 match 都会编译失败，逼你处理。

## 代码导读

```
01-fundamentals/
├── Cargo.toml          ← 依赖与构建配置
├── src/
│   ├── lib.rs          ← 库 crate 入口（暴露 calc 模块）
│   ├── calc.rs         ← 业务逻辑 + 单元测试
│   └── main.rs         ← 二进制 crate 入口 / CLI
└── tests/
    └── calc_it.rs      ← 集成测试（黑盒,只用公开 API）
```

读顺序：`src/calc.rs` → `src/lib.rs` → `src/main.rs` → `tests/calc_it.rs` → `Cargo.toml`。

## 常见坑

- **`mut` 写在变量上**，不是类型上：`let mut x = 1`，不是 `let x: mut i64`
- **`String` 和 `&str` 不一样**：`String` 拥有所有权，`&str` 是借用切片；本章不深讲，第 02 章细说
- **整数除法默认向 0 截断**：`-7 / 2 = -3`，不是 `-4`
- **`.parse()` 必须告诉类型**：`let n: i64 = s.parse()?;` 或 `s.parse::<i64>()?`
- **clippy 喷你不是坏事**：它的建议绝大多数都对，认真读

## 延伸阅读

- [The Rust Book (官方)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [`reference/STDLIB_MAP.md`](../../reference/STDLIB_MAP.md)

## 自测

- `let x = 1;` 之后 `x = 2;` 为啥编译失败？
- `Option<T>` 比"返回 -1 表示没有"好在哪？
- `?` 在什么类型的函数里能用？
- `match` 漏写一个 enum 变体会怎样？
- `cargo test` 跑了哪些"测试"？

下一章：[02-ownership-borrowing](../02-ownership-borrowing/)
