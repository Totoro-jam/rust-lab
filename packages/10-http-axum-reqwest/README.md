# 10 - HTTP: axum (server) + reqwest (client)

> 当今写 Rust 服务端最流行组合：**axum** 路由 + **tower** 中间件 + **tokio** runtime；调外部 API 用 **reqwest**。

## 前置

- [08-async-tokio](../08-async-tokio)
- [09-serde-json](../09-serde-json)

## 本章目标

### reqwest（客户端）
- 简单 GET：`reqwest::get(url).await?.text().await?`
- builder：`Client::new().get(url).header(...).json(&body).send().await?`
- JSON：`.json::<T>().await?` 反序列化
- 表单 / multipart
- timeout / 重试（用 `tower::retry` 或 `reqwest-middleware`）
- 异步 vs 阻塞客户端（`reqwest::blocking`）
- 复用 `Client`（连接池）

### axum（服务端）
- 路由 `Router::new().route("/path", get(handler))`
- 提取器（Extractor）：`Path<T>` / `Query<T>` / `Json<T>` / `State<S>` / `Extension<T>`
- 响应：`impl IntoResponse`，`Json`、`StatusCode`、`(StatusCode, body)` tuple
- 共享状态 `Router::with_state(...)`
- 中间件 `Layer`（来自 `tower`）：日志、超时、CORS、压缩
- 错误处理：自定义 `IntoResponse` for 错误 enum
- WebSocket：`WebSocketUpgrade`
- 静态文件：`tower-http::services::ServeDir`

### tower 生态
- Service trait 的力量：`Service<Request, Response = Response, Error = Infallible>`
- 中间件可组合：`ServiceBuilder::new().timeout(...).rate_limit(...)`

## 推荐 crate

- `axum`、`reqwest`、`tower`、`tower-http`、`tracing`
- `serde_json` 几乎必带
- 全功能 web 框架替代：`actix-web`（运行模型不同，单 Service trait）

## 计划要写

- `src/`: 完整的 GET/POST/PUT/DELETE 服务（in-memory 资源），含错误响应
- `src/`: reqwest 客户端调上面的服务，做"端到端"演示
- `tests/`: 用 `axum::Router::oneshot` 做集成测试，无需起真服务

## 自测

- axum 的 handler 函数签名是怎么"自动接受"参数的？(Trait 魔法)
- axum 和 actix-web 选哪个？
- reqwest 的 `Client` 为啥要复用？
- tower 中间件相比 express middleware 的优势？

---

**TODO**: 待补充完整代码与示例。
