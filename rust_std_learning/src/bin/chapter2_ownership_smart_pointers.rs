// chapter2_ownership_smart_pointers.rs
use std::rc::Rc;
use std::cell::{RefCell, Cell};

fn main() {
    println!("=== 所有权和智能指针 ===\n");
    
    // 1. 所有权和移动语义
    println!("1. 所有权示例:");
    let s1 = String::from("hello");
    let s2 = s1; // s1的所有权移动到s2
    // println!("{}", s1); // 这行会编译错误！
    println!("  s2: {}", s2);
    
    // 2. Box<T> - 堆分配
    println!("\n2. Box<T> 示例:");
    let b = Box::new(5);
    println!("  box值: {}", b);
    
    // 递归类型示例
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("  递归列表: {:?}", list);
    
    // 3. Rc<T> - 引用计数
    println!("\n3. Rc<T> 示例:");
    let rc1 = Rc::new(42);
    {
        let rc2 = Rc::clone(&rc1);
        println!("  Rc引用计数: {}", Rc::strong_count(&rc1));
    }
    println!("  Rc引用计数: {}", Rc::strong_count(&rc1));
    
    // 4. RefCell<T> - 内部可变性
    println!("\n4. RefCell<T> 示例:");
    let ref_cell = RefCell::new(42);
    {
        let mut borrowed = ref_cell.borrow_mut();
        *borrowed += 1;
    }
    println!("  RefCell值: {}", ref_cell.borrow());
    
    // 5. Cell<T> - 内部可变性（复制语义）
    println!("\n5. Cell<T> 示例:");
    let cell = Cell::new(42);
    cell.set(100);
    println!("  Cell值: {}", cell.get());
}