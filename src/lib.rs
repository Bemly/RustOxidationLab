/// author: bemly_
/// create_time: 2024/08/06 17:06
/// update_time: 2024/08/06 17:06
/// 
/// 用法：我是二刺猿，这就是歌姬吧
/// 
/// # 举例
/// ```
/// let mut x = 5;
/// println!("{x}支持markdown哦，还能运行测试代码 好耶")
/// ```

/**
    这是文档块注释
*/

mod addons;
mod libs;

mod minecraft {
    mod block {
        fn init() {}
        fn update() {}
        fn render() {}

        fn private_block_data() {
            println!("This is block Function.")
        }
        pub fn public_block_data() {
            private_block_data();
        }
    }

    mod entity {
        use crate::minecraft::block;

        pub fn init() {}
        fn update() {}
        fn render() {}

        fn block_entity_bridge() {
            block::public_block_data();
            crate::minecraft::block::public_block_data();
            super::block::public_block_data();
            self::block::public_block_data();

            // block::init();
            // error[E0603]: function `init` is private
        }

        mod player {
        }
        mod animal {
            fn check_super_bridge() {
                super::block_entity_bridge();
                crate::minecraft::block::public_block_data();
                super::block::public_block_data();
            }
        }
    }

    mod world {
        pub fn init() {}
        fn update() {}
        fn render() {}

    }

    pub mod init {
        use super::world::*;
        use super::entity::init as aa;
        pub fn main() {
            super::block::public_block_data();

            // as 别名来区分 或者直接引入函数所在父mod
            init();
            aa();
        }
    }

    pub fn minecraft_function() {
        println!("This is minecraft Function.")
    }
}

/// 这是介绍
/// `add_one` 返回一个[`Option`]类型
pub fn get_block_data() {
    // 父mod访问子mod需要子mod公开pub mod
    // 访问里面函数体需要pub fn
    minecraft::init::main();

    // fn 和 mod 同级的兄弟 不用公开
    minecraft::minecraft_function();
    addons::pwd();
}
/*
    限制性可见性：
    pub 意味着可见性无任何限制
    pub(crate) 表示在当前包可见
    pub(self) 在当前模块可见
    pub(super) 在父模块可见
    pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

 */

/*
    两种方法下的目录结构：
    方法1：使用 mod.rs

    如果你选择在front_of_house文件夹内创建一个mod.rs文件来定义模块，目录结构将如下所示：

    src/
    │
    ├── lib.rs                 // crate根文件
    │
    └── front_of_house/        // 模块目录
        ├── mod.rs             // 定义front_of_house模块及其子模块
        └── hosting.rs         // hosting子模块

    在这个结构中：

        lib.rs中有一行mod front_of_house;，这指示Rust查找名为front_of_house的模块。
        front_of_house/mod.rs包含对子模块的定义，如pub mod hosting;，这告诉Rusthosting模块在hosting.rs文件中定义。

    方法2：使用与目录同名的 .rs 文件

    如果你选择在front_of_house目录同级下创建一个与目录同名的.rs文件来定义模块，目录结构将如下所示：

    src/
    │
    ├── lib.rs                 // crate根文件
    │
    ├── front_of_house.rs      // 定义front_of_house模块及其子模块
    │
    └── front_of_house/        // 模块目录
        └── hosting.rs         // hosting子模块

    在这个结构中：

        lib.rs中有一行mod front_of_house;，这指示Rust查找名为front_of_house的模块。
        front_of_house.rs包含对子模块的定义，如pub mod hosting;，这告诉Rusthosting模块在front_of_house/hosting.rs文件中定义。

    两种方法都可以有效地组织和管理复杂的模块结构，选择哪一种主要取决于个人或团队的偏好。新版本的Rust趋向于推荐使用第二种方法，因为这样可以避免多个mod.rs文件，使得项目结构更清晰。

*/