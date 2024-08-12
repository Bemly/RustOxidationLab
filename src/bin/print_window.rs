use std::io::{stdin, stdout, BufRead, Read, Write};
use std::process::Command;
use std::thread::sleep;


fn set_row_cacahe_mode(enable: bool) {
    let fd = libc::STDIN_FILENO;
    unsafe {
        let mut termios = std::mem::zeroed::<libc::termios>();
        libc::tcgetattr(fd, &mut termios);
        if !enable {
            termios.c_lflag &= !(libc::ICANON | libc::ECHO);
        } else {
            termios.c_lflag |= libc::ICANON | libc::ECHO;
        }
        libc::tcsetattr(fd, libc::TCSANOW, &termios);
    }
}


/// ```
///     A
/// C   6n   D
///     B
/// ```
fn main() {
    // \x1b[H 光标归位 \x1b[3J 清屏
    // 查看屏幕大小


    let mut buffer = [0u8; 1];
    // let mut rd = stdin();
    set_row_cacahe_mode(false);
    stdin().read_exact(&mut buffer).unwrap();
    print!("\x1b[3J\x1b[9999C\x1b[9999B\x1b[6n");
    println!("{buffer:?}")
    
    // print!("\x1b[6n");
    // stdout().flush().unwrap();
    //
    

    // stdout().flush().unwrap();
    // rd.read_exact(&mut buffer).unwrap();
    // let mut buffer = String::new();
    // stdin().read_line(&mut buffer).unwrap();

    // println!("{buffer:?}")
    // println!("{buffer:?}");

    // ESC[#;#R
}