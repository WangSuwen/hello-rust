在 Rust 中，`&str`、`*str`、`str` 三者都与字符串相关，但含义和使用方式不同，它们的区别如下：

* 1、&str

`&str` 表示对一个字符串的引用。它是一个指向存储在堆上的 UTF-8 字符串的指针，通常是由 String 类型的实例通过引用计数（Rc）共享而来。`&str` 是 Rust 中最常见的字符串类型，它是 Rust 中的一等公民，支持各种字符串操作。

例如，我们可以定义一个函数，接受一个 `&str` 参数，并将其打印出来：

```rust
    fn print_string(s: &str) {
        println!("{}", s);
    }
```
* 2、 *str

`*str` 表示对一个字符串的裸指针。裸指针是不安全的，因为它们不受 Rust 的借用检查和所有权规则的约束。`*str` 类型通常只在与 C 代码交互或进行 `FFI（Foreign Function Interface）`时使用，因为 C 字符串通常是以 null 结尾的字符数组，而 Rust 字符串通常是以长度为前缀的字节数组。

例如，我们可以将一个 `*const u8` 类型的指针转换为 `*const char` 类型的指针，从而得到一个 `*str` 指针：

```rust
    let s = "hello".as_ptr();
    let s = s as *const u8;
    let s = s as *const i8;
    let s = s as *const char;
    let s = unsafe { &*s };
    println!("{}", s); // "hello"
```
* 3、 str

`str` 是 Rust 中的一个类型，表示字符串字面量。它通常出现在函数签名中，表示返回一个字符串字面量。由于字符串字面量存储在程序的只读内存区域中，所以 Rust 编译器可以在程序的只读数据段中直接存储和访问它们，而不必在堆上分配内存。因此，使用 `str` 类型可以提高程序的性能。

例如，我们可以定义一个函数，返回一个字符串字面量：

```rust
fn get_string() -> &'static str {
    "hello"
}
```

需要注意的是，由于 `str` 类型存储在只读内存区域中，因此不能修改它们的值。如果需要修改字符串，应该使用可变的 String 类型或可变的 `&mut str` 引用。