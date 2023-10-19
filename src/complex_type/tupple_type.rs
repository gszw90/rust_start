// 元组:由多种类型组合在一起的复合类型,其中元素长度固定,元素顺序固定

pub fn eg1() {
    // 元组创建
    let tup: (i32, f64, &str) = (12, 12.3, "hello");
    // 使用模式匹配结构元组
    let (x,y,z) = tup;
    println!("{},{},{}",x,y,z);
}