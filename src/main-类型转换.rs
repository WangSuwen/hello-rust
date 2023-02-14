use std::sync::Arc;

// 类型转换
fn main() {
    // 1、as 转换，大类型转小类型时注意数据范围
    /* let a = 3.1 as i8;
    let b = 'a' as i8;
    let c = 100_i8;
    println!("{}, {}", a, b); */

    /* let mut value = [1, 2];
    let p1 = value.as_mut_ptr();
    println!("p1 is {:?}", p1);
    let first_address = p1 as usize;
    println!("first_address is {}", first_address);
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    println!("p2 is {:?}", p2);
    unsafe {
        *p2 += 1;
    }
    println!("value[1] is {}", value[1]) */

    // 2、 TryInto 转换
    /* let a: u8 = 10;
    let b: u16 = 1000;
    // let b_: u8 = b.try_into().unwrap();
    // if a < b_ {
    //     println!("10 < 1000");
    // }
    let b_: u16 = match a.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    println!("b_ is {:?}", b_); */

    /* trait Trait {}
    fn foo<X: Trait>(t: X) {}
    impl<'a> Trait for &'a i32 {}
    let t: &mut i32 = &mut 0;
    foo(t); */

    /* #[derive(Clone)]
    struct Container<T>(Arc<T>);

    fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
        let foo_cloned = foo.clone();
        let bar_cloned = bar.clone();
    } */


    #![allow(unused)]
    fn foo() -> i32 {
        0
    }

    let pointer = foo as *const ();
    let function = unsafe { 
        // 将裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer) 
    };
    assert_eq!(function(), 0);


}