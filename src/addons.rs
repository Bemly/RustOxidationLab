// 引入子模块
mod ui;

pub(crate) fn pwd() {
    println!("{:?}", std::env::current_dir());

    super::minecraft::init::main();
}

// 给子模块addons文件夹下面的mod访问
// 新版本的Rust趋向于推荐使用第二种方法，因为这样可以避免多个mod.rs文件