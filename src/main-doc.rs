// 注释 + 文档


fn main() {
    /// 文档注释
    /// add_one 是一个给参数加1的函数
    /// 
    /// # Example
    /// ```
    /// let age = add_one(17);
    /// println!("age is {}", age);
    /// ```
    /// 
    /// 
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    let s = String::from("asdf");
    println!("{}", s);

}