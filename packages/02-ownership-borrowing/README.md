# 02 - Ownership, Borrowing & Lifetimes

> Rust 的招牌特性。这章过了——后面 90% 的"编译不过"你能 5 秒看出原因。

## 前置

- [01-fundamentals](../01-fundamentals)

## 本章目标

- 三条所有权规则（见 [`reference/OWNERSHIP_RULES.md`](../../reference/OWNERSHIP_RULES.md)）
- `Copy` vs `Clone` vs move
- 不可变借用 `&T` vs 可变借用 `&mut T`
- 借用规则：任意多个 `&T` **或** 一个 `&mut T`，不能同时
- 生命周期 `'a`：什么时候必须写、什么时候编译器省略
- `&str` vs `String`，`&[T]` vs `Vec<T>`
- `Box<T>` 第一个智能指针：堆分配
- `Rc<T>` / `Arc<T>`：共享引用计数
- `RefCell<T>` / `Cell<T>`：内部可变性（运行期借用检查）
- 非词法生命周期（NLL）

## 推荐工具

- rust-analyzer 的内联类型提示
- 经典练习：[exercism Rust 路径](https://exercism.org/tracks/rust)

## 计划要写

- `src/`: 各种"故意编译失败"的小片段，配编译器错误解读
- `src/`: 同一个函数 4 种参数风格（owned、`&`、`&mut`、`Box`）
- `tests/`: `Rc<RefCell<T>>` 共享状态 demo

## 自测

- `let a = String::from("hi"); let b = a; println!("{a}");` 为啥编译失败？
- `i32` 为啥能直接 `let b = a; println!("{a}");`？
- `&mut T` 同时存在两个会怎样？编译期还是运行期发现？
- `RefCell::borrow_mut()` 两次会怎样？

---

**TODO**: 待补充完整代码与示例。
