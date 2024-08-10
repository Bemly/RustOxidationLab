use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use byteorder::{NativeEndian, ReadBytesExt};

fn main() {
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);

    let mut dev_file = file_options.open("/dev/input/event7").unwrap();

    // 计算鼠标轮询率
    let checker = thread::spawn(move || {
        
    });

    loop {

        let mut packet = [0u8; 24];
        dev_file.read_exact(&mut packet).unwrap();

        let mut rdr = Cursor::new(packet);
        let tv_sec  = rdr.read_u64::<NativeEndian>().unwrap();
        let tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
        let evtype  = rdr.read_u16::<NativeEndian>().unwrap();
        let code    = rdr.read_u16::<NativeEndian>().unwrap();
        let value   = rdr.read_i32::<NativeEndian>().unwrap();

        println!("Time: {}.{} type:{} code:{} value:{}", tv_sec, tv_usec, evtype, code, value);
        
        
        
    }
}