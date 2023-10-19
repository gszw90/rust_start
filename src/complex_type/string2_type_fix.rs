use std::mem;

// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
pub fn fix1() {
    // let mut s: String = "hello, ";
    // s.push_str("world".to_string());
    // s.push(__);

    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership1(&s);

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership1(s: &String) {
    println!("ownership of \"{}\" is moved here!", s)
}

pub fn fix2() {
    let mut s = String::from("hello, world");

    //let slice1: &str = __; // 使用两种方法
    let slice1: &str = &s;
    assert_eq!(slice1, "hello, world");

    let slice11: &str = &s[..];
    assert_eq!(slice11, "hello, world");

    //let slice2 = __;
    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    //let slice3: __ = __;
    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}

// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答: 2
fn fix3() {
    // 基于 `&str` 类型创建一个 String,
    // 字符串字面量的类型是 `&str`
    let s: String = String::from("hello, world!"); // 第一次

    // 创建一个切片引用指向 String `s`
    let slice: &str = &s;

    // 基于刚创建的切片来创建一个 String
    let s: String = slice.to_string(); // 第二次

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

pub fn fix4() {
    let s = String::from("hello, 世界");
    //let slice1 = s[0]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    let slice1 = &s[..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    assert_eq!(slice1, "h");

    //let slice2 = &s[3..5];// 提示: `世` 在 UTF-8 编码中占用 3 个字节
    let slice2 = &s[7..10];// 提示: `世` 在 UTF-8 编码中占用 3 个字节
    assert_eq!(slice2, "世");

    // 迭代 s 中的所有字符
    // for (i, c) in s.__ {
    //     if i == 7 {
    //         assert_eq!(c, '世')
    //     }
    // }
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}

// 使用 from_utf8 方法
pub fn fix5() {
    let mut s = String::new();
    //__;
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    //let s1 = __;
    let s1 = String::from_utf8(v).unwrap();


    assert_eq!(s, s1);

    println!("Success!")
}

// 如果 String 的当前容量足够，那么添加字符将不会导致新的内存分配
// 修改下面的代码以打印如下内容:
// 25
// 25
// 25
// 循环中不会发生任何内存分配
pub fn fix6() {
    //let mut s = String::new();
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}

pub fn fix7() {
    let story = String::from("Rust By Practice");

    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);

    // let ptr = story.__();
    // let len = story.__();
    // let capacity = story.__();

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    //let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}
