// TODO: 一、特征
#![allow(unused)]
use std::fmt;
use std::{fmt::{Display, Debug}, ops::Add, process::Output};
fn main () {
    #[derive(Debug)]
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
    /* pub trait Summary {
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
    {} */
    /* #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> {
        x: T,
        y: T
    }
    impl<T: Add<T, Output = T>> Add for Point<T> {
        type Output = Point<T>;
        fn add(self, p: Point<T>) -> Point<T> {
            Point { x: self.x + p.x, y: self.y + p.y }
        }
    }
    fn add<T: Add<T, Output = T>> (a: T, b: T) -> T {
        a + b
    }
    let p1 = Point{ x: 1.1f32, y: 1.1f32};
    let p2 = Point{ x: 2.1f32, y: 2.1f32};
    print!("{:?}", add(p1, p2));
    let p3 = Point{ x: 1i32, y: 1i32};
    let p4 = Point{ x: 2i32, y: 2i32};
    print!("{:?}", add(p3, p4)); */

    /* #[derive(Debug,PartialEq)]
    enum FileState {
        Open,
        Closed,
    }

    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
        state: FileState,
    }
    impl Display for FileState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                FileState::Open => write!(f, "OPEN"),
                FileState::Closed => write!(f, "CLOSED"),
            }
        }
    }
    impl Display for File {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<{} ({})>", self.name, self.state)
        }
    }

    impl File {
        fn new(name: &str) -> File {
            File {
                name: String::from(name),
                data: Vec::new(),
                state: FileState::Closed,
            }
        }
    }
    let f6 = File::new("f6.txt");
    //...
    println!("{:?}", f6);
    println!("{}", f6); */

    // TODO: 二、特征对象
    /*  // 传统方式编写
    #[derive(Debug)]
    enum UiObject {
        Button,
        SelectBox
    }
    let objects = [
        UiObject::Button,
        UiObject::SelectBox
    ];
    for o in objects {
        draw(o);
    }
    fn draw (o: UiObject) {
        println!("{:?}", o);
    } */
    // 特征对象编写
    pub trait Draw {
        fn draw(&self);
    }
    #[derive(Debug)]
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("{:?}", self);
        }
    }
    #[derive(Debug)]
    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("{:?}", &self);
        }
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}