// 注释 + 文档

// 引入自定义的依赖库，以 【包根】为始 `hello_rust`
use hello_rust::kinds::PrimaryColor; 

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