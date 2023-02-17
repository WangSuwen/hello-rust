// 引用 与 借用

fn main() {
    let x = 5;
    let y = &x;  // y 是 x 的引用

    println!("*y is {}", *y); // *y 表示解引用
    println!("y is {}", y);

    let string1 = String::from("Hello, world!");
    println!("&string1 is {}", &string1); // 在 string1 前面加一个 & 符号，表示这里使用的是 string1 的引用，没有他的所有权
}