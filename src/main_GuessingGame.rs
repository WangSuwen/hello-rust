/* use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
} */
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    /* let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y); */
    let secret_number = rand::thread_rng().gen_range(1..=3);
    println!("The secret number is: {secret_number}");
    println!("Guess the number!");
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

        println!("You guessed: {guess}");
        
        // 将 guess 转换成 int 类型
        // let guess: u32 = guess.trim().parse().expect("please input a number !!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small !!"),
            Ordering::Greater => println!("too big !!"),
            Ordering::Equal => {
                println!("you win !!");
                break;
            }
        }
    }
}
