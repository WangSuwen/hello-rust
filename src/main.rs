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

    #[derive(Debug)]
    struct ImportantExcept<'a> {
        part: &'a str
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // println!("{}", first_sentence);
    let i = ImportantExcept{
        part: first_sentence
    };
    println!("{:?}", i.part);

}