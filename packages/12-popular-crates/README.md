# 12 - 流行 crate 速查

> 把"写 Rust 时常用的 crate"按场景列一遍。每条只给定位 + 一句话场景，不展开 API。
> 真正学某个 crate 时，建议参考它在 docs.rs 的页面 + 项目 examples/。

## 前置

- 前 11 章基本走完，知道自己缺哪类工具

## 选 crate 的原则

- 优先看 **下载量 / 最近发版 / issue 活跃度**（lib.rs / crates.io 都能看）
- 维护者是否换过手、是否在 `1.x` 稳定线
- 是否兼容你的 MSRV（最低支持 Rust 版本）
- 别陷入"小工具吸引症"——多数项目 10 个 crate 内能解决

## 序列化 / 数据格式

- `serde` + `serde_json`：JSON 事实标准
- `serde_yaml` / `toml` / `ron`：YAML / TOML / RON
- `bincode` / `rmp-serde` / `ciborium`：二进制 / MessagePack / CBOR
- `prost` / `tonic`：Protobuf / gRPC

## 错误处理

- `anyhow`：应用层"任意错误"，带 context 链
- `thiserror`：库层定义错误 enum
- `color-eyre` / `eyre`：anyhow 增强版，彩色 backtrace

## 异步 runtime / IO

- `tokio`：事实标准 runtime（多线程 + IO）
- `async-std`：另一选择，API 更贴 std
- `smol`：极简 runtime
- `futures` / `futures-util`：异步组合子
- `async-trait`：trait 里 async fn（新版 Rust 已部分原生支持）

## HTTP / 网络

- `axum`：基于 tower 的 web 框架（推荐）
- `actix-web`：高性能 actor 风格 web 框架
- `reqwest`：HTTP 客户端
- `hyper`：底层 HTTP 实现
- `tower` / `tower-http`：可组合中间件
- `tonic`：gRPC

## 数据库

- `sqlx`：异步 + 编译期 SQL 检查（Postgres/MySQL/SQLite）
- `sea-orm` / `diesel`：ORM
- `rusqlite`：同步 SQLite 绑定
- `redis`：Redis 客户端
- `mongodb`：官方 MongoDB driver

## CLI

- `clap`：参数解析（derive 风格首选）
- `argh`：极简参数解析
- `indicatif`：进度条 / spinner
- `dialoguer`：交互式提示
- `console` / `colored` / `owo-colors`：彩色输出
- `comfy-table` / `tabled`：终端表格

## 日志 / 可观测

- `tracing` + `tracing-subscriber`：结构化日志 / span（推荐）
- `log` + `env_logger`：经典日志门面
- `tracing-opentelemetry`：导出到 OTLP
- `metrics` / `prometheus`：指标

## 并发 / 并行

- `rayon`：数据并行（迭代器 .par_iter()）
- `crossbeam`：高级 channel / scope
- `parking_lot`：更快的 Mutex/RwLock
- `dashmap`：并发 HashMap

## 配置 / 环境

- `config`：分层配置（YAML/TOML/JSON/env）
- `figment`：另一选择
- `dotenvy`：加载 .env
- `directories-next`：跨平台 XDG 路径

## 时间 / ID

- `chrono` / `time`：日期时间
- `uuid`：UUID 生成
- `ulid`：可排序 ID
- `nanoid`：短 ID

## 加解密 / 哈希

- `ring` / `rustls`：TLS + 加密原语
- `sha2` / `blake3`：哈希
- `argon2` / `bcrypt`：密码哈希
- `jsonwebtoken`：JWT

## 解析 / 文本

- `regex`：正则
- `nom` / `winnow`：parser combinator
- `pest`：PEG 解析器
- `pulldown-cmark`：Markdown 解析
- `html5ever` / `scraper`：HTML

## 数值 / 科学计算

- `num` / `num-traits`：数值 trait
- `ndarray`：N 维数组（类 NumPy）
- `nalgebra`：线性代数
- `polars`：DataFrame（对标 pandas，性能强）
- `plotters`：绘图

## 测试 / 模拟

- `proptest` / `quickcheck`：property-based
- `rstest`：参数化 + fixture
- `mockall`：trait mock
- `criterion`：基准测试
- `insta`：快照测试
- `assert_cmd` + `predicates`：CLI 端到端测
- `wiremock`：mock HTTP 服务

## 工具 / 杂项

- `itertools`：补全标准库 Iterator
- `once_cell` / `lazy_static`（旧）：全局只初始化一次
- `bytes`：高效字节缓冲
- `bytemuck` / `zerocopy`：零拷贝转换
- `tempfile`：临时文件 / 目录
- `walkdir`：递归遍历
- `ignore` / `globset`：路径过滤（ripgrep 同款）
- `notify`：文件系统事件

## 嵌入 / FFI / WASM

- `bindgen`：从 C 头自动生成 FFI
- `cbindgen`：从 Rust 生成 C 头
- `pyo3`：Rust ↔ Python
- `napi-rs`：Rust ↔ Node.js
- `wasm-bindgen` / `wasmer` / `wasmtime`：WASM

## 模板 / 渲染

- `tera`：Jinja2 风格模板
- `askama`:: 编译期模板
- `handlebars`

## "工具链" 类

- `cargo-watch`：文件变化自动重跑 cargo
- `cargo-nextest`：更快的 test runner
- `cargo-edit`：`cargo add / rm / upgrade`（新版 cargo 已内置）
- `cargo-outdated` / `cargo-audit` / `cargo-deny`：依赖审计
- `cargo-llvm-cov`：覆盖率
- `cargo-expand`：展开宏看生成的代码

## 计划要写

- `src/`: 拼一个"全家桶"小项目示例，体现常见组合（axum + sqlx + tracing + clap + anyhow）
- `examples/`: 每个分类挑一个最小 demo
- `LINKS.md`: 各 crate 官方文档 / 仓库链接整理

## 自测

- 同一件事 `tokio` 和 `async-std` 都能干，怎么选？
- `anyhow` vs `thiserror` 在库 / 应用各自的角色？
- 想要"DataFrame"应该选 `polars` 还是别的？
- `rayon` 和 `tokio` 有什么本质区别？
- 一个 web 服务想要 metrics + tracing + 日志，最少要引哪几个 crate？

---

**TODO**: 把每个分类挑一两个 crate 写最小 runnable 示例。
