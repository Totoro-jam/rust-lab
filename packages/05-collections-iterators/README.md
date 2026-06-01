# 05 - Collections & Iterators

> Vec / HashMap 用法 + 迭代器链式。"不要写 for 循环"在 Rust 里也成立。

## 前置

- [02-ownership-borrowing](../02-ownership-borrowing)
- [04-traits-generics](../04-traits-generics)

## 本章目标

### 集合
- `Vec<T>`：构造（`vec![]` / `Vec::with_capacity`）、`push` / `pop` / `extend` / `iter` / `iter_mut` / `into_iter`
- `&[T]` 切片：函数参数通常用它
- `HashMap<K, V>`：`insert` / `get` / `entry().or_insert_with(...)` / `remove`
- `BTreeMap<K, V>`：有序，按 key 迭代
- `HashSet<T>` / `BTreeSet<T>`
- `VecDeque<T>` / `BinaryHeap<T>`
- 字符串集合：`String` / `&str` / `Cow<'_, str>`

### 迭代器
- 三种迭代：`iter()`（`&T`）、`iter_mut()`（`&mut T`）、`into_iter()`（`T`）
- 适配器（懒）：`map` / `filter` / `take` / `skip` / `take_while` / `chain` / `zip` / `enumerate` / `flat_map` / `inspect`
- 消费者：`collect` / `sum` / `product` / `count` / `for_each` / `fold` / `reduce` / `any` / `all` / `max` / `min`
- `collect::<Vec<_>>()` / `collect::<HashMap<_,_>>()` / `collect::<Result<Vec<_>, _>>()`
- `Iterator::peekable` / `windows` / `chunks`（slice 的）
- 自定义迭代器：实现 `Iterator` trait

## 推荐工具

- `cargo bench` 后续比较 for 循环 vs 迭代器链
- itertools crate（更多组合子）

## 计划要写

- `src/`: 词频统计三种写法（手写、algorithm 链、entry API）
- `src/`: 自定义迭代器：斐波那契无限序列
- `tests/`: 各种 `collect` 形态

## 自测

- `v.iter()` / `v.iter_mut()` / `v.into_iter()` 区别？
- `for x in &v` 等价于哪种 iter？
- `collect::<Result<Vec<_>, _>>()` 干啥用？
- `HashMap::entry(k).or_insert_with(|| ...)` 比 `if !contains_key { insert }` 好在哪？

---

**TODO**: 待补充完整代码与示例。
