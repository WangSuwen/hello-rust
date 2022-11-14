// 枚举

#![allow(unused)]
/* #[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);
}

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
} */

/*
    任何类型的数据都可以放入枚举成员中
    Quit 没有任何关联数据
    Move 包含一个匿名结构体
    Write 包含一个 String 字符串
    ChangeColor 包含三个 i32
*/
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1, y:1};
    let m3 = Message::ChangeColor(255,255,0);
    println!("{:?}", m2);
}