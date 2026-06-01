# 03 - Error Handling

> `Result<T, E>` + `?` 是 Rust 的核心日常。配 `thiserror`（库）/ `anyhow`（应用）让代码省一半。

## 前置

- [01-fundamentals](../01-fundamentals)

## 本章目标

- `panic!` 何时合理（不变量违反、调试断言）
- `Option<T>` / `Result<T, E>` 复习与组合子：
  - `.map` / `.and_then` / `.or_else` / `.unwrap_or` / `.unwrap_or_else`
  - `Option` ↔ `Result` 转换：`ok_or` / `ok_or_else`、`.ok()`
- `?` 运算符 + `From::from` 自动转换
- 库代码：用 **`thiserror`** 派生具体错误 enum
  ```rust
  #[derive(thiserror::Error, Debug)]
  enum MyError {
      #[error("io: {0}")] Io(#[from] std::io::Error),
      #[error("parse: {0}")] Parse(#[from] std::num::ParseIntError),
  }
  ```
- 应用代码：用 **`anyhow::Result<T>`** 聚合任意错误 + `.context("...")`
- 何时区分库/应用边界
- `Result::is_ok()` / `if let Err(e) = ...`
- `panic_unwind` vs `panic_abort`

## 推荐 crate

- `thiserror`：库用
- `anyhow`：应用用
- `eyre` / `color-eyre`：anyhow 同类，错误打印更漂亮

## 计划要写

- `src/`: 一个文件 CSV 解析的小程序，分别用 thiserror 和 anyhow 写两遍
- `tests/`: `assert!(matches!(err, MyError::Parse(_)))`

## 自测

- `?` 在 `Option<T>` 返回类型的函数里能用吗？
- 库为啥不该返 `Box<dyn Error>`？
- `unwrap()` 和 `expect("...")` 区别？什么时候用？
- 如何把 `Option<T>` 变成 `Result<T, E>`？

---

**TODO**: 待补充完整代码与示例。
