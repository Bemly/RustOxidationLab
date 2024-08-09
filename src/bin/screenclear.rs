
mod common_cmds {
    /// # 常量
    ///
    /// NUL 字符串尾部\
    /// EOF 文件结束符\
    /// BEL 响铃字符\
    /// LF 换行符\
    /// CR 回车符\
    /// TAB 制表符\
    /// SPACE 空格\
    /// DEL 删除键\
    /// VT 垂直制表符\
    /// BS 退格键\
    /// FF 换页符\
    /// ESC CSI转义序列
    ///
    pub const NUL: char = '\0';
    pub const EOF: char = '\x1a';
    pub const LF: char = '\n';
    pub const BEL: char = '\x07';
    pub const CR: char = '\r';
    pub const TAB: char = '\t';
    pub const SPACE: char = ' ';
    pub const BS: char = '\x08';
    pub const DEL: char = '\x7f';
    pub const VT: char = '\x0b';
    pub const FF: char = '\x0c';
    pub const ESC: char = '\x1b';
    pub const ERASE_LINE: char = 'K';

    // front 删除前面 back 删除后面 visual 删除可视区域 all 清空
    enum CSI {
        Front, Back, Next, Prev, Up, Down, Right, Left, Home, End, Visual, All
    }
    impl CSI {
        /// 返回模式对应数字
        fn num_mode(self, mode: char) -> isize {
            match mode {
                'K' | 'J' => match self {
                    CSI::Front => 0,
                    CSI::Back => 1,
                    CSI::Visual => 2,
                    CSI::All => 3,
                    _ => unimplemented!("废弃参数不可达")
                },
                _ => unimplemented!("废弃参数不可达")
            }
        }
        fn num(self) -> isize {
            match self { 
                CSI::Front => 0,
                CSI::Back => 1,
                CSI::Visual => 2,
                _ => unimplemented!("废弃参数不可达")
            }
        }
        
        /// 拼接为转义序列
        fn str(self, mode: char) -> String { format!("{ESC}[{}{mode}", self.num_mode(mode)) }
    }
    
    //     清屏： \u001b[{n}J
    //     n=0从光标清除直到屏幕结束
    //     n=1从光标清除到屏幕开头
    //     n=2清除整个屏幕
    //     清行： \u001b[{n}K
    //     n=0从光标清除到行尾
    //     n=1从光标清除到行首
    //     n=2清除整行
    
    // 清行
    // 看上去对象是反着的，Rust枚举实现只能这样子惹（
    // 参数(模式) 反直觉捏 以后改过来
    pub fn clear_line() { print!("{ESC}[{}{ERASE_LINE}", CSI::Visual.num()); }
}

use common_cmds::*;

fn main() {
    println!("{BEL}")
}
