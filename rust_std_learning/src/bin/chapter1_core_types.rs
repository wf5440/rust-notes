// chapter1_core_types.rs
fn main() {
    println!("=== Rust核心类型和特质 ===\n");
    
    // 1. Option<T> 示例
    println!("1. Option<T> 示例:");
    let some_number = Some(5);
    let none_number: Option<i32> = None;
    
    if let Some(n) = some_number {
        println!("  有值: {}", n);
    }
    
    match none_number {
        Some(n) => println!("  有值: {}", n),
        None => println!("  没有值"),
    }
    
    // 2. Result<T, E> 示例
    println!("\n2. Result<T, E> 示例:");
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出错了!");
    
    match success {
        Ok(value) => println!("  成功: {}", value),
        Err(e) => println!("  失败: {}", e),
    }
    
    // 3. Vec<T> 和迭代器
    println!("\n3. Vec<T> 和迭代器示例:");
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  原数组: {:?}", numbers);
    println!("  加倍后: {:?}", doubled);
    
    // 4. String 操作
    println!("\n4. String 操作示例:");
    let mut s = String::from("Hello");
    s.push_str(" Rust!");
    println!("  字符串: {}", s);
    
    // 5. 特质示例
    println!("\n5. 特质示例:");
    #[derive(Debug, Clone)] // 使用Debug和Clone特质
    struct Person {
        name: String,
        age: u8,
    }
    
    let person = Person { name: "Alice".to_string(), age: 30 };
    let person_clone = person.clone();
    println!("  人物: {:?}", person_clone);
}