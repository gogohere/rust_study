// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }


// 代码中的条件 必须是 bool 值。如果条件不是 bool 值，我们将得到一个错误。
// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }


// 不像 Ruby 或 JavaScript 这样的语言，Rust 并不会尝试自动地将非布尔值转换为布尔值
// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }


// 可以将 else if 表达式与 if 和 else 组合来实现多重条件。
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }


// 因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {}", number);
// }


// 整个 if 表达式的值取决于哪个代码块被执行。这意味着 if 的每个分支的可能的返回值都必须是相同类型
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {}", number);
// }


// loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。
// fn main() {
//     loop {
//         println!("again!");
//     }
// }


// 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
// 你可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，
// 使这些关键字应用于已标记的循环而不是最内层的循环
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {}", count);
// }


// 如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }


// 当条件为真就执行，否则退出循环。
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {}", element);
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}