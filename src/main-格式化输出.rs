// 格式化输出

/**
 *  print! 将格式化文本输出到标准输出，不带换行符
    println! 同上，但是在行的末尾添加换行符
    format! 将格式化文本输出到 String 字符串
 * 
 * 
 *  {} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
    {:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景
    {:#?} 与 {:?} 几乎一样，唯一的区别在于它能更优美地输出内容：
 */

fn main() {
    println!("{:04}", 42);
    eprintln!("Error: Here is an error!!!");

    // 1、为自定义类型实现 Display 特征
    /* struct Person {
        name: String,
        age: u8,
    }
    use std::fmt;
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
                self.name, self.age
            )
        }
    }
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p); */

    // 2、在格式化字符串时捕获环境中的值（Rust 1.58 新增）
    fn get_person() -> String {
        String::from("sunface")
    }

    let p = get_person();
    println!("Hello, {}!", p);                // implicit position
    println!("Hello, {0}!", p);               // explicit index
    println!("Hello, {person}!", person = p);
    println!("Hello, {p}!");
}