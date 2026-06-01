# 08 - Async / await + Tokio

> Rust 的 async 是"语言定义协议，runtime 自选"。最常用的是 **tokio**。

## 前置

- [02-ownership-borrowing](../02-ownership-borrowing)
- [04-traits-generics](../04-traits-generics)

## 本章目标

### 基础概念
- `async fn` 返回 `impl Future<Output = T>` —— Future 是个状态机
- `.await` 让出当前任务
- runtime 负责调度（标准库不带 runtime）
- 协作式调度：不能阻塞，否则别人没法跑

### Tokio
- `#[tokio::main]` 主入口
- `tokio::spawn(async move { ... })` 起任务
- `tokio::join!` / `tokio::select!` 并发组合
- 超时：`tokio::time::timeout(dur, fut)`
- 周期：`tokio::time::interval(dur)`

### 异步 IO
- `tokio::fs` / `tokio::net::{TcpListener, TcpStream}`
- `AsyncReadExt` / `AsyncWriteExt` trait
- buffered IO：`BufReader::new(stream).lines()`

### 同步原语（async 版）
- `tokio::sync::Mutex` / `RwLock`：跨 `.await` 安全
- `tokio::sync::mpsc` / `oneshot` / `broadcast` / `watch` channel
- `tokio::sync::Notify`

### 任务结构
- detached task vs JoinHandle
- 取消（`abort()` / drop JoinHandle）
- 优雅关闭：`tokio::signal::ctrl_c` + select

### 性能 / 调试
- 多线程 runtime vs 单线程 runtime
- `tokio-console` 看实时任务状态
- `tracing` + `tracing-subscriber` 结构化日志

### 阻塞活
- CPU 密集 → `tokio::task::spawn_blocking`
- 调用同步库（不接受 async）→ 同上

## 推荐 crate

- `tokio`（必装）
- `tracing` / `tracing-subscriber`
- `async-trait`（trait 里写 async fn 的旧方案；MSRV 高时可用原生）
- `futures` / `futures-util`

## 计划要写

- `src/`: 异步 TCP echo server
- `src/`: 并发拉 10 个 URL，汇总
- `src/`: channel 实现 worker 池
- `tests/`: `#[tokio::test]` 异步测试

## 自测

- `async fn foo() {}` 和 `fn foo() -> impl Future<Output = ()>` 区别？
- 在 tokio 任务里调用 `std::thread::sleep` 会怎样？
- `tokio::join!` 和起两个 `tokio::spawn` 区别？
- 跨 `.await` 持锁为啥危险？

---

**TODO**: 待补充完整代码与示例。
