以下是 Rust 中 `map` 方法的具体示例及详细解析。`map` 是迭代器的核心适配器方法，用于对每个元素进行转换，生成新的迭代器。它会保留原始迭代器的结构，但通过闭包对每个元素进行操作。

---

### **基础用法示例**

#### 示例1：数值转换

将数字转换为平方：

```rust
let nums = vec![1, 2, 3];
let squares: Vec<i32> = nums.iter().map(|x| x * x).collect();
assert_eq!(squares, vec![1, 4, 9]);
```

- `nums.iter()` 创建不可变引用的迭代器。
- `map(|x| x * x)` 对每个元素计算平方。
- `collect()` 将结果收集到 `Vec<i32>` 中。

---

#### 示例2：字符串处理

将字符串转换为大写：

```rust
let words = vec!["hello", "world"];
let uppercase: Vec<String> = words
    .iter()
    .map(|s| s.to_uppercase())
    .collect();
assert_eq!(uppercase, vec!["HELLO", "WORLD"]);
```

---

### **进阶用法示例**

#### 示例3：处理结构体

转换结构体字段：

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

let users = vec![
    User { name: "Alice".into(), age: 30 },
    User { name: "Bob".into(), age: 25 },
];

// 提取所有用户名
let names: Vec<String> = users.iter().map(|u| u.name.clone()).collect();
assert_eq!(names, vec!["Alice", "Bob"]);
```

---

#### 示例4：链式调用

结合 `filter` 过滤后转换：

```rust
let nums = vec![1, 2, 3, 4, 5];
let even_squares: Vec<i32> = nums
    .iter()
    .filter(|x| x % 2 == 0)  // 保留偶数 [2, 4]
    .map(|x| x * x)          // 计算平方 [4, 16]
    .collect();
assert_eq!(even_squares, vec![4, 16]);
```

---

### **复杂场景示例**

#### 示例5：处理 `Option` 或 `Result`

转换 `Option` 内的值：

```rust
let maybe_numbers = vec![Some(1), None, Some(3)];
let incremented: Vec<Option<i32>> = maybe_numbers
    .iter()
    .map(|opt| opt.map(|x| x + 1)) // 对 Some 内部的值加1
    .collect();
assert_eq!(incremented, vec![Some(2), None, Some(4)]);
```

---

#### 示例6：闭包捕获外部变量

使用外部变量参与计算：

```rust
let factor = 3;
let nums = vec![1, 2, 3];
let scaled: Vec<i32> = nums
    .iter()
    .map(|x| x * factor)  // 闭包捕获外部变量 `factor`
    .collect();
assert_eq!(scaled, vec![3, 6, 9]);
```

---

### **所有权与可变性**

#### 示例7：转移所有权

通过 `into_iter()` 消费原始数据：

```rust
let strings = vec!["a".to_string(), "b".to_string()];
let lengths: Vec<usize> = strings
    .into_iter()         // 转移所有权
    .map(|s| s.len())    // s 是 String 类型（所有权已转移）
    .collect();
// strings 不再可用
```

---

#### 示例8：修改元素（需要可变性）

通过 `iter_mut()` 修改元素：

```rust
let mut nums = vec![1, 2, 3];
nums.iter_mut()
    .map(|x| *x += 1)    // 修改每个元素（x 是 &mut i32）
    .for_each(|_| {});   // 触发迭代（map 返回 ()，需消费）
assert_eq!(nums, vec![2, 3, 4]);
```

- 注意：`map` 此处仅用于副作用，通常更推荐直接使用 `for_each`。

---

### **与其他方法的对比**

#### `map` vs `for_each`

- **`map`**：转换数据并返回新迭代器。
  ```rust
  let nums = vec![1, 2, 3];
  let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
  ```
- **`for_each`**：执行副作用操作（如打印），无返回值。
  ```rust
  nums.iter().for_each(|x| println!("{}", x));
  ```

---

### **总结**

| **场景**     | **示例**               | **关键点**                    |
| ------------ | ---------------------- | ----------------------------- | -------- | ---------------- |
| 简单数值转换 | `map(                  | x                             | x \* x)` | 闭包直接操作元素 |
| 处理复杂类型 | 结构体字段提取         | 结合 `clone()` 避免所有权问题 |
| 链式调用     | `filter` + `map`       | 惰性求值，优化性能            |
| 错误处理     | 转换 `Option`/`Result` | 嵌套 `map` 处理内部值         |
| 所有权控制   | `into_iter()` + `map`  | 明确转移所有权场景            |

掌握 `map` 方法能显著提升 Rust 代码的简洁性和可读性，尤其在与迭代器链式调用结合时，可以高效处理集合数据。
