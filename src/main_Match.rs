// 模式匹配 -- match  
fn main () {
    /* enum Direction {
        East,
        South,
        West,
        North
    }
    let direc = Direction::North;
    match direc {
        Direction::East => println!("I`m east"),
        Direction::South | Direction::North => {
            println!("I`m south or north");
        },
        _ => println!("_____")
    } */
    /* enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r: {}, g: {}, b: 0)', 'b' has been ignored", r, g);
            }
        }
    } */

    /* // 2、 matches!   宏，只有一个模式匹配时用这个，省代码
    let foo = 'f';
    println!("matches! {}", matches!(foo, 'a'..='z' | 'A'..='Z'));

    let bar = Some(4);
    println!("Some(4) {}, bar is {:?}", matches!(bar, Some(x) if x > 5), bar);


    // 3、if let 匹配
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (), // 因为 match 匹配的穷尽性，必须这么写
    }
    // 相比较于上面这个写法，只有一个模式匹配时，也可用它
    if let Some(3) = v {
        println!("three");
    } */
    
    // 4、while let 模式匹配循环
    /* let mut arr = Vec::new();
    arr.push(1);
    arr.push(2);
    arr.push(3);

    while let Some(x) = arr.pop() {
        println!("arr pop x is {}", x);
    } */
    // TODO: let , for 和 match 都必须要求完全覆盖匹配， if let 可以只匹配一种模式

    /* let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y); */

    // 5、 单分支多模式
    /* let x = 2;
    match x {
        1 | 2 => {
            println!("matched 1 or 2, x is {}", x);
        },
        3 => println!("three"),
        _ => println!("default ...")
    } */
    // 6、通过序列 ..= 匹配值的范围
    /* let x = 5;
    match x {
        1..=5 => {
            println!("match one to five");
        },
        _ => println!("ignore others!")
    } */

    // 7、 @ 绑定
    /* enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        /**
         * 匹配到到第一个分支，并将id的值绑定给了 id_variable。
         */
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
    // 输出： Found an id in range: 5 */


    // 7.1 @ 绑定加解构
    struct Point {
        x: i32,
        y: i32,
    }
    fn main() {
        // 绑定新变量 `p`，同时对 `Point` 进行解构
        let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
        println!("x: {}, y: {}", px, py);
        println!("{:?}", p);
        let point = Point {x: 10, y: 5};
        if let p @ Point {x: 10, y} = point {
            println!("x is 10 and y is {} in {:?}", y, p);
        } else {
            println!("x was not 10 :(");
        }
    }
}