pub fn fix1() {
    // 使用尽可能多的方法来通过编译
    // let x = String::from("hello, world");
    // let y = x;

    let x = "hello, world";
    let y = x;

    println!("{},{}", x, y);
}

pub fn fix2() {
    let s1 = String::from("hello, world fix2");
    let s2 = take_ownership2(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
// fn take_ownership2(s: String) {
//     println!("{}", s);
// }
fn take_ownership2(s: String) -> String {
    println!("{}", s);
    s
}

pub fn fix3() {
    let s = give_ownership3();
    println!("{}", s);
}
// 只能修改下面的代码!
// fn give_ownership3() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let _s = s.into_bytes();
//     s
// }

fn give_ownership3() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    let _s = s.clone().into_bytes();
    s
}

pub fn fix4() {
    let s = String::from("hello, world");

    //print_str4(s);
    print_str4(s.clone());

    println!("{}", s);
}

// 修复错误，不要删除任何代码行
fn print_str4(s: String) {
    println!("{}", s)
}

pub fn fix5() {
    // 不要使用 clone，使用 copy 的方式替代
    // let x = (1, 2, (), "hello".to_string());
    // let y = x.clone();
    // println!("{:?}, {:?}", x, y);

    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

// 当所有权转移时，可变性也可以随之改变。
pub fn fix6() {
    let s = String::from("hello, ");

    // 只修改下面这行代码 !
    //let s1 = s;
    let mut s1 = s;

    s1.push_str("world")
}

pub fn fix7() {
    // let x = Box::new(5);
    // let ...      // 完成该行代码，不要修改其它行！
    // *y = 4;
    // assert_eq!(*x, 5);

    let x = Box::new(5);
    let mut y = Box::new(3);
    *y = 4;
    assert_eq!(*x, 5);
}