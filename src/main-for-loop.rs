// for 循环

fn main() {
    /* 
    // 1、数组先转迭代器再转枚举
    let a = [4, 3, 2, 1];
    for (ind, val) in a.iter().enumerate() {
        println!("第{}个元素是{}", ind, val);
    } */

   /*  
    // 1..4  表示： 1-3
    // 1..=4 表示： 1-4
   for i in 1..=4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    } */

    // ## 2、loop 循环 + 跳出循环
    let mut count = 1;
    loop {
        if count > 5 {
            break;
        }
        println!("loop times is {}", count);
        count += 1;
    }
    println!("loop is overed!");

    // loop 是一个表达式，所以可以返回值
    let mut c = 1;
    let result = loop {
        if c == 10 {
            break c * 20;
        }
        c += 1;
    };
    println!("result is {}", result);

}