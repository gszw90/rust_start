/**
堆与栈:
      栈按照顺序储存值并按照相反的顺序取出,即先进后出.
      栈中的所有数据都必须占用已知且固定大小的内存空间，假设数据大小是未知的，那么在取出数据时，你将无法取到你想要的数据
      对于大小未知或者可能变化的数据，我们需要将它存储在堆上

所有权的规则:
     Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
     一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
     当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
 */

pub fn my_str() {
    // s 是被硬编码进程序里的字符串值（类型为 &str ）,该方法部适用的场景:
    //  字符串字面值是不可变的，因为被硬编码到程序代码中
    //  并非所有字符串的值都能在编写代码时得知
    let s = "hello";
    // rust提供的动态字符类型String,该类型被分配到堆上,能动态伸缩,能储存编译时大小未知的文本
    let mut s2 = String::from("hello String");
    s2.push_str(" !");
    println!("{}", s2);
}

// 所有权转移
pub fn scope_transform() {
    // 对于基本数据类型,赋值采用自动拷贝的方式来实现,不会存在所有权转移
    let x = 5; // 将5绑定到变量x上
    let y = x; // 将x的值赋予y,x还是可以用的
    println!("x = {},y={}", x, y);

    // 对于复杂类型,赋值会产生所有权转移,使得之前的变量失效
    let s1 = String::from("hello");
    let s2 = s1; // 将s1值赋值给s2,s1的所有权转移给s2,s1失效
    let s3 = s2.clone(); // 当同时需要s2,s3时,采用clone来深拷贝,完整复制s2的值,s2也不会发生所有权转移

    // 如果一个类型有copy特征,那么旧值在赋予给新值后,旧值仍然是可以使用的
    // copy的通用规则:任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的
    // 可以copy的类型有:
    //    所有整数与浮点数类型
    //    布尔类型
    //    字符类型
    //    包含的类型都是可以copy的元组
    //    不可变引用 &T
}

// 函数的传值也会伴随着所有权的转移
pub fn fn_vars() {
    let s = String::from("hello");
    take_ownership(s); // s的所有权转移到take_ownership函数里,s失效

    let i = 32;
    make_copy(i); // i是可以copy的,所有权没有转移,后面可以继续使用
    println!("after make_copy() i=>{}", i);

    let s2 = give_ownership();
    let s3 = take_and_give_ownership(s2); // s2的所有权被转移给take_and_give_ownership,take_and_give_ownership又将所有权返回给s3
}

fn take_ownership(s: String) {
    println!("s => {}", s)
}

fn make_copy(i: i32) {
    println!("i => {}", i);
}

fn give_ownership() -> String {
    let s = String::from("give ownership");
    s // 返回s
}

fn take_and_give_ownership(s: String) -> String {
    s // 返回s
}
