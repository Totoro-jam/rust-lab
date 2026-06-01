//! rustlab01 — 第一章的库。
//!
//! `pub mod calc` 把内部模块暴露出去,bin (`main.rs`) 与集成测试
//! (`tests/calc_it.rs`) 都通过 `rustlab01::calc::xxx` 访问。

pub mod calc;
