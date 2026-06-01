# 所有权与借用规则

## 三条所有权规则

1. **每个值都有一个所有者**。
2. **同一时间只有一个所有者**。
3. **当所有者离开作用域，值被 drop**。

## 借用规则（同一作用域内）

可以同时存在：

- **任意多个不可变引用** `&T`
- **或** 一个可变引用 `&mut T`

**不能同时存在**：`&T` + `&mut T`。

这条规则在编译期由 borrow checker 保证，**消灭了 data race**。

## 生命周期标注

生命周期 `'a` 是引用必须有效的作用域。

```rust
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

返回值的生命周期 = 输入中较短的那个。编译器会校验调用方在那段时间内不释放底层值。

### 省略规则

绝大多数情况下你不用写 `'a`，编译器按三条规则推断：

1. 每个引用参数自有一个生命周期
2. 只有一个输入生命周期 → 应用到所有输出
3. 方法的第一个参数是 `&self` 或 `&mut self` → 输出借 self 的生命周期

## 常见借用错误（中译）

```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
```
→ 你同时持有 `&x` 和 `&mut x`。让 `&x` 提前结束（NLL 已经能识别更多场景）。

```
error[E0382]: borrow of moved value
```
→ 值已被 move（赋值或传给函数），你又想用旧名字。
- 想保留：`.clone()`（接受拷贝代价）或用引用 `&v`
- 或者把"消费"的函数改成"借用"

```
error[E0597]: borrowed value does not live long enough
```
→ 引用比被引用对象活得久。要么把对象提前定义，要么不要返回这个引用。

## 智能指针 vs 引用

| 你想要 | 用 |
|--------|-----|
| 普通借用 | `&T` / `&mut T` |
| 唯一所有权、堆分配 | `Box<T>` |
| 共享引用计数 | `Rc<T>`（单线程）/ `Arc<T>`（多线程） |
| 内部可变性（共享但仍可变） | `Cell<T>` / `RefCell<T>`（单线程）/ `Mutex<T>` / `RwLock<T>`（多线程） |
| 弱引用（破环） | `Weak<T>` |

Rc/Arc + RefCell/Mutex 的组合 ≈ "共享可变状态"，但用起来要谨慎（绕过 borrow checker 到运行期检查）。

## Send / Sync

- `T: Send` → 可以**所有权跨线程**
- `T: Sync` → 可以 **&T 跨线程**（多线程同时读）

绝大多数标准类型自动 Send + Sync。`Rc<T>`、`RefCell<T>` 不 Send。

记忆：跨线程用 `Arc<Mutex<T>>` 就对了。

## 移动 / 借用 / 复制（Copy）

- 实现了 `Copy` 的类型（`i32`、`f64`、`bool`、`(T1, T2)` 当 T1/T2 都 Copy 等）**赋值/传参=按位拷贝**
- 没实现 `Copy` 的（`String`、`Vec<T>`、自定义大多数 struct）**赋值/传参=move**
- 想 deep copy → `.clone()`（要求 `Clone`）
- 想浅拷贝两份所有权 → 不行（这就是 Rust 拒绝 double free 的方式）

## 经验法则

- **函数签名先想"消费还是借用"**：消费用 `T`，只读借用用 `&T`，要改用 `&mut T`
- **返回值默认 owned**，除非有非常清晰的生命周期理由
- 借用编译错误时，**优先看作用域是否能切短**，再考虑加 lifetime 标注
- `clone()` 不是罪，**编程节奏先于性能微调**——先编过再优化
