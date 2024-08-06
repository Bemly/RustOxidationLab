
pub(crate) fn pwd() {
    println!("{:?}", std::env::current_dir());

    super::minecraft::init::main();
}