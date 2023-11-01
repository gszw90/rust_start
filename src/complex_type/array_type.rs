// 数组: 将多个类型相同的元素组合在一起,就是一个数据,这里的数组是长度固定的数组,还有一种长度不固定的动态数组Vec
// 长度固定的数组大小已知,储存在栈上,动态数组大小未知,储存在堆上
// 数组的特性有: 长度固定,元素类型相同,依次线性排列
pub fn eg1() {
    // 创建数组
    // 编译器自动判断类型与长度
    let arr1 = [1, 2, 3, 4];
    // 声明类型与长度
    let arr2: [i32; 2] = [0, 1];
    // 让某个值出现n次,表示数组长度为4,每个元素都等于3
    let arr3 = [3; 4];
    // 等同于
    let arr3: [i32; 4] = [3, 3, 3, 3];
    // [type;num]这种创建方式对于复杂类型可能报错,因为复杂类型没有深拷贝,可以使用std::array::from_fn来实现
    let arr3: [String; 8] = std::array::from_fn(|_i| String::from("arr for String"));
    println!("{:#?}", arr3);

    // 访问元素
    println!("first: {}", arr2[0]); // 第一个元素
    println!("last: {}", arr2[1]);

    // 数组切片
    let s1 = &arr1[1..3]; // [2,3]
    let s2 = &arr1[1..=3]; // [2,3,4]
    println!("s1:{:?}", s1);
    println!("s2:{:?}", s2);
    // 数组长度
    println!("len:{}", arr1.len());
    // 遍历数据
    for v in &arr1 {
        println!("ref:{}", v);
    }
    for v in arr1.iter() {
        println!("iter:{}", v);
    }
    // 遍历数组与下标
    for (k, v) in arr1.iter().enumerate() {
        println!("{} => {}", k, v);
    }
}
