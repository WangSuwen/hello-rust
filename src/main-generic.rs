// 泛型
fn main () {
    
    // #[derive(Debug)]
    // TODO: 注意：泛型在使用之前需要先声明，  largest 后面的 <T> 就是声明
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    

    // 2、方法中使用泛型
    // TODO: 在方法中使用泛型时，需要先定义泛型，注意：   impl<T> 这才是定义泛型， 而 Point<T> 整体是表示一个Struct；
    /* struct Point<T> {
        x: T,
        y: T
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let point = Point{ x: 12, y: 13 };
    println!("point`s x is {}", point.x()) */

    // 3、TODO: const 泛型
    /*  
        TODO: 之前的泛型 都是为 类型声明的泛型（比如：方法入参的类型、结构体内属性的 类型等）；
        而 const 泛型，是用来为值声明泛型的， 比如一个数组 let a = [i32, 10] 其中的 10就是值，表示这个数组的长度是10.
        下例中的定义了一个 可以接收 任意类型，任意长度的 数组的函数, 其中 const N: usize 就是一个 【const泛型】
        注意：如果入参用的是引用，则可不必如此定义
    */
    fn display_arr<T: std::fmt::Debug, const N: usize> (arr: [T; N]) {
        println!("arr is {:?}", arr);
    }
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 4] = [1, 2, 3, 4];
    display_arr(arr1);
    display_arr(arr2);

}