//  结构体: 由多种类型组成的复合类型
// 添加调试特性,使得结构体可以使用{:?}占位符被打印
#[derive(Debug)]
struct User {
    name: String,
    active: bool,
    email: String,
    age: u16,
}

// 如果结构体中使用了引用,就必须加上声明周期,否则会报错
struct User2<'a> {
    name: &'a str,
    active: bool,
    email: String,
    age: u16,
}
pub fn eg1() {
    // 初始化:初始化结构体时每个字段都需要初始化,顺序不必按照定义的顺序来初始化
    let mut u = User {
        name: String::from("Tom"),
        active: true,
        age: 18,
        email: String::from("tom@gmail.com"),
    };
    // 访问元素
    println!("name:{},age:{}", u.name, u.age);
    // 修改元素
    // 结构体需要是可变的才能改变结构体的元素,不支持将结构体的某个元素标记为可变
    u.age = 21;
    println!("age:{}", u.age);
    let u2 = createUser(String::from("Jerry"), String::from("jerry@163.com"));
    println!("u2 name:{}", u2.name);
    // 根据已有的结构体创建新的结构体,
    // ..u2必须在末尾使用
    let u3 = User {
        name: String::from("July"),
        ..u2
    };
    println!("u3 name:{},age:{}", u3.name, u3.age);
    // 赋值时可能会导致已有结构体中字段发生所有权转移,导致已有结构体对应字段不再能使用
    let u4 = User {
        name: u3.name, // String类型,发生结构体转移,u3.name不可再被使用
        email: String::from("new email"),
        age: u3.age, // 有Copy特性,不影响
        active: false,
    };
    println!("u3 age:{}", u3.age);
    println!("u4 name:{},email:{}", u4.name, u4.email);
}

fn createUser(name: String, email: String) -> User {
    // 如果函数参数与结构体字段名称相同,可以使用缩略方式进行初始化
    User {
        name,
        email,
        active: true,
        age: 12,
    }
}

// 元组结构体:结构体必须要有名称,结构体字段可以没有名称,这种结构类似于元组,因此被称为元组结构体
// 使用场景:希望结构体有一个整体名称,但是不关注结构体字段名称,如坐标点,颜色配置
struct Color(i32, i32, i32);
pub fn eg2() {
    let black = Color(0, 0, 0);
}

// 单元结构体:没有任何字段与属性,只关心它的行为
struct AlwaysEqual;
