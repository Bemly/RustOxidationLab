use libc;
use std::io::{self, Read};

// 不得不用
fn set_canonical_mode(enable: bool) {
    let fd = libc::STDIN_FILENO;

    unsafe {
        let mut termios = std::mem::zeroed::<libc::termios>();
        libc::tcgetattr(fd, &mut termios);

        if !enable {
            // 关闭规范模式
            termios.c_lflag &= !(libc::ICANON | libc::ECHO);
        } else {
            // 打开规范模式
            termios.c_lflag |= libc::ICANON | libc::ECHO;
        }

        libc::tcsetattr(fd, libc::TCSANOW, &termios);
    }
}

fn main() {
    // 禁用规范模式（非缓冲模式）
    set_canonical_mode(false);

    println!("请输入字符（无需回车）：");

    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).expect("读取失败");
    println!("你输入的字符是：{}", buffer[0] as char);

    // 恢复规范模式
    set_canonical_mode(true);
}
