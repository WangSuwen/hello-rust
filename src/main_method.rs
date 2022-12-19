// 方法
fn main () {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32 
    }
    /* impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {  // TODO: 返回大写 Self 
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
    println!("rectangle`s width is {}", Rectangle::width(&rect2));  // 结构体直接调用内部方法 */

    /* 2、关联函数
        定义在 impl 中且没有 self 的函数被称之为【关联函数】，不可通过 结构体实例.函数名() 方式调用，只能通过 `::`方式调用。
        定义在 impl 中且有 self 的函数称之为 【方法】， 通过 结构体实例.方法名() 方式调用
    */
    /* impl Rectangle {
        fn new(w: u32, h: u32) -> Rectangle {
            Rectangle { width: w, height: h }
        }
    }
    println!("rectangle is: {:?}", Rectangle::new(12, 30)); */

}