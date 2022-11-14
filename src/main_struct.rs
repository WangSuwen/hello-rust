// 结构体
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i64
}

struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    // 实例化结构体
    /* let mut user = User {
        username: String::from("Rust"),
        email: String::from("rust@rust.com"),
        active: true,
        sign_in_count: 20
    };
    // 修改 可变 结构体某个属性的值
    user.username = String::from("Diana");
    println!("user`s username is {}", user.username);

    /*
     * 解构赋值--根据老对象创建新对象
     * 解构赋值时，原 结构体的没实现Copy方法的属性，会发生所有权转移的现象
     */
    let user2 = User {
        email: String::from("user2@mail.com"),
        ..user
    };
    println!("user2`s email is {}", user2.email);


    // 函数构建一个结构体实例
    let user1 =  build_user(String::from("张三"), String::from("张三@rust.com"));
    println!("user1`s name is {}", user1.username); */

    // 元组结构体 + 格式化输出结构体
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black is {:?} \norigin is {:#?}", black, origin);

    // dbg! 它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，当然还有我们需要的表达式的求值结果。除此之外，它最终还会把表达式值的所有权返回！
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

}

fn build_user (username: String, email: String) -> User {
    let user = User {
        username,
        email,
        active: true,
        sign_in_count: 32
    };
    user
}