// 元组 tup
fn main() {
    let tup = (500, 6.4, 1);

    // 解构方式使用元组
    let (x, y, z) = tup;
    println!("The value of x is {}, y is: {}, z is {}", x, y, z);

    // 使用 . 的方式使用元组，索引从 0 开始
    let xx = tup.0;
    println!("xx is {}", xx);

    // 元组的使用示例
    let string = String::from("I like Rust");
    let (str_length, new_strr) = calculate_length(&string);
    println!("newStr ({})`s length is {}", new_strr, str_length);
}
fn calculate_length (s: &String) -> (usize, &String) {
    let length = s.len();
    (length, &s)
}