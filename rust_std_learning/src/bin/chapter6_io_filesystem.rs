// chapter6_io_filesystem.rs
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("=== 文件系统和I/O操作 ===\n");
    
    // 1. 文件读写
    println!("1. 文件读写操作:");
    
    // 写入文件
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!\nThis is a test file.")?;
    println!("  文件创建成功");
    
    // 读取文件
    let mut contents = String::new();
    File::open("example.txt")?.read_to_string(&mut contents)?;
    println!("  文件内容: {}", contents);
    
    // 2. 追加内容
    println!("\n2. 文件追加操作:");
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")?;
    file.write_all(b"\nAppended line!")?;
    println!("  内容追加成功");
    
    // 3. 缓冲读取
    println!("\n3. 缓冲读取:");
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);
    
    for (i, line) in reader.lines().enumerate() {
        println!("  行 {}: {}", i + 1, line?);
    }
    
    // 4. 文件定位
    println!("\n4. 文件定位:");
    let mut file = File::open("example.txt")?;
    file.seek(SeekFrom::Start(7))?; // 跳到第7个字节
    
    let mut buffer = [0; 5];
    file.read_exact(&mut buffer)?;
    println!("  从位置7读取: {}", String::from_utf8_lossy(&buffer));
    
    // 5. 目录操作
    println!("\n5. 目录操作:");
    
    // 创建目录
    fs::create_dir_all("test_dir/sub_dir")?;
    println!("  目录创建成功");
    
    // 写入测试文件
    fs::write("test_dir/file1.txt", "File 1 content")?;
    fs::write("test_dir/file2.txt", "File 2 content")?;
    
    // 读取目录
    println!("  目录内容:");
    for entry in fs::read_dir("test_dir")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("    文件: {:?}", path.file_name().unwrap());
        }
    }
    
    // 6. 路径操作
    println!("\n6. 路径操作:");
    let path = Path::new("test_dir/file1.txt");
    println!("  文件名: {:?}", path.file_name());
    println!("  扩展名: {:?}", path.extension());
    println!("  父目录: {:?}", path.parent());
    
    // 7. 文件元数据
    println!("\n7. 文件元数据:");
    let metadata = fs::metadata("example.txt")?;
    println!("  文件大小: {} 字节", metadata.len());
    println!("  是否文件: {}", metadata.is_file());
    println!("  是否目录: {}", metadata.is_dir());
    
    // 清理
    println!("\n8. 清理:");
    fs::remove_file("example.txt")?;
    fs::remove_dir_all("test_dir")?;
    println!("  临时文件清理完成");
    
    Ok(())
}