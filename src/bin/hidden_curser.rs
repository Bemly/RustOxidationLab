use std::fs::OpenOptions;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::process::exit;
use libc::{c_ulong, ioctl};

const EVIOCGRAB: c_ulong = 1074021776; // ioctl command to grab the device

fn main() {
    // 打开/dev/input/event7设备文件
    let mut file = OpenOptions::new().read(true).open("/dev/input/event6").unwrap();

    let mut isctrl = false;

    // 使用ioctl来抓取设备
    unsafe {
        if ioctl(file.as_raw_fd(), EVIOCGRAB, 1) != 0 {
            eprintln!("Failed to grab the device");
            return;
        }
    }
    // println!("▀▄█■");
    // 240 67
    // 240/8 67*2/8
    // 30 16 
    println!("");

    // 初始化一个缓冲区来存储读取的数据
    let mut buffer = [0u8; 24];  // 输入事件结构体的大小是24字节

    loop {
        // 从文件读取24字节的数据
        file.read_exact(&mut buffer).unwrap();

        // 解析时间戳（tv_sec和tv_usec）
        // let _tv_sec = u64::from_ne_bytes(buffer[..8].try_into().unwrap());
        // let _tv_usec = u64::from_ne_bytes(buffer[8..16].try_into().unwrap());

        // 解析事件类型、代码和值
        let evtype = u16::from_ne_bytes(buffer[16..18].try_into().unwrap());
        let code = u16::from_ne_bytes(buffer[18..20].try_into().unwrap());
        let value = i32::from_ne_bytes(buffer[20..24].try_into().unwrap());

        // 只处理键盘事件（evtype == 1 表示键盘事件）
        if evtype == 1 {
            // println!("Key event: code = {}, value = {}", code, value);
            match code {
                0x1D_u16 | 0x61 => isctrl = value == 1,
                0x2E if isctrl => {
                    // 当程序退出时，释放设备
                    unsafe {
                        ioctl(file.as_raw_fd(), EVIOCGRAB, 0);
                    }
                    println!("\x07");
                    exit(0)
                }
                _ => {}
            }
        }




    }


}
