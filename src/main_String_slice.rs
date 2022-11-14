// String - 切片（slice）
/**
 * String: 是一个是一个指针类型
 * &str: 是一个值类型，编译器已经知道了他的具体大小，并直接编译到可执行文件中
 */

/* fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
} */
fn main() {
    // let ss = String::from("通过：String::from得到的字符串");
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    // 如果不想转义，在前面加 r 即可
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    // 操作Unicode 字符
    for c in "中国人".chars() {
        println!("{}", c);
    }
    // 操作 字节
    for b in "中国人".bytes() {
        println!("{}", b);
    }
}