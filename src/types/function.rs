// 函数
// 要点: 函数名与变量名使用蛇形命名法
//      函数的位置可以任意放置,只要定义了即可
//      每个函数参数都需要标注类型

fn add(x: i32, y: i32) -> i32 {
    // 函数的返回值就是函数体最后一条表达式的返回值,也可以使用return提前返回
    x + y
}

pub fn max(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    }
    y
}
