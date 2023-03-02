// Box 学习
// 在 Rust 中，所有值默认都是栈分配的。通过创建 Box<T>，可以把值装箱（boxed）来使它在堆上分配。箱子（box，即 Box<T> 类型的实例）是一个TODO: 【智能指针】，指向堆分配的 T 类型的值

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 在堆上分配这个点（point），并返回一个指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // （所有的类型标注都不是必需的）
    // 栈分配的变量
    let mut point: Point = origin();
    point.x = 1.0;
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    // 堆分配的 rectangle（矩形）
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });

    // 函数的输出可以装箱
    let mut boxed_point: Box<Point> = Box::new(origin());
    boxed_point.x += 1.0;
    // 两层装箱
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes in the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in the stack", mem::size_of_val(&rectangle));
    // box 的宽度就是指针宽度
    println!("Boxed point occupies {} bytes in the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack", mem::size_of_val(&box_in_a_box));
    // 将包含在 `boxed_point` 中的数据复制到 `unboxed_point`
    let mut unboxed_point: Point = *boxed_point;
    unboxed_point.x += 1.0;
    println!("Unboxed point occupies {} bytes in the stack", mem::size_of_val(&unboxed_point));
    println!("unboxed_point is {:?}, boxed_point is {:?}, point is {:?}", unboxed_point, boxed_point, point);
}
