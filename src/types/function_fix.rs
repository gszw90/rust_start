pub fn fix1() {
    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

// fn sum(x, y: i32) {
//     x + y;
// }
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn fix2() {
    print();
}

// 使用另一个类型来替代 i32
// fn print() -> i32 {
//     println!("hello,world");
// }
fn print() -> () {
    println!("hello,world");
}

pub fn fix3() {
    // 填空
    // let b = __;
    //
    // let _v = match b {
    //     true => 1,
    //     // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
    //     false => {
    //         println!("Success!");
    //         panic!("we have no value for `false`, but we can panic")
    //     }
    // };

    let b = false;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
