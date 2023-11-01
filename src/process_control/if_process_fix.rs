pub fn fix1() {
    let n = 5;

    // if n < 0 {
    //     println!("{} is negative", n);
    // } __ n > 0 {
    //     println!("{} is positive", n);
    // } __ {
    //     println!("{} is zero", n);
    // }
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

pub fn fix2() {
    let n = 5;

    // let big_n =
    //     if n < 10 && n > -10 {
    //         println!(" 数字太小，先增加 10 倍再说");
    //
    //         10 * n
    //     } else {
    //         println!("数字太大，我们得让它减半");
    //
    //         n / 2.0 ;
    //     }
    let big_n = if n < 10 && n > -10 {
        println!(" 数字太小，先增加 10 倍再说");
        10 * n
    } else {
        println!("数字太大，我们得让它减半");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

pub fn fix3() {
    // for n in 1..=100 { // 修改此行，让代码工作
    //     if n == 100 {
    //         panic!("NEVER LET THIS RUN")
    //     }
    // }
    for n in 1..100 {
        // 修改此行，让代码工作
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }
}

pub fn fix4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    // for name in names {
    //     // do something with name...
    // }
    for name in &names {
        // do something with name...
        println!("{name}");
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // numbers中的元素实现了 Copy，因此无需转移所有权
    for n in numbers {
        // do something with name...
        println!("{n}");
    }

    println!("{:?}", numbers);
}

pub fn fix5() {
    let a = [4, 3, 2, 1];

    // 通过索引和值的方式迭代数组 `a`
    // for (i,v) in a.__ {
    //     println!("第{}个元素是{}",i+1,v);
    // }
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

pub fn fix6() {
    // 一个计数值
    let mut n = 1;

    // 当条件为真时，不停的循环
    // while n __ 10 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    //     __;
    // }
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n = n + 1;
    }

    println!("n 的值是 {}, 循环结束", n);
}

pub fn fix7() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            //__
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);
}

pub fn fix8() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            //__;
            continue;
        }

        //__
        break;
    }

    assert_eq!(n, 66);
}

pub fn fix9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过当此循环的剩余代码
            //__;
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            //__;
            break;
        }
    }

    assert_eq!(count, 5);
}
// loop 是一个表达式，因此我们可以配合 break 来返回一个值
pub fn fix10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            //__;
            break 20;
        }
    };

    assert_eq!(result, 20);
}

// 当有多层循环时，你可以使用 continue 或 break 来控制外层的循环。
// 要实现这一点，外部的循环必须拥有一个标签 'label, 然后在 break 或 continue 时指定该标签
pub fn fix11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // 这只会跳出 inner1 循环
                break 'inner1; // 这里使用 `break` 也是一样的
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    // assert!(count == __)
    assert!(count == 30)
}
