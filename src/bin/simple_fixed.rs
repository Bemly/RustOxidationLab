use std::fs::OpenOptions;
use std::io::{stdout, Read, Write};
use std::process::exit;

fn main() {

    let mut file = OpenOptions::new().read(true).open("/dev/input/event7").unwrap();

    print!("\x1b[3J\x1b[H");
    /* RRRRRRRRRRGGGGGGGGGGGGGGBBBBBBBBBB 24-bit
    for r in 0..256 {
        for g in 0..256 {
            for b in 0..256 {
                print!("\x1b[48;2;{r};{g};{b}mA\x1b[0m");
            }
        }
    }
    */

    /* 8-bit 256 color
    for i in 0..256 {
        print!("\x1b[48:5:{i}m ")
    }
     */
    
    /* 3/4-bit 16 color
    for i in 0..11 {
        for j in 0..10 {
            let n = 10 * i + j;
            if n > 108 { break; }
            print!("\x1b[{n}m{n:3}\x1b[m");
        }
        println!();
    }
     */
    
    stdout().flush().unwrap();


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

        // println!("Time: {}.{} type:{:#x} code:{:#x} value:{:#x}", tv_sec, tv_usec, evtype, code, value);

        let mouse_key = || {
            match code {
                0x110_u16 => { /* BTN_LEFT */
                    if let 0x1_i32 = value {
                        // BTN_DOWN
                        print!("\x1b[?5h"); // 反色开始
                        stdout().flush().unwrap();
                    } else { 
                        // BTN_UP
                        print!("\x1b[?5l"); // 反色结束
                        stdout().flush().unwrap();
                    }
                },
                0x111 => { /* BTN_RIGHT */
                    if let 0x1_i32 = value {
                        print!("\x1b[;46m");
                        stdout().flush().unwrap();
                    } else {
                        print!("\x1b[0m");
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

        let mouse_wheel = || {
            match code {
                0 => { /* REL_X */ },
                1 => { /* REL_Y */ },
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