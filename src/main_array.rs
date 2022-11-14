/*
    数组 - array
    1、数组内各元素的类型必须一致
*/
#![allow(unused)]

use std::io;
fn main () {
    // 1、array 类型数组
    let a = [1, 2, 3, 4, 5];
    // println!("{:?}", a);
    let b: [i32; 5] = [1,2,3,4,5];  // 显式声明 数组元素类型和长度
    let c = [3; 5]; // 数组内有 5 个 3
    println!("c is: {:?}", c);

   /*  let mut index = String::new();
    println!("请输入要查询的索引：");
    io::stdin().read_line(&mut index).expect("读取控制台失败！");
    let index: usize = index.trim().parse().expect("索引值不是数字！");
    let elem = a[index];
    println!("The value of the element at index {} is: {}", index, elem) */

    /* // 2、数组元素为非基本类型
    let array: [String; 8] = core::array::from_fn(|i| String::from("I`m a String!"));
    println!("array<String> is {:#?}", array);

    // 3、数组切片
    let slice = &a[1..3];
    println!("数组切片 [slice] 是 {:?}, 原数组a是：{:?}", slice, a); */
    // 编译器自动推导出one的类型

    let one= [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}