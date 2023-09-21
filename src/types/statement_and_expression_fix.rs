pub fn fix1() {
    // // 使用两种方法让代码工作起来
    // let v = {
    //     let mut x = 1;
    //     x += 2
    // };

    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}

pub fn fix2() {
    // let v = (let x = 3);
    // assert!(v == 3);

    let v = {
        let x = 3;
        x
    };
    assert!(v == 3);
}

pub fn fix3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    // x + y;
    x + y
}