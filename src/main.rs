// 关联类型 + 调用同名方法 + 特征约束
/* use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
fn main () {
    assert_eq!(Point { x: 1, y: 2 } + Point { x: 2, y: 2}, Point { x: 3, y: 3 });
} */

// 调用同名方法
/* 
trait Polit {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}
struct Human;

impl Polit for Human {
    fn fly(&self) {
        println!("Polit flying");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying")
    }
}

impl Human {
    fn fly(&self) {
        println!("Human flying");
    }
}

fn main () {
    let person = Human;
    person.fly(); // Human flying  调用 Human 类型自身的 fly 方法
    Polit::fly(&person);  // Polit flying  调用特征Polit上的 fly 方法
    Wizard::fly(&person); // Wizard flying
} */

// 特征约束
use std::fmt;
trait OutlinePrint:fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn main () {
    let point = Point {
        x: 12,
        y: 12
    };
    point.outline_print();
}