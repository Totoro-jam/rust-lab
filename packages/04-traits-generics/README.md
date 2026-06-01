# 04 - Traits & Generics

> Rust 的多态系统。trait = 接口 + extension method；泛型 = 编译期单态化（零运行期开销）。

## 前置

- [02-ownership-borrowing](../02-ownership-borrowing)

## 本章目标

- `trait` 定义：方法签名、默认实现
- `impl Trait for Type` 给类型实现 trait
- `derive` 派生：`#[derive(Debug, Clone, PartialEq, Eq, Hash)]`
- 泛型函数 `fn f<T: Trait>(x: T)` / `where` 子句
- `impl Trait` 作参数 / 作返回值
- `dyn Trait` 动态分发（运行期）vs `impl Trait` / 泛型 静态分发（编译期）
- 对象安全（object safety）：哪些 trait 能 `dyn`、哪些不能
- 关联类型 `type Item;`（vs 泛型参数）
- 关联常量 `const X: i32;`
- 标记 trait：`Send` / `Sync` / `Copy` / `Sized` / `?Sized`
- 孤儿规则（orphan rule）：你只能为自己定义的 trait 或自己定义的类型实现
- newtype + `Deref` 绕开孤儿规则
- 操作符重载（`Add`、`Sub`、`Index` 等）

## 推荐工具

- rust-analyzer 看 trait bound 是否满足
- `cargo expand` 看派生宏展开的代码

## 计划要写

- `src/`: 自定义 `Shape` trait + `Circle` / `Rectangle`，两种风格调度（`dyn` vs 泛型）
- `src/`: 实现 `Iterator` 给自己的类型
- `src/`: 用 newtype 给 `Vec<u8>` 加自定义 trait
- `tests/`: trait 对象集合 `Vec<Box<dyn Shape>>`

## 自测

- `impl Trait` 作返回值和返回 `Box<dyn Trait>` 区别？
- 什么 trait 不能做 `dyn`？为什么？
- 派生 `Clone` 要求所有字段都 `Clone` 吗？
- 关联类型 vs 泛型参数，什么时候用哪个？

---

**TODO**: 待补充完整代码与示例。
