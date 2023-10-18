pub fn fix1() {
    // 修复错误，不要新增代码行
    //let s: str = "hello, world";
    let s: &str = "hello, world";
}

// 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
pub fn fix2() {
    // 使用至少两种方法来修复错误
    //let s: Box<str> = "hello, world".into();
    //greetings2(s)
    // 1
    let s: Box<str> = "hello, world".into();
    greetings2(&s);
    // 2
    let s: Box<&str> = "hello, world".into();
    greetings2(*s);
}

fn greetings2(s: &str) {
    println!("{}", s)
}

pub fn fix3() {
    //let mut s = __;
    let mut s = String::new();


    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}

// 修复所有错误，并且不要新增代码行
pub fn fix4() {
    // let  s = String::from("hello");
    // s.push(',');
    // s.push(" world");
    // s += "!".to_string();

    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}

pub fn fix5() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    //let s1 = s.__("dogs", "cats");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}

// 你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
pub fn fix6() {
    // 修复所有错误，不要删除任何一行代码
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    //let s3 = s1 + s2;

    let s3 = s1.clone() + &s2;

    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

pub fn fix7() {
    // 使用至少两种方法来修复错误
    //let s = "hello, world";
    //greetings7(s);

    // method 1
    let s = "hello, world";
    greetings7(s.to_string());

    // method 2
    let s = String::from("hello, world");
    greetings7(s);
}

fn greetings7(s: String) {
    println!("{}", s)
}

// 我们可以使用 String::from 或 to_string 将 &str 转换成 String 类型
pub fn fix8() {
    // 使用两种方法来解决错误，不要新增代码行
    //let s = "hello, world".to_string();
    //let s1: &str = s;
    let s = "hello, world";
    let s1: &str = &s;

    let s = "hello, world".to_string();
    let s1: String = s;
}

pub fn fix9(){
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    //let byte_escape = "I'm writing Ru\x73__!";

    let byte_escape = "I'm writing Ru\x73\x74!";

    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

pub fn fix10(){
    //let raw_str = r"Escapes don't work here: \x3F \u{211D}";

    let raw_str = "Escapes don't work here: \x3F \u{211D}";

    // 修改上面的行让代码工作
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    //let long_delimiter = __;

    let long_delimiter = r###"Hello, "##""###;

    assert_eq!(long_delimiter, "Hello, \"##\"")
}