use enigo::{Enigo, Mouse, Settings};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::new(0, 1000_000_00));
        println!("{:?} \x07", enigo.location())
    }
}
