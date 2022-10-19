// 数据类型

fn main() {
    /* let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}"); */

    // tuple 元组
    /* let tup = (10, 1.2, 500);
    let (x, _y, _z) = tup; // 解构
    let yy = tup.1;
    println!("The value of x is {x}, yy is {yy}");

    let x: (i32, f64, u8) = (500, 6.4, 1); // TODO: 元组可以每个元素的类型不同 */

    // Array
    let a = [1, 2, 3, 4, 5]; // 动态判断数组元素的类型
    let b = [3; 5]; // [3, 3, 3, 3, 3]
    let c: [i32; 5] = [5, 5, 5, 5, 5]; // TODO: 数组的每个元素类型必须相同
    println!("Array: a - {:?}", a);
    println!("Array: b - {:?}", b);
    println!("Array: c - {:?}", c);
}
