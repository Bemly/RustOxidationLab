use std::fs::OpenOptions;
use std::io::{self, stdin, stdout, Read, Write};
use std::process::exit;
use std::thread;
use std::sync::mpsc;

/// C 还在追我\
/// 干嘛的：不需要换行来强行刷新缓存了！
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

/// 光标位置 鼠标基础
# [derive(Debug)]
struct Position(u16,u16);
impl Position {
    fn new(x: u16, y: u16) -> Self {
        Position(x, y)
    }
    fn init() -> Self {
        Position(0, 0)
    }
    fn auto_update(&mut self) {
        print!("\x1b[6n");
        stdout().flush().unwrap();
        // 读取终端的响应
        if let Some((x, y)) = self.read_cursor_position() {
            self.0 = x;
            self.1 = y;
        } else {
            panic!("Invalid cursor position response");
        }
    }

    fn read_cursor_position(&self) -> Option<(u16,u16)> {
        todo!()
    }

    fn update(&mut self) {

    }
}


fn main() {
    // 关闭他喵的为我好的行缓冲模式
    set_row_cacahe_mode(false);

    let mut position = Position::init();
    let mut screen = Position::init();
    
    // 线程通信，和gpio一致
    // Master ==tx ==> Slave
    // Master <== rx== Slave
    // revc时阻塞等待
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut buffer = Vec::new();
        // 是靠监听R字符实现的，运行期间手动输入R会造成不可预期的错误
        for byte in stdin().bytes() {
            match byte {
                Ok(ascii@ b'R') => {
                    buffer.push(ascii);
                    tx.send(buffer).expect("无法发送输入");
                    break
                },
                Ok(ascii) => buffer.push(ascii),
                Err(e) => panic!("{e}")
            }
        }
    });

    // 清屏 移动光标到右下 获取位置
    print!("\x1b[3J\x1b[9999C\x1b[9999B\x1b[6n");
    stdout().flush().unwrap();

    // 得到坐标(行列) => 自己计算行列不再依靠单独计算
    // TODO:　这样做不能动态更改行列，之后需要时刻刷新，解藕注册模式
    if let Ok(input) = rx.recv() {
        // println!("你输入的是：{:#x?}", input);
        // test output: 你输入的是：[0x1b,0x5b,0x31,0x32,0x3b,0x32,0x35,0x30,0x52,0xa,]
        // ASCII MODE:  0x52 => `R`, 0xa => `\n`, 0x5b => `[`, X ,0x3b => `;`, Y
        if !input.is_ascii() { unimplemented!("Invalid input. 不正确的输入，获取坐标异常") }
        let mut slice_symbol = 0_u8;
        let mut x = Vec::new();
        let mut y = Vec::new();
        for ascii in input {
            match ascii {
                // 1 => X , 2 => Y, 3 => STOP
                b'[' => slice_symbol = 1,
                b';' => slice_symbol = 2,
                b'R' => break,
                Y if slice_symbol == 1 => y.push(Y),
                X if slice_symbol == 2 => x.push(X),
                _ => {}
            }
        }

        let ascii2u16 = |acc, &ascii| acc * 10u16 + (ascii - 0x30) as u16;
        screen.0 = x.iter().fold(0, ascii2u16);
        screen.1 = y.iter().fold(0, ascii2u16);
    }

    // 开启他喵的为我好的行缓冲模式
    set_row_cacahe_mode(true);

    println!("窗口大小：({},{})", screen.0, screen.1);

    print!("\x1b[H");
    stdout().flush().unwrap();
    for i in 0..41 {
        println!("{i} || ");
    }
    print!("\x1b[H");
    stdout().flush().unwrap();

    // 读取鼠标数据 - 外围驱动
    let mut file = OpenOptions::new().read(true).open("/dev/input/event7").unwrap();
    loop {

        // timestamp-int              | timestamp-float         | type  | code  | value
        // 0x 00 00 00 00 00 00 00 00 | 00 00 00 00 00 00 00 00 | 00 00 | 00 00 | 00 00 00 00
        let mut packet = [0u8; 24];
        file.read_exact(&mut packet).unwrap();

        /* TODO:
            根据时间戳求鼠标加速度
            鼠标加速度 = 鼠标速度 / 鼠标时间间隔
            求鼠标采样率（频率相反）
            鼠标采样率 = 鼠标采样次数 / 鼠标时间间隔
            争取把timestamp搞出来
        */
        let _tv_sec = u64::from_ne_bytes(packet[..8].try_into().unwrap());
        let _tv_usec = u64::from_ne_bytes(packet[8..16].try_into().unwrap());
        let evtype = u16::from_ne_bytes(packet[16..18].try_into().unwrap());
        let code = u16::from_ne_bytes(packet[18..20].try_into().unwrap());
        let value = i32::from_ne_bytes(packet[20..].try_into().unwrap());

        // println!("Time: {}.{} type:{:#x} code:{:#x} value:{:#x}", tv_sec, tv_usec, evtype, code, value);

        // 监听鼠标事件
        let mouse_key = || {
            match code {
                0x110_u16 => { /* BTN_LEFT */
                    if let 0x1_i32 = value {
                        // BTN_DOWN
                    } else {
                        // BTN_UP
                    }
                },
                0x111 => { /* BTN_RIGHT */
                    if let 0x1_i32 = value {
                        // print!("\x1b[;46mW");
                        stdout().flush().unwrap();
                    } else {
                        // print!("\x1b[0m");
                        stdout().flush().unwrap();
                    }
                },
                0x112 => { /* BTN_MIDDLE */ },
                0x113 => { /* BTN_SIDE */ },
                0x114 => { /* BTN_EXTRA */
                    print!("\x1b[3J\x1b[H");
                    exit(0)
                },
                _ => unimplemented!()
            }
        };


        /// ```
        ///     A
        /// D   6n   C
        ///     B
        /// ```
        fn this_is_ign_warn_comment() {}
        // 最丑的写法来了 快给你看我新拉的一堆大的
        let mut mouse_wheel = || {
            match code {
                0 => { /* REL_X */
                    if value < 0 { // 左
                        if position.0 > 1 {
                            position.0 -= 1;
                            print!("\x1b[1D")
                        }
                    } else { // 右
                        if position.0 < screen.0 {
                            position.0 += 1;
                            print!("\x1b[1C")
                        }
                    }
                },
                1 => { /* REL_Y */
                    if value.abs() > 1 {
                        if value < 0 { // 上
                            if position.1 > 1 {
                                position.1 -= 1;
                                print!("\x1b[1A")
                            }
                        } else { // 下
                            if position.1 < screen.1 {
                                position.1 += 1;
                                print!("\x1b[1B")
                            }
                        }
                    }
                },
                8 => { /* REL_WHEEL */ },
                11 => { /* REL_WHEEL_HI_RES */ },
                _ => unimplemented!()
            }
        };

        match evtype {
            0x00_u16 => { /* EOF */ },
            0x01 => mouse_key(),
            0x02 => mouse_wheel(),
            0x03 => unimplemented!(),
            0x04 => { /* SYNC */ },
            _ => unimplemented!()
        }

        // print!("\x07");
        stdout().flush().unwrap();
    }
}
