// &str是基本类型,String是复合类型
// rust中字符是unicode类型,每个占4字节内存空间,但是在字符串是utf8,字符所占空间是(1-4)字节,是动态的
pub fn str1() {
    let s1: &str = "hello";
    let s2: String = String::from("world");
    // &str 转化为 String
    let s11: String = String::from("to String");
    let s12: String = "to String".to_string();
    // String 转化为&str只需要取引用即可
    let s21: &str = &s11;
    let s22: &str = &s11[..];
    let s23: &str = s11.as_str();

    // 字符串中不能通过索引直接获取字符,因为每个字符是utf8格式,所占空间为(1-4),不能确定字符边界

    // 追加字符,必须是可变类型
    let mut s = String::from("hello");
    println!("before push:{}", s);
    s.push_str(" rust");
    println!("after push:{}", s);
}

pub fn slice_type() {
    let s = String::from("hello world");
    // 截取hello
    let hello = &s[0..5];
    // 截取world
    let world = &s[6..11];
    // 从头开始截取
    let hello = &s[..5];
    // 截取到末尾为止
    let world = &s[6..];
    // 截取全部
    let total = &s[..];
    // 字符截取时需要注意字符间的边界位置,也就是utf8的边界位置,如中文一个字符占3字节
    let s = String::from("中国人民");
    // 会报错
    let s1 = &s[..2];
}
