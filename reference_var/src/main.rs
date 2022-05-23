// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// 尝试修改引用值
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // 报错，默认是不允许修改引用值的
// }



// 可变引用
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("The s is {}", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// 在同一时间只能有一个对某一特定数据的可变引用
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }


// 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

//     let r2 = &mut s;
// }


// 同时使用可变与不可变引用，也是不被允许的
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     let r3 = &mut s; // 大问题

//     println!("{}, {}, and {}", r1, r2, r3);
// }


// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     println!("{} and {}", r1, r2);
//     // 此位置之后 r1 和 r2 不再使用

//     let r3 = &mut s; // 没问题
//     println!("{}", r3);
// }


fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}