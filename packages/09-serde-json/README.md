# 09 - Serde + JSON / YAML / TOML

> `serde` 是 Rust 序列化的事实标准。一个 `#[derive(Serialize, Deserialize)]` 抹平 N 种格式。

## 前置

- [04-traits-generics](../04-traits-generics)
- [03-error-handling](../03-error-handling)

## 本章目标

### Serde 基础
- `#[derive(Serialize, Deserialize)]` 给 struct / enum 自动派生
- `serde_json::{to_string, from_str, Value}`
- `Value`（"任意 JSON"）vs 强类型 struct，何时选哪个
- 字段重命名 `#[serde(rename = "id")]` / 全局规则 `#[serde(rename_all = "camelCase")]`
- 跳过 / 默认 / 可选：
  - `#[serde(default)]`
  - `#[serde(skip_serializing_if = "Option::is_none")]`
  - `#[serde(skip)]`
- enum 表达：`untagged` / `tagged` / `adjacently tagged`
- 自定义 `serialize_with` / `deserialize_with`
- `#[serde(flatten)]` 展开嵌套字段
- 错误信息：`serde_json::Error`

### 其它格式
- YAML：`serde_yaml`
- TOML：`toml` crate（cargo 配置自己也是 TOML）
- BSON / MessagePack / CBOR：同样的派生，换序列化器
- bincode：紧凑二进制

### 性能
- `serde_json` 通用快
- `simd-json`：极致性能
- `sonic-rs`：来自字节跳动的快速实现
- 大流式 JSON：`serde_json::Deserializer::from_reader(...).into_iter::<T>()`

## 推荐 crate

- `serde` + `serde_json`（必装）
- `serde_yaml` / `toml`
- `bincode` / `rmp-serde`(MessagePack)

## 计划要写

- `src/`: 一份"配置文件 + API 响应"两种 JSON 解析
- `src/`: 用 `#[serde(tag = "type")]` 解析多态 JSON
- `src/`: 流式解析大 JSON 文件
- `tests/`: 边界（缺字段、多字段、null、空数组）

## 自测

- `Option<T>` 字段缺失和值为 null 怎么区分？
- `untagged` enum 解析多个变体都匹配怎么办？
- 一个字段叫 `type`（Rust 关键字）怎么序列化？
- `flatten` 何时有用？

---

**TODO**: 待补充完整代码与示例。
