pub fn eg1() {
    // 使用if来做条件判断
    let condition = true;
    if condition {
        println!("it's true");
    } else {
        println!("it's false");
    }
    // 使用if来赋值
    let num = if condition { 5 } else { 6 };
    println!("number eq {}", num);
}

pub fn eg2() {
    // for 循环
    // 一般循环时使用的是集合引用,否则的话所有权会转移到循环内部,导致后面不能再使用该集合
    for v in 1..5 {
        println!("num=>{v}");
    }
    for v in 1..5 {
        // 跳过2
        if v == 2 {
            continue;
        }
        println!("num=>{v}");
    }
    for v in 1..5 {
        // 只循环到2
        if v == 2 {
            break;
        }
        println!("num=>{v}");
    }
    // while: 有条件的循环
    let mut n = 5;
    while n >= 0 {
        println!("while for {n}");
        n = n - 1;
    }
    // loop:无条件的循环,需要用break跳出循环
    let mut n = 5;
    loop {
        println!("just loop {n}");
        if n <= 1 {
            break;
        }
        n = n - 1;
    }
    // 跳出多重循环
    'outLoop: for i in 1..4 {
        'inLoop: for j in 1..4 {
            println!("({i},{j})");
            if i == 2 && j == 2 {
                println!("break out");
                break 'outLoop;
            }
            if i == 3 && j == 3 {
                println!("break in");
                break 'inLoop;
            }
        }
    }
}
