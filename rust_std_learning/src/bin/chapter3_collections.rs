// chapter3_collections.rs
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};

fn main() {
    println!("=== 集合类型 ===\n");
    
    // 1. Vec 动态数组
    println!("1. Vec 动态数组:");
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("  Vec: {:?}", vec);
    println!("  第一个元素: {:?}", vec.get(0));
    
    // 2. String
    println!("\n2. String 操作:");
    let mut s = String::from("Hello");
    s.push_str(" World!");
    println!("  字符串: {}", s);
    println!("  字符数: {}", s.chars().count());
    
    // 3. HashMap
    println!("\n3. HashMap 示例:");
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);
    
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    
    // 4. HashSet
    println!("\n4. HashSet 示例:");
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(1); // 重复元素不会被插入
    
    println!("  HashSet: {:?}", set);
    println!("  是否包含2: {}", set.contains(&2));
    
    // 5. VecDeque 双端队列
    println!("\n5. VecDeque 双端队列:");
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    
    println!("  VecDeque: {:?}", deque);
    println!("  弹出前端: {:?}", deque.pop_front());
    
    // 6. BinaryHeap 二叉堆（最大堆）
    println!("\n6. BinaryHeap 示例:");
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(5);
    heap.push(2);
    
    println!("  最大元素: {:?}", heap.peek());
    while let Some(val) = heap.pop() {
        println!("  弹出: {}", val);
    }
}