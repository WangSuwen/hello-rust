// 方法
fn main () {
    struct Rectangle {
        width: u32,
        height: u32 
    }
    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }
        fn area(&self) -> u32 {
            self.width * self.height
        }
        pub fn width(&self) -> u32 {
            self.width
        }
    }

    // let rect1 = Rectangle { width: 12, height: 25 };
    // println!("Rectangle`s area is {}", rect1.area());
    
    let rect2 = Rectangle::new(32, 16);
    // println!("rectangle`s width is {}", rect2.width());  // 结构体的实例 调用内部方法
    println!("rectangle`s width is {}", Rectangle::width(&rect2));  // 结构体直接调用内部方法
}