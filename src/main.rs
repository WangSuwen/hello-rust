// 闭包


fn main () {
    // 1、闭包的声明方式
    /* |param1, param2, ...| {
        语句1;
        语句2;
        返回表达式
    } */

    // 2、闭包实现
    /* fn workout (intensity: u32, random_number: u32) {
        let action = || {
            println!("muuuuuu...");
            thread::sleep(Duration::from_secs(2));
            intensity
        };

        if intensity < 25 {
            println!("做 {} 个俯卧撑", action());
            println!("再做 {} 个伏地挺身", action());
        } else if random_number == 3 {
            println!("今天休息！！");
        } else {
            println!("跑步 {} 分钟", action());
        }
    }

    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number); */

    // 3、 结构体中的闭包
    /* struct Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy,
    {
        query: T,
        value: Option<E>
    }

    impl <T, E> Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy,
    {
        fn new(query: T) -> Cacher<T, E> {
            Cacher { query, value: None }
        }
        fn value(&mut self, arg: E) -> E {
            match self.value {
                Some(v) => v,
                None => {
                    let val = (self.query)(arg);
                    self.value = Some(val);
                    val
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x);
    println!("{}", cacher.value(12));
    let mut cacher = Cacher::new(|x| x);
    println!("{}", cacher.value(22)); */

    // 4.1、 闭包 可以捕获作用域中的值
    // TODO: 注意：闭包在捕获环境中值时，会额外开辟内存空间来存储此变量，因此会对内存造成一定的负担
    /* let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));

    // 4.2、 普通函数却不可以
    // fn equal_to_x_fn(z: usize) -> bool {
    //     z == x // 报错： can't capture dynamic environment in a fn item
    // }
    // assert!(equal_to_x_fn(y)); */

    // 5、三种Fn特征  闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种
    //  5.1 、  FnOnce -> 获取被捕获变量的所有权
    fn fn_once<F> (func: F)
        where F: FnOnce(usize) -> bool + Copy,
    {
        println!("{}", func(3));  // 此处发生了所有权转移，导致下面代码报错，解决方法是给 F 实现 Copy trait
        println!("{}", func(4));
    }
    
    // 5.2、 FnMut -> 获取被捕获变量的可变引用
   /*  let mut s = String::new();

    let mut update_string = |str| s.push_str(str); // 这里会报错，提示 因为使用了 mut s ,所以 update_string 函数也应该声明称 mut类型。
    update_string("Hello");
    println!("{:?}", s); */

    // 5.3、 Fn -> 获取被捕获变量的不可变引用
    /* let mut s = String::new();
    let update_string = |str| s.push_str(str); 
    exec(update_string);
    
    fn exec<'a, F: Fn(&'a str)>(mut f: F) {
        f("hello")
    } */
    
    // 上面写法会报错，是因为 闭包实现了 FnMut 特征（对 s 采用了可变借用），但是我们在 exec中强制他实现 Fn 特征，所以报错。
    // 修改如下：
    // 1.闭包中使用的是 不可变借用 s，所以，闭包默认实现 Fn 特征。
    let s = "Hello".to_string();
    let update_string = |str| println!("{} {}", s, str);
    exec(update_string);
    fn exec<F: Fn(String)> (f: F) {
        f("World!".to_string());
    }

}