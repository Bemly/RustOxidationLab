use std::{io, thread};
use std::time::Duration;

fn current_cpu() -> Result<usize, io::Error> {
    let ret = unsafe {
        libc::sched_getcpu()
    };

    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret as usize)
    }
}
fn main() {
    // 创建一个线程A
    let new_thread = thread::spawn(move || {
        // 再创建一个线程B
        thread::spawn(move || {
            loop {
                print!("{}", current_cpu().unwrap());
            }

        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    // 睡眠主线程一段时间，看子线程创建的子线程是否还在运行
    thread::sleep(Duration::from_millis(1000000000));
}