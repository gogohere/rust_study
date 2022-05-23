// fn main() {
//     {                    // s 在这里无效, 它尚未声明
//         let s = "hello"; // 从此处起，s 是有效的

//         // 使用 s
//     }
// }


// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() 在字符串后追加字面值

//     println!("{}", s); // 将打印 `hello, world!`
// }


// 为了确保内存安全，在 let s2 = s1 之后，Rust 认为 s1 不再有效
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }


// 如果我们 确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }


// 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。
// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {}, y = {}", x, y);
// }


// fn main() {
//     let s = String::from("hello");  // s 进入作用域

//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效

//     let x = 5;                      // x 进入作用域

//     makes_copy(x);                  // x 应该移动函数里，
//                                     // 但 i32 是 Copy 的，
//                                     // 所以在后面可继续使用 x

// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 没有特殊之处

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。
//   // 占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。没有特殊之处


// 我们可以使用元组来返回多个值
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}