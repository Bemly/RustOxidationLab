use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    let mut file = OpenOptions::new().read(true).open("/dev/input/event7").unwrap();


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

        println!("Time: {}.{} type:{:#x} code:{:#x} value:{:#x}", tv_sec, tv_usec, evtype, code, value);


        // 根据时间戳求鼠标加速度
        // 鼠标加速度 = 鼠标速度 / 鼠标时间间隔

        // 求鼠标采样率（频率相反）
        // 鼠标采样率 = 鼠标采样次数 / 鼠标时间间隔
    }
}