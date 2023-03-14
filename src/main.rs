use std::fmt::Debug;

// 生命周期
fn main() {
    // 1、 悬垂指针， 下面代码中，x 在 } 之后就已经被释放了，然后 println! 中的r引用x时，引用了个寂寞，这就是所谓的【悬垂指针】
    /* let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r); */

    // 2、生命周期的应用
    /* let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 如果不加生命周期的话，会报错
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    } */

    /* fn longest<'a>(x: &str, y: &str) -> String {
        let result = String::from("Hello, world!");
        // result.as_str()  // 这里函数要求返回一个生命周期为'a的引用【 &'a str 】，但是声明周期在 } 处已结束，会导致悬垂引用，所以编译不过
        result // 我们可以返回 String 的所有权，这样就不会有生命周期的问题了，但是需要将函数返回值改成 String
    }
    println!("{}", longest("", "")); */

    /* #[derive(Debug)]
    struct ImportantExcept<'a> {
        part: &'a str
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // println!("{}", first_sentence);
    let i = ImportantExcept{
        part: first_sentence
    };
    println!("{:?}", i.part); */

    /* use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    } */

    // 3、方法中的生命周期
   /*  struct ImportantExcept<'a> {
        part: &'a str,
    } */

    // 1）、不显示的声明生命周期
    /* impl<'a> ImportantExcept<'a> {
        fn announce_and_retur_part(&self, announcement: &str) -> &str {
            println!("Announcement: {}", announcement);
            self.part
        }
    } */
    // 2）、显示的声明生命周期
    /* impl <'a: 'b, 'b> ImportantExcept<'a> {
        fn announce_and_retur_part(&'a self, announcement: &'b str) -> &'b str {
            println!("Announcement: {}", announcement);
            self.part // 返回值的生命周期是 'b， 但是这里实际上是 self的生命周期 'a, 所以需要在impl中加上 'a: 'b,来表示 生命周期 a > b
        }
    } */
    // 3）、另一种显示声明的方法
    /* impl <'a> ImportantExcept<'a> {
        fn announce_and_retur_part<'b>(&'a self, announcement: &'b str) -> &'b str 
        where 'a: 'b,
        {
            println!("Announcement: {}", announcement);
            self.part
        }
    }

    let import = ImportantExcept{ part: "import" };
    import.announce_and_retur_part("123443211234"); */

    // 4、&'static 与 T: 'static 的区别
    // 4.1、 &'static
    /* use std::{slice::from_raw_parts, str::from_utf8_unchecked};

    fn get_memory_location() -> (usize, usize) {
        let string = "Hello world";
        let pointer = string.as_ptr() as usize;
        let length = string.len();
        (pointer, length)
    }

    fn get_str_at_location(pointer: usize, length: usize) -> &'static str{
        unsafe {
            from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
        }
    }
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!("The {} bytes at 0X{:X} stored: {}", length, pointer, message); */

    // 4.2、 T: 'static
    /* fn print_it<T: Debug + 'static>(input: &T) {
        println!("'static value passed in is: {:?}", input);
    }
    let i = 12;
    print_it(&i);

    let closure_slision = |x: &i32| -> &i32 { x }; // 这里是闭包 closure */
    // 4.3、 再借用 reborrow
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }
    impl Point {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }

    let mut p = Point{ x: 0, y: 0};
    let r = &mut p;
    let rr = &*r; // 此处为再借用，不会破坏借用规则，但是不能在其生命周期内使用原始的借用 r。
    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}