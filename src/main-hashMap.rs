// HashMap
fn main() {
    use std::collections::HashMap;
    /* let term_list = vec![
        ("中国队".to_string(), 1),
        ("美国队".to_string(), 10),
        ("英国队".to_string(), 100)
    ];
    let term_map: HashMap<_, _> = term_list.into_iter().collect();
    println!("term_map is {:?}", term_map); */

    /* let name = String::from("Zhangsan");
    let age = 18;
    let mut boys = HashMap::new();
    boys.insert(&name, age);

    // std::mem::drop(name); // TODO: 因为上面有用到 name 的引用，但是这里将其从内存中移除了，所以会报错
    println!("因为过于无耻{:?}已被除名", boys); */

    // 2、 从map 中取值
    /* let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let color = String::from("Blue");
    let score = scores.get(&color);
    println!("Blue score is {:?}", score.unwrap());

    let color2 = String::from("Blue2");
    let score2 = scores.get(&color2);
    println!("Blue2 score is neq Blue ?  {:?}", score2.eq(&Option::None));

    // 遍历 map
    for item in &scores {
        println!("item is {:?}", item);
        println!("item`s key is {}, value is {}", item.0, item.1);
    }
    // 也可以解构item
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    } */

    // 3、更新 map 中的值
    /* let mut scores = HashMap::new();
    scores.insert("Blue", 12);

    // let old = scores.insert("Blue", 20);
    // assert_eq!(old, Some(20));
    // println!("scores is {:?} old is:  {:?}", scores, old);

    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5);
    println!("v is {:?}", v); */

    /* let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{:?}", map); */
    /* println!("{}", i8::MAX);
    let a = 366666666.1;
    println!("{}", a as i8); // TODO: 大类型转小类型，如果超了小类型范围，会自动转成小类型的最大值 */

    /* let mut val1 = [1.1, 2.2];
    let p1 = val1.as_mut_ptr();
    unsafe {
        println!("{:?}, {:?}", p1, *p1.add(0));
        println!("{}", *p1.add(1) + 1 as f64); // TODO: 根据指针取第二个值并加1
    } */

    // 使用try_into 进行类型转换时，可以捕获异常
    /* let b: i16 = 150;
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    println!("{}", b_); */

    // 强制类型转换
    /* trait Trait {}
    fn foo<X: Trait>(t: X) {}
    
    impl<'a> Trait for &'a mut i32 {}

    let t = &mut 0;
    foo(t); */
    
}