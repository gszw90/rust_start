struct Person {
    name: String,
    age: u8,
    hobby: String,
}

// 对于结构体，我们必须为其中的每一个字段都指定具体的值
pub fn fix1() {
    let age = 30;
    // let p = Person {
    //     name: String::from("sunface"),
    //     age,
    // };

    let p = Person {
        name: String::from("sunface"),
        hobby: String::from("sing dance rap basketball"),
        age,
    };
}

// 单元结构体没有任何字段。
struct Unit;
trait SomeTrait {
    // ...定义一些行为
}
impl SomeTrait for Unit {}
pub fn fix2() {
    let u = Unit;
    do_something_with_unit2(u);
}
// 填空，让代码工作
//fn do_something_with_unit2(u: __) {   }
fn do_something_with_unit2(u: Unit) {}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
pub fn fix3() {
    // let v = Point(0, __, __);
    let v = Color(0, 127, 255);
    check_color3(v);
}
/*fn check_color3(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
}*/
fn check_color3(p: Color) {
    let Color(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

// 你可以在实例化一个结构体时将它整体标记为可变的，但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.
// 填空并修复错误，不要增加或移除代码行
struct Person4 {
    name: String,
    age: u8,
}
pub fn fix4() {
    let age = 18;
    // let p = Person4 {
    //     name: String::from("sunface"),
    //     age,
    // };
    let mut p = Person4 {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18?
    p.age = 30;

    // 填空
    p.name = String::from("sunfei");
}

// 使用结构体字段初始化缩略语法可以减少一些重复代码
struct Person5 {
    name: String,
    age: u8,
}
pub fn fix5() {}

fn build_person5(name: String, age: u8) -> Person5 {
    // Person {
    //     age,
    //     __
    // }
    Person5 { age, name }
}

// 使用结构体更新语法基于一个结构体实例来构造另一个
struct User6 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
pub fn fix6() {
    let u1 = User6 {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email6(u1);
}

fn set_email6(u: User6) -> User6 {
    // User6 {
    //     email: String::from("contact@im.dev"),
    //     __,
    // }
    User6 {
        email: String::from("contact@im.dev"),
        ..u
    }
}

// 使用 #[derive(Debug)] 让结构体变成可打印的
//#[__]
#[derive(Debug)]
struct Rectangle7 {
    width: u32,
    height: u32,
}
pub fn fix7() {
    let scale = 2;
    let rect1 = Rectangle7 {
        width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        height: 50,
    };

    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

    //println!(__, rect1); // 打印 debug 信息到标准输出 stdout
    println!("{:?}", rect1);
}

#[derive(Debug)]
struct File8 {
    name: String,
    data: String,
}
pub fn fix8() {
    let f = File8 {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = f.name;

    // 只能修改这一行
    // println!("{}, {}, {:?}",f.name, f.data, f);
    println!("{}, {}", _name, f.data);
}
