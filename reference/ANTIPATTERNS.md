# Rust 反模式

## 类型 / 借用

### ❌ 到处 `.unwrap()` / `.expect("...")`

库代码不该 unwrap。返回 `Result`，让调用方决定。`unwrap` 留给"不变量保证"和示例代码。

### ❌ 一切都 `Rc<RefCell<T>>`

通常是借用检查没绕过来的"投降"。先想想能不能用：
- 重组数据结构（避免共享可变）
- 把 `&mut` 切短一些
- 用消息传递（channel）替代共享状态

`Rc<RefCell<T>>` 把编译期错误推到运行期 panic。

### ❌ 改 `Box::leak` 当 `'static`

```rust
let s: &'static str = Box::leak(String::from("x").into_boxed_str()); // 永久泄漏
```
真的需要 `'static` 时优先 `String` + 用 `&s`、或 `OnceLock`、或常量字面量。

### ❌ 不必要的 `clone()`

```rust
let n = data.clone().len();   // 拷贝整个 vec 只为 len
```
应该是 `data.len()`。`clone` 不犯法但常多余。

### ❌ 把 `String` 当函数参数

```rust
fn greet(name: String) {  ... }  // 调用方被迫给所有权
```
除非你真要消费，否则 `&str`：
```rust
fn greet(name: &str) { ... }
```

## 错误处理

### ❌ `Result<T, Box<dyn Error>>` 在库里

库给具体错误类型（`thiserror` 帮你定义）。`Box<dyn Error>` / `anyhow::Error` 留给应用层。

### ❌ 把所有错误吃掉

```rust
let _ = file.write_all(&buf);   // 默默忽略
```
真的不在乎也要明示（注释 + `let _ = ...`），通常应该 `?`。

### ❌ 用 panic 做控制流

panic 不是异常。它是"程序逻辑错误"。流程控制用 `Result` / `Option`。

## API 设计

### ❌ 暴露内部容器类型

```rust
pub fn items(&self) -> &Vec<Item> { ... }     // 把 Vec 暴露出去
```
✅ 返回切片：
```rust
pub fn items(&self) -> &[Item] { ... }
```
这样你将来换 SmallVec 也不破坏 API。

### ❌ `pub` 滥用

默认 `pub(crate)`（同 crate 可见）。真要给外部用才 `pub`。

### ❌ 方法名 `get_xxx`

Rust 风格用 `xxx()` 不带 get：
```rust
fn name(&self) -> &str          // ✅
fn get_name(&self) -> &str      // ❌（Java/C++ 习惯）
```

## 并发

### ❌ `Mutex<Vec<T>>` 围住整个集合

如果只是 push，channel + 单一消费者更香（`std::sync::mpsc` / `crossbeam-channel`）。

### ❌ 持锁过久（含 `await` 持锁）

```rust
let g = mu.lock().unwrap();
some_async().await;        // ← 整个 await 期间锁还拿着 → 死锁高发
```
✅ `let v = mu.lock().unwrap().get_something(); drop(g); some_async().await;`
或：用 `tokio::sync::Mutex`，它跨 await 是合法的，但还是尽量短。

### ❌ tokio 任务里跑阻塞 CPU 活

会阻塞调度器。用 `tokio::task::spawn_blocking`。

## Cargo / 依赖

### ❌ `extern crate xxx;`（2015 风格）

2018+ 直接 `use xxx::...` 即可。

### ❌ 不锁版本：`xxx = "*"`

至少 `"1"` / `"1.2"`。

### ❌ 把 dev 工具放 dependencies

测试库（`criterion`、`mockall`）放 `[dev-dependencies]`，不污染生产。

## 杂

### ❌ 大量 `as` 类型转换

```rust
let x: u32 = (some_i64) as u32;   // 静默截断
```
✅ 用 `try_from`：
```rust
let x: u32 = u32::try_from(some_i64)?;
```

### ❌ 自定义 `fmt::Display` 时 `println!`

会爆栈。`Display::fmt` 里只能写到 `f: &mut Formatter`。

### ❌ struct 字段全 `pub`（除非真的是 POD）

破坏不变量。提供构造函数 + getter。
