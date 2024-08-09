use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {


    // 尝试发送转义序列以启用鼠标事件报告
    print!("\x1b[?1000h");
    io::stdout().flush().unwrap();

    // 等待一段时间来模拟程序运行
    thread::sleep(Duration::from_secs(10));

    // 尝试读取输入（在这个示例中，我们只是简单地读取并打印）
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => println!("Received: {}", buffer),
        Err(error) => println!("Error reading from stdin: {}", error),
    }

    // 禁用鼠标事件报告
    print!("\x1b[?1000l");
    io::stdout().flush().unwrap();
}
