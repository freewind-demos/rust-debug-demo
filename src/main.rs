fn main() {
    println!("=== Rust Debug 演示 ===\n");

    // 1. Debug trait - 格式化调试输出
    println!("--- Debug trait ---");
    let x = 42;
    println!("Debug: {:?}", x);

    let s = String::from("hello");
    println!("String: {:?}", s);

    // 2. derive Debug
    println!("\n--- derive Debug ---");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);
    println!("Point 格式化: {:#?}", p);

    // 3. 嵌套结构
    println!("\n--- 嵌套结构 ---");
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let rect = Rectangle {
        top_left: Point { x: 1, y: 2 },
        bottom_right: Point { x: 10, y: 20 },
    };
    println!("Rectangle: {:#?}", rect);

    // 4. 枚举的 Debug
    println!("\n--- 枚举 Debug ---");
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }

    let msgs = vec![
        Message::Quit,
        Message::Move { x: 5, y: 10 },
        Message::Write(String::from("hello")),
    ];

    for msg in &msgs {
        println!("{:?}", msg);
    }

    // 5. dbg! 宏
    println!("\n--- dbg! 宏 ---");
    let x = 5;
    let y = 10;
    dbg!(x + y);

    let arr = [1, 2, 3];
    dbg!(&arr);

    // 6. 自定义 Debug 实现
    println!("\n--- 自定义 Debug ---");
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl std::fmt::Debug for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Person")
                .field("name", &self.name)
                .field("age", &self.age)
                .finish()
        }
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person: {:?}", person);

    // 7. 调试格式化
    println!("\n--- 调试格式化 ---");
    let v = vec![1, 2, 3];
    println!("默认: {:?}", v);
    println!("漂亮: {:#?}", v);

    // 8. Display vs Debug
    println!("\n--- Display vs Debug ---");
    println!("Display: {}", 42);
    println!("Debug: {:?}", 42);

    println!("\n=== 总结 ===");
    println!("Debug trait 用于调试输出");
    println!("{:?} 使用 Debug 格式");
    println!("{:#?} 使用美化后的 Debug 格式");
    println!("dbg! 宏用于快速调试");
    println!("derive(Debug) 自动实现 Debug");
}
