fn main () {
    another_function();
}

fn another_function() {
    // scope block 作用于块
    /* let y = {
        let x = 10;
        x + 1
    };
    println!("scope block y is {y}"); 
    println!("scope block y is {y}"); 
    */

    // Loop 
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");

    
}