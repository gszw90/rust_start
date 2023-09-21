// rust 中的字符包括ascii与unicode
pub fn char_type() {
    // 汉字字符,长度为4字节
    let a = '中';
    println!("汉字字符长度:{}", std::mem::size_of_val(&a));
    let mut s = String::from("");
    // 看看你我之间相隔的是什么
    for i in '我'..='你' {
        s.push(i);
        //s.clone().add(&i.to_string());
        //println!("{}",i);
    }
    println!("我与你之间的距离: {}", s)

    // 单元类型 ()
    // main()函数与println!()返回的也是单元类型,用来表示不关注具体的值,用来占位用,不占用内存空间
}

// 语句与表达式
// 语句: 会执行一个操作,不会返回值
// 表达式: 会在求值后返回一个值,表达式末尾是没有分号的
fn statement_and_expression() -> i32 {
    let a = 5; // 语句
    let b = 6; // 语句
    a + b // 表达式
}
