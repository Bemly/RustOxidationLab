mod addons;

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

        fn init() {}
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
        fn init() {}
        fn update() {}
        fn render() {}

    }

    pub mod init {
        pub fn main() {
            super::block::public_block_data()
        }
    }

    pub fn minecraft_function() {
        println!("This is minecraft Function.")
    }
}

fn get_block_data() {
    // 父mod访问子mod需要子mod公开pub mod
    // 访问里面函数体需要pub fn
    minecraft::init::main();

    // fn 和 mod 同级的兄弟 不用公开
    minecraft::minecraft_function();
    addons::pwd();
}