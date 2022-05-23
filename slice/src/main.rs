// fn main() {
//     println!("Hello, world!");
// }


// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }


// 字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：
// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
// }

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}