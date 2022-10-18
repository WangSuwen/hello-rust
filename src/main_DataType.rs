// 数据类型
fn main() {
    /* let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}"); */
    let tup = (10, 1.2, 500); // tuple 元组
    let (x, _y, _z) = tup; // 解构
    let yy = tup.1;
    println!("The value of x is {x}, yy is {yy}");
}