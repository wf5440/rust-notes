// chapter4_error_handling.rs
use std::fs::File;
use std::io::{self, Read, Write};

// 自定义错误类型
#[derive(Debug)]
enum MyError {
    IoError(io::Error),
    ParseError,
    Custom(String),
}

// 文件读取函数
fn read_file_contents(path: &str) -> Result<String, MyError> {
    let mut file = File::open(path)
        .map_err(|e| MyError::IoError(e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| MyError::IoError(e))?;
    
    Ok(contents)
}

// 解析数字函数
fn parse_number(s: &str) -> Result<i32, MyError> {
    s.parse().map_err(|_| MyError::ParseError)
}

fn main() {
    println!("=== 错误处理 ===\n");
    
    // 1. Result 基础用法
    println!("1. Result 基础用法:");
    let result: Result<i32, &str> = Ok(42);
    let error_result: Result<i32, &str> = Err("出错了");
    
    match result {
        Ok(value) => println!("  成功: {}", value),
        Err(e) => println!("  失败: {}", e),
    }
    
    // 2. ? 操作符
    println!("\n2. ? 操作符示例:");
    fn process_file() -> Result<(), MyError> {
        let contents = read_file_contents("test.txt")?;
        println!("  文件内容: {}", contents);
        Ok(())
    }
    
    if let Err(e) = process_file() {
        println!("  文件处理错误: {:?}", e);
    }
    
    // 3. unwrap 和 expect
    println!("\n3. unwrap 和 expect:");
    let ok_value = Ok(100).unwrap();
    println!("  unwrap成功值: {}", ok_value);
    
    // let bad_value = Err("错误").unwrap(); // 这会panic!
    
    // 4. 组合错误处理
    println!("\n4. 组合错误处理:");
    fn process_data(data: &str) -> Result<i32, MyError> {
        let num = parse_number(data)?;
        if num < 0 {
            return Err(MyError::Custom("数字不能为负".to_string()));
        }
        Ok(num * 2)
    }
    
    match process_data("42") {
        Ok(result) => println!("  处理成功: {}", result),
        Err(e) => println!("  处理失败: {:?}", e),
    }
    
    match process_data("-5") {
        Ok(result) => println!("  处理成功: {}", result),
        Err(e) => println!("  处理失败: {:?}", e),
    }
    
    // 5. 创建测试文件并读取
    println!("\n5. 文件操作错误处理:");
    let _ = File::create("test.txt").and_then(|mut file| {
        file.write_all(b"Hello Rust!")
    });
    
    match read_file_contents("test.txt") {
        Ok(contents) => println!("  读取成功: {}", contents),
        Err(e) => println!("  读取失败: {:?}", e),
    }
    
    // 清理
    let _ = std::fs::remove_file("test.txt");
}