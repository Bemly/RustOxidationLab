#![allow(overflowing_literals)]

use std::ops::{Range, RangeInclusive};
use std::u8;
use RustOxidationLab::get_block_data;

fn main() {

    // 普通的类型转换
    let _x:i16 = 1;
    let x = 1_u16;
    println!("{}", x);
    let x = 1u8;
    println!("{}", x);
    let x = 1 as u8;
    println!("{}", x);
    let x = _x;
    println!("{}", x);
    let x = 20u8 as i8;
    println!("{}", x);

    // 进制转换
    assert_eq!(i8::MAX, 0b0111_1111i8);
    assert_eq!(u8::MAX, 0b1111_1111u8);
    assert_eq!(i8::MAX, 0x7fi8);
    assert_eq!(u8::MAX, 0xffu8);

    assert_eq!(i8::MAX, 0b_01_111_111_i8);
    assert_eq!(u8::MAX, 0b_11_111_111_u8);
    assert_eq!(i8::MAX, 0o177i8);
    assert_eq!(u8::MAX, 0o377u8);

    // 区间范围
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert!(sum == -3 + -2);
    for c in 'a'..='z' {
        println!("{}",c as usize);
    }


    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    assert_eq!((1..=5).contains(&3), true);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // ?
    // 0&1=>0
    assert_eq!(true & false, false);
    // 1|0=>1
    assert_eq!(true | false, 1 != 0);
    // 0^1=>1 1^1=>0 0^0=>0
    assert_eq!(false ^ true, true);
    assert_eq!(true ^ true, false);
    assert_eq!(false ^ false, false);


    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    let v = {
        let mut x = 1;
        x + 2
    };

    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    fn some_function(input: Option<i32>) -> Option<i32> {
        match input {
            Some(value) => {
                Some(value)
            }, // 这里隐式返回value
            _ => {
                Some(0)
            }
        }
    }
    println!("{:?}", some_function(None));

    let mut s = String::from("hello, world!");
    s.push_str("!");

    // 值拷贝不需要借用borrowing
    // 自动拷贝的方式来赋值的，都被存在栈中，完全无需在堆上分配内存
    let x = 5;
    let _y = x;
    println!("{:?}",x);

    // 堆上的借用之后所有权ownership改变
    // 需要拷贝来开辟新空间
    // 存储在堆上的，因此不能自动拷贝
    // **一个值只允许有一个所有者!!!**
    let _s = s;
    // println!("{:?}",s);
    // error[E0382]: borrow of moved value: `s`
    s = _s.clone();
    println!("{:?}",s);
    println!("{:?}",_s);


    let foo = String::from("foo");
    let bar = 10;
    fn print_foo(foo: String) {
        println!("{:?}",foo);
    }
    fn print_bar(bar: i32) {
        println!("{:?}",bar);
    }

    print_foo(foo);
    // 此时进入函数，foo所有权被转移
    // print_foo(foo);
    // 存在堆上面（非基本类型）的没有Copy  trait 的类型，无法被多次借用
    // 基本类型具有Copy trait，可以多次借用（栈上的）
    print_bar(bar);
    print_bar(bar);

    // 如果使用&str引用，在传参的时候才新建所有权，本身foo2没有所有权
    let foo2 = "&str yeah!";
    print_foo(foo2.to_string());
    print_foo(foo2.to_string());

    // **总是把一个值传来传去来使用它**

    let a = 5;
    let b = &a;
    println!("{:p}", &a);
    println!("{:p}", b);
    println!("{:p}", &b);
    println!("==================");

    // 接上foo(dropped) 复写
    let foo = String::from("foo");
    fn print_foo_ref(foo: &String) {
        println!("{:p}", foo);
        println!("{:p}", &foo);
        println!("{}", *foo);
    }
    println!("{:p}",&foo);
    println!("{:p}",&*&*&*&*&*&*&*&*&*&*&*&*&*&foo);
    // 传入引用参数，不会转移所有权
    print_foo_ref(&foo);
    print_foo_ref(&foo);

    // **同一作用域，只能有一个可变引用**
    println!("==================");
    // error[E0499]: cannot borrow `foo` as mutable more than once at a time
    // let mut foo = String::from("foo");
    // let ref_foo1 = &mut foo;
    // {
    //     let ref_foo2 = &mut foo;
    //     println!("{}", ref_foo2);
    // }
    // println!("{}", ref_foo1);

    // 指定生命周期来结束掉ref_foo1的可变引用
    let mut foo = String::from("foo");
    let ref_foo1 = &mut foo;
    // &mut 属于可变引用，不存在所有权转移，没有所有权
    println!("{}", ref_foo1);
    println!("{}", ref_foo1);
    ref_foo1.push_str("bar");
    println!("{}", ref_foo1);
    println!("{:p}", ref_foo1);
    // 1的引用结束，2的引用开始，可变引用的地址不变
    // 下面没有用到1，这里编译后自动dropped释放1
    // **引用作用域的结束位置从花括号变成最后一次使用的位置** Rust > 1.31
    {
        let ref_foo2 = &mut foo;
        println!("{}", ref_foo2);
        println!("{:p}", ref_foo2);
    }
    // Rust中的可变引用不拥有数据的所有权，但它们确实有权限修改数据
    println!("{}", foo);
    // 可变引用与不可变引用不能同时存在
    // 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    let s = String::from("hello, world");
    // convert String to Vec
    let _s = s.into_bytes();


    // 元组如果里面都是Copy就可以Copy，这里第四个元素是as_str(&str)引用，所以可以Copy
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);

    // 这里toString()开辟了堆空间，
    //  只能深拷贝clone（新建所有权）
    //  或者引用（不要所有权）
    //  或者可变引用（不要所有权）
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);



    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);
    // println!("The person's name from person struct is {}", person.name);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);

    let foo = 1;
    println!("{foo}"); // 原来println自带format大意了啊 我没有闪
    let ref ref_foo1 = foo;
    let ref_foo2 = &foo;
    assert_eq!(format!("{ref_foo1:p}"), format!("{ref_foo2:p}"));

    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码
    let s = String::from("hello,world!");
    println!("{}", &s);
    println!("{}", &s[..]);
    println!("{}", s.as_str());

    // 数组中的所有元素必须是同一类型,元组中的元素可以是不同类型

    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    assert_eq!(std::mem::size_of_val(&slice), 16);
    println!("{:?}", slice);

    // 元组结构体解构需要加上元组名


    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Qux(10);
    if let Foo::Bar = a {
        println!("match foo::bar")
    } else if let Foo::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
    // if let 也可以else
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        Foo::Qux(_) => println!("match others")
    }


    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }


    let mut v = String::from("hello,");
    let r = &mut v;


    match *r {
        ref mut value => value.push_str(" world!"),
    }
    // 下面这几个会转移所有权
    // match r {
    //     mut value => value.push_str(" world!"),
    // }
    match r {
        value => value.push_str(" world!"),
    }

    // 结构体类型的一部分，和数组类型一样，这意味着长度不同会导致类型不同
    struct Array<T, const N: usize> {
        data : [T; N]
    }
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            // data: [1.0, 2.0, 3.0],
            data: [1, 2, 3],
        },
        Array {
            // data: [1, 2],
            data: [1, 2, 4],
        }
    ];

    fn multiply<T: std::ops::Mul<U, Output=V>,U,V>(x: T,y: U) -> V {
        x * y
    }
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    // 特征对象
    // 当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    //
    //     方法的返回类型不能是 Self
    //     方法没有任何泛型参数
    fn sum<T>(x: T, y: T) -> T
    where T: std::ops::Add<T,Output=T> {
        x + y
    }



    fn example1() {
        // `T: Trait` 是最常使用的方式
        // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }

        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    },
                }
            }
        }

        fn anonymous_function(x: u32) -> u32 { x + 1 }
        let mut cacher = Cacher::new(anonymous_function);
        assert_eq!(cacher.value(10), 11);
        assert_eq!(cacher.value(15), 11);
    }


    fn example2() {
        struct Cacher<T>
        where T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    },
                }
            }
        }

        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(20), 21);
        assert_eq!(cacher.value(25), 21);
    }

    example1();
    example2();




    trait Bird {
        fn quack(&self) -> String;
    }

    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String{
            "duck duck".to_string()
        }
    }

    impl Bird for Swan {
        fn quack(&self) -> String{
            "swan swan".to_string()
        }
    }
    fn hatch_a_bird(tp: u8) -> Box<dyn Bird> {
        match tp {
            1 => Box::new(Swan {}),
            2 => Box::new(Duck {}),
            _ => unimplemented!()
        }
    }
    let duck = Duck {};
    duck.swim();

    // 类的多态性（鸭子类型）=> 封装传参
    //      1.子类不能逆推继承的父类
    //      2.实现功能不能逆推具体实例

    // 2.!!!!
    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");


    // 1.!!!!
    // dyn必须显式声明，因为编译器无法推断出dyn Bird的实现
    // 需要在运行时才能确定dyn Bird的实现
    // let birds = [Box::new(Duck{}), Box::new(Swan{})];
    let birds = [Box::new(Duck{}) as Box<dyn Bird>, Box::new(Swan{})];
    let first_type = &birds[0];
    let second_type = &birds[1];
    // 上面数组必须是同类型 第二个被隐式转换为dyn Bird
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck{}) , Box::new(Swan{})];
    for bird in birds {
        println!("{}", bird.quack());
        // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
        // 因此，以下代码会报错
        // bird.fly();
    }

    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    let x = 1.1f64;
    let y = 8u8;
    // draw x
    draw_with_box(Box::new(x));
    // draw y
    draw_with_ref(&y);
    println!("Success!");

    // dyn_box动态内存开辟和dyn_ref动态引用的用法区别
    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }
    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }




    trait Foo1 {
        fn method(&self) -> String;
    }

    impl Foo1 for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo1 for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // 1.a.通过泛型实现以下函数 => 静态分发
    fn static_dispatch1<T>(x: T)
    where T: Foo1 {
        x.method();
    }
    // 1.b.通过特征实现以下函数 => 静态分发 (泛型的语法糖)
    fn static_dispatch2(x: impl Foo1) {
        x.method();
    }

    // 2.a.通过特征实现以下函数 => 静态分发 (泛型的语法糖)
    fn dynamic_dispatch1(y: &impl Foo1) {
        y.method();
    }
    // 2.b.通过泛型实现以下函数 => 静态分发
    fn dynamic_dispatch<T: Foo1>(y: &T) {
        y.method();
    }
    // 2.c.通过特征对象实现以下函数 => 动态分发
    fn dynamic_dispatch2(y: &dyn Foo1) {
        y.method();
    }

    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch1(x);
    static_dispatch2(x);
    dynamic_dispatch(&y);
    dynamic_dispatch1(&y);
    dynamic_dispatch2(&y);

    println!("Success!");

    // 上一个还是unsafe的循环链表指针捏
    // 为工程性考虑，dyn违背rust的变量类型编译时确定原则，个人项目不建议使用
    // dyn死线 => 确保对象安全 => 特征对象的必要条件
    // 当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    //
    //     方法的返回类型不能是 Self => 破坏子类与父类的继承关系
    //     方法没有任何泛型参数 => 动态分发不和静态分发冲突
    // 动态(运行时确定)：
    trait MyTrait {
        fn f(&self) -> Box<dyn MyTrait>;
    }
    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
    }
    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
    }
    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
        x.f()
    }
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
    println!("Success!");

    // 静态特征(编译期确定)：
    trait MyTrait2 {
        fn f(&self) -> Self;
    }

    impl MyTrait2 for u32 {
        fn f(&self) -> u32 { 42 }
    }

    impl MyTrait2 for String {
        fn f(&self) -> String { self.clone() }
    }

    fn my_function2(x: impl MyTrait2) -> impl MyTrait2  {
        x.f()
    }
    my_function2(13_u32);
    my_function2(String::from("abc"));

    // Rust 并不支持创建自定义运算符，你也无法为所有运算符进行重载，目前来说，只有定义在 std::ops 中的运算符才能进行重载

    // 标记的生命周期只是为了取悦编译器，让编译器不要难为我们

    // 允许overflowing_literals溢出，但是不安全
    assert_eq!(u8::MAX, 255);
    let v = 257 as u8;
    println!("{}", v);


    fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, std::num::ParseIntError> {
        Ok(n1_str.parse::<i32>()? * n2_str.parse::<i32>()?)
    }
    assert_eq!(multiply2("3", "4").unwrap(), 12);


    // 1.map => 要封装成Result<T, E>
    // 2.and_then => 不会封装，要手动包装
    fn add_two1(n_str: &str) -> Result<i32, std::num::ParseIntError> {
        n_str.parse::<i32>().map(|num| num +2)
    }
    fn add_two2(n_str: &str) -> Result<i32, std::num::ParseIntError> {
        n_str.parse::<i32>().and_then(|num| Ok(num +2))
    }
    assert_eq!(add_two1("4").unwrap(), 6);
    assert_eq!(add_two2("4").unwrap(), 6);

    fn print(result: Result<i32, std::num::ParseIntError>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    fn multiply_foo(n1_str: &str, n2_str: &str) -> Result<i32, std::num::ParseIntError> {
        // 实现...
        n1_str.parse::<i32>().and_then(|n1| Ok(n1 * n2_str.parse::<i32>()?))
    }
    fn multiply_bar(n1_str: &str, n2_str: &str) -> Result<i32, std::num::ParseIntError> {
        // IMPLEMENT...
        n1_str.parse::<i32>().and_then(|n1| {
            n2_str.parse::<i32>().map(|n2| n1 * n2)
        })
    }
    // This still presents a reasonable answer.
    let twenty = multiply_foo("10", "2");
    print(twenty);
    // The following now provides a much more helpful error message.
    let tt = multiply_bar("2", "2");
    print(tt);

    // 通过二进制运行包引入库包
    get_block_data();
    
}

/// ```
/// # // 使用#开头的行会在文档中被隐藏起来，但是依然会在文档测试中运行
/// # fn try_main() -> Result<(), String> {
/// let res = world_hello::compute::try_div(10, 0)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// #
/// # }
/// ```
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// world_hello::compute::div(10, 0);
/// ```
/// 
pub fn div(x: i32, y: i32) -> i32 {
    assert_ne!(y, 0);
    x / y
}