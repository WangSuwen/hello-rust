// 动态数组 Vector

fn main () {

   /*  // 1、不指定类型时，第一次push后，会自动确定数组的类型
    let mut v = Vec::new();
    v.push(1);

    // 2、显示的声明数组内元素的类型
    let mut v1: Vec<String> = Vec::new();
    v1.push(String::from("张三"));

    // 3、创建指定长度的数组
    let mut v2: Vec<i32> = Vec::with_capacity(6);
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    v2.push(5);
    v2.push(6);
    v2.push(7);
    println!("v2 的长度： {}", v2.len());
    
    // 4、 使用宏 vec!
    let v3 = vec![1, 2, 3, 4];
    println!("v3 的长度： {}", v3.len()); */

    /*
    5、从Vector 中读取元素 
        TODO: 注意：  
            1、v[index] 方式会出现下标越界问题，v.get(index)则不会，内部处理的会返回None；
            2、不可变借用和可变借用的顺序问题

    */
    /* let v4 = vec![1, 2, 3, 4, 5, 6];
    let third = &v4[2];
    println!("v4 的第三个元素是： {}", third);
    match v4.get(1) {
        Some(match_val) if match_val < &3 => {
            println!("match 匹配到值 {} 小于3", match_val);
        },
        Some(match_val @ 3..=5) => {
            println!("match 在 [3:5]的范围内: {}", match_val);
        },
        Some(match_val) => {
            println!("match 匹配到值了： {}", match_val);
        },
        None => println!("match 了个寂寞!")
    } */

    // 6、遍历Vector中的元素
    /* let v = vec![1, 2, 3, 4, 5, 6];
    for item in v {
        println!("item is {}", item);
    }
    let mut v1 = vec![1, 2, 3, 4, 5];
    for item in &mut v1 {
        *item += 1;
    }
    println!("v1 is {:?}", v1);
    */
    // 7、存储不同类型的元素
    // 7-1、 通过枚举来实现
    /* 
    enum IpAddr {
        V4(String),
        V6(String)
    }
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];
    for ip in v {
        show_addr(ip);
    }
    fn show_addr(ip:IpAddr) {
        println!("ip is {:?}", ip);
    } */
    // 7-2、 通过特征对象 + 结构体实现
    trait IpAddr {
        fn display(&self);
    }
    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("IpV4 is {}", &self.0);
            println!("IpV4 is {}", self.0);
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("IpV6 is {:?}", self.0);
        }
    }
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string()))
    ];
    for ip in v {
        ip.display();
    }
}