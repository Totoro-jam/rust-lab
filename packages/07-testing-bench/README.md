# 07 - Testing & Benchmarking

> 01 章已经写了 `#[test]`，这章把测试工具链补齐：表格驱动、property、文档测试、基准测试、coverage。

## 前置

- [01-fundamentals](../01-fundamentals)
- [03-error-handling](../03-error-handling)

## 本章目标

### 单元 & 集成测试
- `#[cfg(test)] mod tests` 模块内私有项测试
- `tests/` 目录的集成测试（黑盒）
- `assert!` / `assert_eq!` / `assert_ne!` / `assert_matches!`（nightly 或 `assert_matches` crate）
- 期望 panic：`#[should_panic(expected = "...")]`
- 测返回 `Result`：`#[test] fn foo() -> Result<(), MyErr> { ... }`
- `#[ignore]` 标记慢测试，`cargo test -- --ignored` 才跑

### Doc tests
- 文档注释里的 `/// ```rust\n... \n```` ` 会被当测试跑
- `cargo test --doc`

### Property-based testing
- `proptest`：生成随机输入找反例
  ```rust
  proptest! {
      #[test]
      fn add_commutative(a in 0i64..1000, b in 0i64..1000) {
          assert_eq!(add(a, b), add(b, a));
      }
  }
  ```
- `quickcheck`：另一选择

### 参数化测试
- `rstest`：fixture + parametrize
  ```rust
  #[rstest]
  #[case(1, 2, 3)]
  #[case(-1, 1, 0)]
  fn add_cases(#[case] a: i64, #[case] b: i64, #[case] want: i64) { ... }
  ```

### Mock
- `mockall` 通过过程宏给 trait 生成 mock 实现

### Benchmark
- 稳定版：`criterion` crate
  ```rust
  fn bench_add(c: &mut Criterion) {
      c.bench_function("add", |b| b.iter(|| add(black_box(1), black_box(2))));
  }
  ```
- nightly：`#[bench]` 内建（不推荐普通项目）

### Coverage
- `cargo-llvm-cov`：`cargo install cargo-llvm-cov && cargo llvm-cov`

## 推荐 crate

- `proptest`、`quickcheck`、`rstest`、`mockall`、`criterion`
- 工具：`cargo-llvm-cov`、`cargo-nextest`（更快的测试 runner）

## 计划要写

- 重写 01 章测试：加 proptest、rstest、doctest 各一例
- benches/：基准 add / divide
- coverage 脚本

## 自测

- 单元测试和集成测试访问私有项的区别？
- doctest 失败会让 `cargo test` 失败吗？
- proptest 找到反例后能保留作为回归测试吗？
- criterion 用 `black_box` 是为啥？

---

**TODO**: 待补充完整代码与测试。
