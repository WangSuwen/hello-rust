// 动态数组 Vector

fn main () {

    // 1、不指定类型时，第一次push后，会自动确定数组的类型
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
    println!("v3 的长度： {}", v3.len());

    // 5、从Vector 中读取元素 TODO: 注意： v[index] 方式会出现下标越界问题，v.get(index)则不会，内部处理的会返回None；
    let v4 = vec![1, 2, 3, 4, 5, 6];
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
    }

}