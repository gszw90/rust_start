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
