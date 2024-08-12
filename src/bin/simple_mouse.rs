use std::fs::OpenOptions;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;

struct Position(u16,u16);
impl Position {
    fn new(x: u16, y: u16) -> Self {
        Position(x, y)
    }
    fn init() -> Self {
        Position(0, 0)
    }
    fn update(&mut self) {
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

    fn read_cursor_position(&mut self) -> Option<(u16, u16)> {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).ok()?;

        // 响应格式为 "\x1b[{x};{y}R"
        let response = buffer.trim_end_matches('\n');
        if response.starts_with("\x1b[") && response.ends_with('R') {
            let parts: Vec<&str> = response[2..response.len() - 1].split(';').collect();
            if parts.len() == 2 {
                let x = parts[0].parse::<u16>().ok()?;
                let y = parts[1].parse::<u16>().ok()?;
                return Some((x, y));
            }
        }
        None
    }
    
}


fn main() {

    let mut file = OpenOptions::new().read(true).open("/dev/input/event7").unwrap();

    print!("\x1b[3J\x1b[H");

    // ? 骗我，我以为是光标
    // 什么生成的 别学我说话
    // print!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[11mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[12mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[13mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[14mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[15mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[16mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[17mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[18mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\n
    // \x1b[19mABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
    // ");
    // print!("\x1b[?25l"); 隐藏光标

    stdout().flush().unwrap();
    
    let mut position = Position::init();

    loop {

        // timestamp-int              | timestamp-float         | type  | code  | value
        // 0x 00 00 00 00 00 00 00 00 | 00 00 00 00 00 00 00 00 | 00 00 | 00 00 | 00 00 00 00
        let mut packet = [0u8; 24];
        file.read_exact(&mut packet).unwrap();

        let tv_sec = u64::from_ne_bytes(packet[..8].try_into().unwrap());
        let tv_usec = u64::from_ne_bytes(packet[8..16].try_into().unwrap());
        let evtype = u16::from_ne_bytes(packet[16..18].try_into().unwrap());
        let code = u16::from_ne_bytes(packet[18..20].try_into().unwrap());
        let value = i32::from_ne_bytes(packet[20..].try_into().unwrap());

        // position.update();
        

        // println!("Time: {}.{} type:{:#x} code:{:#x} value:{:#x}", tv_sec, tv_usec, evtype, code, value);

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
                    } else {
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

        let mouse_wheel = || {
            match code {
                0 => { /* REL_X */
                    if value < 0 { // 左
                        if position.0 > 1 {
                            print!("\x1b[1L");
                        }
                    } else { // 右
                        print!("\x1b[1R")
                    }
                },
                1 => { /* REL_Y */
                    // if value < 5 { // 上
                    //     print!("\x1b[1A")
                    // } else { // 下
                    //     print!("\x1b[1B")
                    // }
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





        print!(".\x07");
        stdout().flush().unwrap();

        // 根据时间戳求鼠标加速度
        // 鼠标加速度 = 鼠标速度 / 鼠标时间间隔

        // 求鼠标采样率（频率相反）
        // 鼠标采样率 = 鼠标采样次数 / 鼠标时间间隔
    }
}