pub struct MyInteger;

impl MyInteger {
    pub fn show() {
        println!("this is integer mod");
        let a1: i8 = 8;
        let a2: u8 = 9;
        println!("a1 = {a1},a2 = {a2}");
        let b1: i16 = 16;
        let b2: u16 = 17;
        println!("b1 = {b1},b2 = {b2}");
        let c1: isize = 8;
        let c2: usize = 9;
        println!("c1 = {c1},c2 = {c2}");
    }
}

pub fn numbers() {
    // nan 类型不可用于比较
    let x = (-42.0_f32).sqrt();
    println!("x = {x}");
    // 比较时程序会崩溃
    //assert_eq!(x,x);

    // 可以用is_nan()来判断
    if x.is_nan() {
        println!("is not a number");
    }

    // 位运算
    let a = 2;
    let b = 3;
    // 与
    println!("(a & b) = {}", a & b);
    // 或
    println!("(a | b) = {}", a | b);
    // 异或
    println!("(a ^ b) = {}", a ^ b);
    // 非
    println!("(!b) = {}", !b);
    // 左移
    println!("(a << b) = {}", a << b);
    // 右移
    println!("(a >> b) = {}", a >> b);

    // range 生成连续数值
    println!("range");
    // 0到10,包括10
    for i in 0..=10 {
        println!("number => {i}");
    }
    // 0到10,不包括10
    for i in 0..10 {
        println!("number2 => {i}");
    }
    // 也可以是字符
    for i in 'a'..='z' {
        println!("char => {i}");
    }
}
