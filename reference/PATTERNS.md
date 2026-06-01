# Rust 常用模式

## 错误传播：`?` + `From`

```rust
fn read_count(path: &Path) -> Result<usize, MyError> {
    let s = std::fs::read_to_string(path)?;        // io::Error -> MyError
    let n = s.trim().parse::<usize>()?;            // ParseIntError -> MyError
    Ok(n)
}
```

`?` 在 `Err` 时立即返回；类型差异由 `From` impl 自动转换——`thiserror` 帮你写。

## "新类型"模式

包一个 tuple struct 区分语义：

```rust
struct UserId(u64);
struct OrderId(u64);

fn lookup(u: UserId) { ... }   // 编译期防止把 OrderId 当 UserId 传
```

零运行期开销，类型安全直接送到飞起。

## "构造时校验"

把构造函数包成 `try_new`：

```rust
pub struct Email(String);

impl Email {
    pub fn new(s: impl Into<String>) -> Result<Self, EmailError> {
        let s = s.into();
        if !s.contains('@') { return Err(EmailError::Invalid); }
        Ok(Self(s))
    }
}
```

后续代码拿到 `Email` 就**保证有效**，不必再判。

## RAII 守卫 / Drop

```rust
struct LockGuard<'a> { mu: &'a Mutex<i32>, val: ... }
impl Drop for LockGuard<'_> {
    fn drop(&mut self) { /* 释放 */ }
}
```

`Mutex::lock` 的 `MutexGuard` 就是这套——离开作用域自动 unlock，没有忘 unlock 的可能。

## Iterator 改 for 循环

```rust
// 不要：
let mut out = Vec::new();
for x in &v { if *x > 0 { out.push(x * x); } }

// 要：
let out: Vec<_> = v.iter().filter(|&&x| x > 0).map(|x| x * x).collect();
```

链式表达意图更清晰；零成本抽象，性能不输手写。

## `match` 优于 `if-else`

```rust
match status {
    Status::Ok => println!("ok"),
    Status::Err(msg) if msg.is_empty() => println!("unknown error"),
    Status::Err(msg) => println!("error: {msg}"),
}
```

- 编译器强制穷尽 → 加 enum 变体时所有 match 都会编译失败提醒你
- guard 子句 `if cond`
- 模式绑定 `Status::Err(msg)` 直接拿到内部值

## Builder 模式（链式构造）

```rust
let req = Request::builder()
    .method("POST")
    .url("...")
    .header("...", "...")
    .body(b)
    .build()?;
```

适合参数多、组合多变的 API。

## `From` / `Into` 双向转换

`impl From<A> for B` → 自动得到 `impl Into<B> for A`。函数签名用 `arg: impl Into<B>` 接受多种输入：

```rust
fn greet(name: impl Into<String>) { println!("hi, {}", name.into()); }
greet("alice");          // &str -> String
greet(String::from(...));
```

## 引用 vs 拷贝的选择

- 接受参数：能用 `&str` 就别用 `String`；能用 `&[T]` 就别用 `Vec<T>`
- 返回值：返 owned `String` / `Vec` 通常更省心，除非有清晰生命周期需求
- `Cow<'a, str>`：可能借用、可能拥有；解析/转义场景常用

## "状态机用 enum + match"

```rust
enum Conn {
    Disconnected,
    Connecting,
    Connected { since: Instant },
    Failed(io::Error),
}

fn tick(c: Conn) -> Conn {
    match c {
        Conn::Disconnected => Conn::Connecting,
        Conn::Connecting => Conn::Connected { since: Instant::now() },
        c => c,
    }
}
```

编译器保证你处理每个状态。

## "测试和实现放一起"

```rust
// 文件末尾：
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn it_works() { assert_eq!(add(2, 2), 4); }
}
```

不要为了测试把私有 API 改 public——`#[cfg(test)] mod tests` 能直接访问。
