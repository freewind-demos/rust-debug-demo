# Rust Debug Demo

## 简介

演示 Rust Debug trait。

## 基本原理

Debug trait 用于调试输出，使用 {:?} 格式化。

## 启动和使用

```bash
cargo run
```

## 教程

### derive Debug

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }
println!("{:?}", point);
```

### 美化输出

```rust
println!("{:#?}", point);
```

### dbg! 宏

```rust
let x = 5;
dbg!(x + 10);
```

### 自定义 Debug

```rust
impl Debug for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .finish()
    }
}
```
