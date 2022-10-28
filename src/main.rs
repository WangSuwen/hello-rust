struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    /* let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com"); */
    let user = build_user(String::from("1002853070@qq.com"), String::from("Diana"));
    println!("User is");
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        email,
        username
    }
}