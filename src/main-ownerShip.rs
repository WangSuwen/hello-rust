// ownerShip

fn main () {
    /* let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("The s is {}", s); */


    /* let s = String::from("Hello");
    takes_ownership(s);
    // println!("s is: {s}"); // borrow of moved value: `s`;  value borrowed here after move
    let x = 5;
    makes_copy(x); */

    // 2、变量被函数用过，它的归属权就会丢失
    let _s1 = gives_ownership();         // gives_ownership moves its return value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let _s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3

}

/* fn takes_ownership (some_string: String) {
    println!("{}", some_string);
}
fn makes_copy (some_integer: i32) {
    println!("{}", some_integer);
} */

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}