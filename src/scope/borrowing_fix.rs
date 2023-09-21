pub fn fix1() {
    let x = 5;
    // 填写空白处
    //let p = __;
    let p = &x;

    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

pub fn fix2() {
    let x = 5;
    let y = &x;

    // 只能修改以下行
    //assert_eq!(5, y);
    assert_eq!(5, *y);
}

pub fn fix3() {
    // let mut s = String::from("hello, ");
    // borrow_object3(s)

    let mut s = String::from("hello, ");
    borrow_object3(&s)
}

fn borrow_object3(s: &String) {}

pub fn fix4() {
    // 修复错误
    let mut s = String::from("hello, ");

    //push_str4(s);
    push_str4(&mut s);
}

fn push_str4(s: &mut String) {
    s.push_str("world")
}

pub fn fix5() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    //let p = __;
    let p = &mut s;

    p.push_str("world");
}

// ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。
pub fn fix6() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    //let __ r2 = c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr6(r1), get_addr6(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr6(r: &char) -> String {
    format!("{:p}", r)
}

// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
pub fn fix7() {
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}

pub fn fix8() {
    // 通过修改下面一行代码来修复错误
    //let  s = String::from("hello, ");
    let mut s = String::from("hello, ");

    borrow_object8(&mut s);
}

fn borrow_object8(s: &mut String) {}

// 从可变对象借用不可变
// 下面的代码没有任何错误
pub fn fix9() {
    let mut s = String::from("hello, ");

    borrow_object9(&s);

    s.push_str("world");
}

fn borrow_object9(s: &String) {}

// 注释掉一行代码让它工作
pub fn fix10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}

pub fn fix11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    //r1.push_str("world");
}