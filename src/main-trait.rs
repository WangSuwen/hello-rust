// 特征
#![allow(unused)]
use std::fmt::{Display, Debug};
fn main () {
    pub struct Post {
        pub title: String,
        pub content: String,
        pub author: String
    }
    pub struct Weibo {
        pub title: String,
        pub content: String
    }
    let post = Post{
        title: "诛仙".to_string(),
        content: "我是文章内容".to_string(),
        author: String::from("萧鼎")
    };
    let weibo = Weibo {
        title: String::from("我是微博标题"),
        content: String::from("我是微博内容！！！")
    };
   /*  pub trait Summary {
        fn summarize(&self) -> String;
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("文章：《{}》的作者是：{}", self.title, self.author)
        }
    }
    println!("{}", post.summarize()); */
    // 2、 特征的默认实现
   /*  pub trait Summary {
        fn summarize(&self) -> String {
            String::from("Read more...")
        }
    }
    impl Summary for Post {}
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("微博：{}，的内容是：{}", self.title, self.content)
        }
    }
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    // 3、使用特征作为参数（实现了某个特征，才可作为函数的参数） TODO: 这是下面 4 中写法的语法糖而已
    fn some_fn (item: &impl Summary) {
        println!("特征作为函数的参数：{}", item.summarize());
    }
    some_fn(&weibo);
    */
    // 4、特征约束
    // 函数的多个入参需要保持类型一致，且都实现了Summary这个特征。
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub fn fn_name<T: Summary> (item1: &T, item2: &T) {}
    // 多重约束（参数实现多个特征）
    pub fn some_fn2 (item: &(impl Summary + Display)) {}
    // 或者：
    pub fn some_fn3<T: Summary + Display>(item: &T) {}

    // 5、where 约束
    // 当参数过多，且每个参数的约束也过多时，可以使用Where约束来解决
    fn some_function <T, U>(t: &T, u: &U)
        where 
            T: Display + Summary,
            U: Clone + Debug
    {}

}