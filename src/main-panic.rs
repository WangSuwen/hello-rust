use std::{fs::File, io::{ErrorKind, Read, self}};

// 异常处理
fn main() {
    /* // panic!("here is an Error!!");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Creating file error: {:?}", e)
            },
            other_error => panic!("opening file error: {:?}", other_error)
        }
    };
    println!("{:?}", f); */

    /* fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(error) => return Err(error)
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e)
        }
    }
    let name = read_username_from_file().unwrap(); */
    // 精简写法 :
    /* fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    println!("name is {}", read_username_from_file().unwrap());

    fn first(arr: &[i32]) -> Option<&i32> {
        arr.get(0)
    } */
}