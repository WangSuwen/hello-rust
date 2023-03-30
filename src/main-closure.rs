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

    /**
     * 5、 三种 Fn 特征
     * 在 Rust 中，Fn、FnMut 和 FnOnce 都是闭包（Closure）的特征标识符（Trait），用于定义闭包的特性。

        Fn 表示一个不可变的借用闭包，它可以捕获周围的值，但不能修改它们。通常使用 Fn 定义只需要读取变量的闭包。

        FnMut 表示一个可变的借用闭包，它可以捕获周围的值并修改它们。通常使用 FnMut 定义需要修改变量的闭包。

        FnOnce 表示一个获取所有权的闭包，它可以捕获周围的值并使用它们，但是它不能被多次调用，因为它会消耗捕获的变量。通常使用 FnOnce 定义只需要使用变量一次的闭包，或者将闭包移动到新的线程中。

        总的来说，这些特征标识符用于区分闭包所需的变量访问级别和所有权。具体使用哪个特征标识符取决于闭包所需要的变量和使用场景。
     */
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
    /* let s = "Hello".to_string();
    let update_string = |str| println!("{} {}", s, str);
    exec(update_string);
    fn exec<F: Fn(String)> (f: F) {
        f("World!".to_string());
    } */

    // 6、闭包作为函数返回值

    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        // move |x| x + num
        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }
    let f = factory(3);
    assert_eq!(f(0), 8);
}