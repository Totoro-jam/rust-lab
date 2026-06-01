# Rust vs C++ / Go / TypeScript

## Rust vs C++

| 维度 | Rust | C++ |
|------|------|-----|
| 内存安全 | 编译期保证（borrow checker） | 程序员保证 + sanitizer |
| 默认 | 移动、不可变、私有 | 拷贝、可变、公开（class private 是例外） |
| 错误处理 | `Result<T, E>` 强制处理 | exception / `expected` |
| 泛型 | trait bound → 错误消息友好 | 模板 → SFINAE 噩梦（concepts 后改善） |
| 并发 | Send/Sync 编译期防 race | 程序员防（sanitizer 帮忙） |
| 包管理 | cargo（统一） | CMake + vcpkg/Conan（割裂） |
| ABI | extern "C" 兼容；Rust ABI 不稳定 | 同样需要 extern "C"；C++ ABI 也不稳定 |
| 生态 | 年轻、增长快 | 巨大、历史悠久 |
| 编译速度 | 慢（borrow check + 单态化） | 慢（模板 + 头文件） |
| 学习曲线 | 高（borrow checker 抗拒期） | 高（特性多 + UB 多） |

> **选 Rust**：新系统项目、希望编译器替你管内存安全、需要无 GC 但要高安全。
> **选 C++**：已有大量 C++ 代码 / 团队、需要某些 C++ 才有的库（Qt、Unreal）、追求极致优化空间。

## Rust vs Go

| 维度 | Rust | Go |
|------|------|------|
| 内存 | 无 GC | GC（低延迟） |
| 并发 | async + Send/Sync 编译期 | goroutine + channel + race detector 运行期 |
| 类型系统 | 强（trait、enum、generics） | 简单（interface 鸭子，generics 较新） |
| 错误处理 | `Result + ?` | `if err != nil { return err }` 重复 |
| 编译速度 | 慢 | 快 |
| 学习曲线 | 高 | 低 |
| 运行时 | 几乎没有 | 带 GC + 调度器 |
| 适合 | 系统、CLI、嵌入式、网络服务、WASM | 网络服务、CLI、云基础设施 |

> **选 Rust**：性能与延迟敏感、需要 FFI 给其他语言用、想要更强类型保证。
> **选 Go**：团队规模大、追求简单和上手快、不在意 GC 暂停。

## Rust vs TypeScript

| 维度 | Rust | TypeScript |
|------|------|------------|
| 类型 | 名义型 + 严格 | 结构型 + 渐进式 |
| 运行时 | 编译为机器码 | 运行在 JS 引擎 |
| async | 需要 runtime（tokio） | event loop 内建 |
| 异常 | 没有 try/catch；用 Result | try/catch |
| 包管理 | cargo / crates.io | npm/pnpm/yarn |
| 模块 | mod + crate | ESM / CommonJS |
| 性能 | 接近 C | V8 JIT，够快 |
| 适合 | 后端服务、CLI、引擎 | 前端、Node 服务、脚本 |

## 一句话

> C++ 给你绳子加复杂的打结教程；Rust 让编译器替你打结、不通过不让你跑；Go 给你一根标准长度的绳子和大字"别打结"的提示；TypeScript 给你一根 JS 绳子加塑料外壳。
