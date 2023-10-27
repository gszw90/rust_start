// 枚举: 允许列举可能的成员来定义的一个枚举类型
// 枚举类型是一个类型,它包含所有可能的枚举成员,而枚举值是该类型中具体某个成员的实例
#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
// 将数据信息关联到枚举值上
// 同一个枚举类型下的枚举值可以有不同的类型
#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(u8),
}

pub fn eg1() {
    // 创建枚举值
    let heart = PokerSuit::Hearts;
    println!("heart:{:?}", heart);
    // 将类型与枚举值关联
    let c1 = PokerCard::Hearts(3);
    let c2 = PokerCard::Diamonds('A');
    println!("c1:{:?}", c1);
    println!("c2:{:?}", c2);
}

// option 枚举用于处理空值,它有两个值,一个表示有值:Some(T),一个表示没有值:None
//       其中T是泛型参数,表示枚成员的数据类型
// enum Option<T>{
//      Some(T),
//      None,
// }
pub fn eg2() {
    let num = Some(233);
    let some_string = Some("a string");
    // 单独的None需要指定数据类型,否则编译器无法知道对应Some所对应的类型
    let some_none: Option<i32> = None;
    println!("{:?},{:?},{:?}", num, some_string, some_none);
}
