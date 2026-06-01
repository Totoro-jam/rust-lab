# Rust 标准库导航

## 集合 `std::collections`

| 你想要 | 用 |
|--------|-----|
| 动态数组 | `Vec<T>` |
| 切片视图（不持有） | `&[T]` |
| 字符串 | `String` 持有 / `&str` 视图 |
| 哈希键值 | `HashMap<K, V>` |
| 有序键值 | `BTreeMap<K, V>` |
| 哈希集合 | `HashSet<T>` |
| 有序集合 | `BTreeSet<T>` |
| 队列 | `VecDeque<T>` |
| 链表 | `LinkedList<T>`（几乎不用，Vec 通常更好） |
| 优先队列 | `BinaryHeap<T>` |

## 字符串

- `String` ≠ Vec<u8>：保证有效 UTF-8
- `&str` 是不可变切片，常量是 `&'static str`
- `b"hello"` 是 `&[u8; 5]`，原始字节
- 拼接：`s.push_str(...)` / `format!("{}{}", a, b)`
- 切片要按字符边界，否则 panic：`s[..n]` 危险，安全的用 `s.chars().take(n)` 或 `s.get(..n)`

## 错误处理

- `Option<T>`：可能没有值
- `Result<T, E>`：可能失败
- `?` 运算符：早返回 + 自动 `From::from(e)` 转换错误
- `panic!`：仅用于不可恢复的程序 bug（断言、调试）
- 库代码用 `Result`，应用代码加 `anyhow`（错误聚合）或 `eyre`

## 迭代器

`Iterator` trait + 一长串组合子：

```rust
nums.iter()
    .filter(|&&x| x > 0)
    .map(|x| x * x)
    .take(10)
    .sum::<i64>();
```

终端方法（消耗迭代器）：`collect` / `sum` / `count` / `for_each` / `fold` / `reduce` / `any` / `all`

懒求值 → 链式不分配中间集合。

## I/O `std::io` / `std::fs`

- `std::fs::read_to_string(path)` 一把梭
- 流式：`File::open` + `BufReader::new(file).lines()`
- 写：`BufWriter::new(File::create(path))`
- `Read` / `Write` / `BufRead` / `Seek` trait 体系
- 控制台：`println!` / `eprintln!` / `print!` / `read_line`

## 并发 `std::thread` / `std::sync`

- `std::thread::spawn(|| { ... })`
- `Mutex<T>` / `RwLock<T>`：阻塞锁
- `Arc<T>`：原子引用计数（跨线程共享只读 / 配 Mutex 共享可变）
- `mpsc::channel`：多生产者单消费者（基础 channel）
- `std::sync::OnceLock<T>` / `LazyLock`：惰性单次初始化
- 想要更多并发原语 → 用 `crossbeam` / `tokio::sync` 替代

## 异步 `std::future`

- `async fn` 返回 `impl Future<Output = T>`
- `await` 暂停当前任务
- 标准库不带 runtime → 用 `tokio` 或 `async-std`

## 智能指针 `std::rc` / `std::sync` / `std::cell` / `std::pin`

| 用途 | 类型 |
|------|------|
| 堆分配单一所有权 | `Box<T>` |
| 单线程共享 | `Rc<T>` |
| 多线程共享 | `Arc<T>` |
| 单线程内部可变 | `Cell<T>`（Copy）/ `RefCell<T>`（运行期借用） |
| 多线程内部可变 | `Mutex<T>` / `RwLock<T>` |
| 弱引用 | `Weak<T>` |
| 固定地址 | `Pin<Box<T>>`（异步等场景） |

## 数值与时间

- `std::time::{Instant, Duration, SystemTime}`
- 大整数 / 高精度 → 用 `num-bigint` / `rust_decimal` crate

## 元编程

- 宏：`macro_rules!`（声明宏）+ `proc_macro`（过程宏，常见于 `#[derive(...)]`）
- 反射：标准库基本没有，靠 `serde` / `bevy_reflect` 之类的 crate 补
