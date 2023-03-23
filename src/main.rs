// 闭包

use std::{thread, time::Duration};


fn main () {
    // 1、闭包的声明方式
    /* |param1, param2, ...| {
        语句1;
        语句2;
        返回表达式
    } */

    // 2、闭包实现
    fn workout (intensity: u32, random_number: u32) {
        let action = || {
            println!("muuuuuu...");
            thread::sleep(Duration::from_secs(2));
            intensity
        };


        if intensity < 25 {
            println!("做 {} 个俯卧撑", action());
            println!("再做 {} 个伏地挺身", action());
        } else if random_number == 3 {
            println!("今天休息！！");
        } else {
            println!("跑步 {} 分钟", action());
        }
    }

    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
}