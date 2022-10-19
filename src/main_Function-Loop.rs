fn main () {
    another_function();
}

fn another_function() {
    // scope block 作用于块
    /* let y = {
        let x = 10;
        x + 1
    };
    println!("scope block y is {y}"); 
    println!("scope block y is {y}"); 
    */

    // 1、Loop 
   /*  let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}"); */

    // 2、Label loop, 注意：label 的变量名之前要加英文的分号： '  。
    /* let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;  // 结束指定 label 的 loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); */

    // 3、for 循环，快速循环一个范围
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    
}