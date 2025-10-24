// chapter5_concurrency.rs
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

fn main() {
    println!("=== 并发编程 ===\n");
    
    // 1. 基本线程
    println!("1. 基本线程示例:");
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  线程: 计数 {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..=2 {
        println!("  主线程: 计数 {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();
    
    // 2. 消息传递 (mpsc通道)
    println!("\n2. 消息传递示例:");
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec!["hello", "from", "the", "thread"];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::spawn(move || {
        let vals = vec!["more", "messages"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("  收到: {}", received);
    }
    
    // 3. 共享状态 (Mutex)
    println!("\n3. 互斥锁 Mutex 示例:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  最终计数: {}", *counter.lock().unwrap());
    
    // 4. Atomic 类型 (无锁编程)
    println!("\n4. 原子操作示例:");
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut atomic_handles = vec![];
    
    for _ in 0..4 {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        atomic_handles.push(handle);
    }
    
    for handle in atomic_handles {
        handle.join().unwrap();
    }
    
    println!("  原子计数: {}", atomic_counter.load(Ordering::SeqCst));
}