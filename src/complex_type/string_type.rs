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
    // 在指定位置插入字符串
    let mut insetStr = String::from("insert string");
    println!("before insert:{}", insetStr);
    insetStr.insert_str(6, " mut");
    println!("after insert:{}", insetStr);

    // 字符串替换
    // replace:可作用于String,&str
    let replaceStr = String::from("replace rust");
    println!("before replace:{}", replaceStr);
    let newReplaceStr = replaceStr.replace("rust", "RUST");
    println!("after replace:{}", newReplaceStr);
    // repalcen: 指定替换个数
    let replacenStr = String::from("replacen rust,rust,rust");
    println!("before replacen:{}", replacenStr);
    let newReplacenStr = replacenStr.replacen("rust", "RUST", 1);
    println!("after replacen:{}", newReplacenStr);

    // repalce_range: 指定替换范围,作用于String,会替换原字符串,需要是可变的mut
    let mut replace_range_str = String::from("replaceRange rust,rust,rust");
    println!("before replace_range:{}", replace_range_str);
    replace_range_str.replace_range(13..17, "RUST");
    println!("after replace_range:{}", replace_range_str);

    // 删除
    // pop:删除并返回最后一个字符,直接操作原字符串
    let mut delete_str = String::from("delete 删除!");
    let pop_str = delete_str.pop();
    println!("pop str:{:?},after pop:{}", pop_str, delete_str);
    // remove:删除并返回指定位置字符,需要传入字符的起始位置,如果起始位置不是合法的字符边界,会报错,
    let remove_char = delete_str.remove(1);
    println!("remove str:{},after remove:{}", remove_char, delete_str);
    // truncate: 删除从指定位置到末尾的字符,没有返回值
    delete_str.truncate(5);
    println!("after truncate:{}", delete_str);
    // clear: 清空字符串
    delete_str.clear();
    println!("after clear:{}", delete_str);
}

// 字符串连接
pub fn concatenate() {
    // +/+=:要求右边的字符串必须是切片引用类型:&str
    let left_str = String::from("left");
    let right_str = String::from("right");
    // 此处使得left_str的所有权被移除了
    let long_str = left_str + &right_str;
    println!("+ 后字符串:{}", long_str);
    let mut long_str = long_str + "!";
    long_str += "??";
    println!("+ 后字符串:{}", long_str);
    // format!:格式化字符串
    let s1 = String::from("hello");
    let s2 = "rust";
    let s = format!("{} {}", s1, s2);
    println!("format!:{}", s);
}

// 操作字符串
pub fn handle() {
    // 字符,遍历字符串最好使用chars(),避免字符边界问题
    let unicode_chars = String::from("hello 打工人;打工人不打工人");
    for char in unicode_chars.chars() {
        print!("{} ", char);
    }
    println!("");
    // 遍历字节
    for byte in unicode_chars.bytes() {
        print!("{} ", byte);
    }
    println!("");
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
