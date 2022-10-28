// references an  borrowing

// references
/* fn main() {
    let s = String::from("asdfdaffdks");
    let len = get_length(&s);
    println!("{s}`s length is {len}");
}
fn get_length(s: &String) -> usize {
    s.len()
} */

// mutable references
/* fn main () {
    let mut s = String::from("Hello");
    println!("s is {s}");
    change_string(&mut s);
    println!("new s is {s}");
}
fn change_string (some_string: &mut String) {
    some_string.push_str(", World!");
} */

fn main() {
    let reference_to_nothing = dangle();
    println!("reference_to_nothing is {reference_to_nothing}");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}