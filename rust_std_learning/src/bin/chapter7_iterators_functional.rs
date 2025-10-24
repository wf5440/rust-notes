// chapter7_iterators_functional.rs

fn main() {
    println!("=== 迭代器和函数式编程 ===\n");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 1. 基本迭代器操作
    println!("1. 基本迭代器操作:");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  加倍: {:?}", doubled);
    
    // 2. 过滤
    println!("\n2. 过滤操作:");
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  偶数: {:?}", evens);
    
    // 3. 折叠/归约
    println!("\n3. 折叠操作:");
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("  总和: {}", sum);
    
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("  乘积: {}", product);
    
    // 4. 链式操作
    println!("\n4. 链式操作:");
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x > 3)
        .map(|x| x * x)
        .filter(|&x| x < 50)
        .collect();
    println!("  链式结果: {:?}", result);
    
    // 5. 枚举迭代
    println!("\n5. 枚举迭代:");
    for (i, num) in numbers.iter().enumerate() {
        println!("  索引 {}: 值 {}", i, num);
    }
    
    // 6. 惰性求值
    println!("\n6. 惰性求值:");
    let lazy_numbers = numbers.iter()
        .map(|x| {
            println!("  处理: {}", x);
            x * 2
        });
    
    println!("  尚未执行计算...");
    
    let collected: Vec<i32> = lazy_numbers.collect();
    println!("  最终结果: {:?}", collected);
    
    // 7. 迭代器适配器
    println!("\n7. 迭代器适配器:");
    
    // take - 取前n个
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("  前三个: {:?}", first_three);
    
    // skip - 跳过前n个
    let after_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("  跳过前三个: {:?}", after_three);
    
    // zip - 合并两个迭代器
    let letters = vec!["a", "b", "c"];
    let zipped: Vec<(&i32, &&str)> = numbers.iter().zip(letters.iter()).collect();
    println!("  合并: {:?}", zipped);
    
    // 8. 收集到不同集合
    println!("\n8. 收集到不同集合:");
    
    // 收集到 HashMap
    use std::collections::HashMap;
    let map: HashMap<_, _> = numbers.iter()
        .map(|&x| (x, x * x))
        .collect();
    println!("  数字平方映射: {:?}", map);
    
    // 收集到 String
    let string: String = numbers.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("  数字字符串: {}", string);
    
    // 9. 自定义迭代器
    println!("\n9. 自定义迭代器:");
    
    struct Countdown {
        count: i32,
    }
    
    impl Countdown {
        fn new(start: i32) -> Self {
            Countdown { count: start }
        }
    }
    
    impl Iterator for Countdown {
        type Item = i32;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                let current = self.count;
                self.count -= 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    let countdown = Countdown::new(5);
    for num in countdown {
        println!("  倒计时: {}", num);
    }
}