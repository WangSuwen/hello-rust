// 类型

fn main () {
    /* let a: i32 = 10;
    let b: u16 = 100;
  
    if a < b { // 或报错，并提示 将 u16 转换成 i32 类型
      println!("Ten is less than one hundred.");
    } */
    /* let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97

    println!("{},{},{}",a,b,c) */

    // 内存地址转换成指针
    /* #![allow(unused)]
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4，即：指针向后移一位
    let p2 = second_address as *mut i32; // 将移动后的新地址转成指针
    unsafe {
        println!("p1: {:?}, first_address: {}, second_address: {},  p2: {:?}, 指针p2指向的值: {:?}", p1, first_address, second_address, p2, *p2);
    }
    unsafe {
        *p2 += 1; // 将指针 p2指向的值加一
    }
    assert_eq!(values[1], 3); */

    let b: u16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(result) => result,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    println!("b_ is: {:?}", b_);


}