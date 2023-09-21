// 仅仅支持通过转移所有权的方式获取一个值，那会让程序变得复杂
// Rust 通过 借用(Borrowing) 这个概念来达成上述的目的，获取变量的引用，称之为借用(borrowing)。

pub fn refer() {
    let x = 5;
    // 创建引用值,使用*来解引用
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn refer2() {
    let s = String::from("hello rust");
    // 将引用作为参数传递出去,不会发生所有权转移
    let l = cal_length(&s);
    println!("the length of {} is {}", s, l);

    // 可变引用需要本身就是可变的
    // 同一个作用域,同一个数据只能有一个可变引用,可以存在多个不可变引用,但是可变与不可变引用不可同时存在
    let mut s2 = String::from("changeble");
    //let sub = &mut s2;
    //let sub2 = &s2;
    //let sub3 = &s2;
    println!("before change:{}", s2);
    change(&mut s2);
    println!("after change:{}", s2);
    //println!("after change mut sub:{}", *sub);
    //println!("after change sub2:{}", *sub2);
    //println!("after change sub3:{}", *sub3);

    // 编译器优化特例
    let mut s = String::from("Juicy");
    let s2 = &s;
    let s3 = &s;
    println!("s2={},s3={}", *s2, *s3);//rust 1.31后 s2,s3的作用域在这里结束

    let s4 = &mut s;
    println!("s4 eq {}", *s4);
}// rust 1.31前s2,s3,s4的作用域在这里结束,以后只有s4的作用域在这里结束

fn cal_length(s: &String) -> usize {
    // 允许使用s的值,但是没有获取到所有权,也不可以修改s的值
    s.len()
}

// 添加mut使之变成可变类型,达到可以改变原值的目的
fn change(s: &mut String) {
    s.push_str("!!");
}

// 悬垂引用,也叫悬垂指针,表示指针指向某个值以后,这个值被释放掉了,而指针仍然存在,指向的内存可能不存在值或者已经被其他变量重新使用
pub fn dangling_reference() {}

// fn dangle()->&String{
//     let s = String::from("dangle");
//     &s
// }
// 可以把所有权转移出去,而不是返回一个引用
fn dangle() -> String {
    let s = String::from("dangle");
    s
}