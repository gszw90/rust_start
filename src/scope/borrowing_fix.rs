
pub fn fix1(){
    let x = 5;
    // 填写空白处
    //let p = __;
    let p = &x;

    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

pub fn fix2(){
    let x = 5;
    let y = &x;

    // 只能修改以下行
    //assert_eq!(5, y);
    assert_eq!(5,*y);
}

pub fn fix3(){
    // let mut s = String::from("hello, ");
    // borrow_object3(s)

    let mut s = String::from("hello, ");
    borrow_object3(&s)
}
fn borrow_object3(s: &String) {}