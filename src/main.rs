// 迭代器 Iterator


fn main () {
    /* let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val);
    } */
/* 
    // Rust 中, for 循环只是 Iterator 的语法糖
    // 将数组转换成 iterator 后，可以直接调用 next方法获取数组元素
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(4));
    assert_eq!(arr_iter.next(), None);
     */

    // 模拟实现 for 循环
    /* let values = vec![1, 2, 3];
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => {
                    println!("{}", x);
                },
                None => break,
            }
        }
    };
    println!("{:?}", result); */
    
    /* 
        into_iter, iter, iter_mut
        在之前的代码中，我们统一使用了 into_iter 的方式将数组转化为迭代器，除此之外，还有 iter 和 iter_mut，聪明的读者应该大概能猜到这三者的区别：

        into_iter 会夺走所有权
        iter 是借用
        iter_mut 是可变借用
        其实如果以后见多识广了，你会发现这种问题一眼就能看穿，into_ 之类的，都是拿走所有权，_mut 之类的都是可变借用，剩下的就是不可变借用。
        详见下方示例：
     */
    // 1、 into_inter 所有权发生转移
    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}", v);
    }
    // println!("{:?}", values); // 这里会报错，因为 values 在 .into_inter() 是发生了所有权转移
    
    // 2、 iter 不可变借用
    let values2 = vec![1, 2, 3];
    let values2_iter = values2.iter();
    // if let Some(v) = values2_iter.next() { println!("{}", v); }  // 这里会报错，因为 values2_inter 是不可变借用，调用 next 方法时，会变更其内部属性值，所以会报错
    println!("{:?}", values2);
    
    // 3、 iner_mut 可变借用
    let mut values3 = vec![1, 2, 3];
    let mut values3_iter_mut = values3.iter_mut();
    if let Some(x) = values3_iter_mut.next() {
        println!("{}", x); // 这里的x 是个可变引用
        *x = 0; // 这里通过  * 解引用，取得 x 指向的值，并修改它的值
    }
    println!("{:?}", values3);

}