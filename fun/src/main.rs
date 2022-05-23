fn main() {
    another_function(5, 'h');
    expression_fn();

    let five = five();
    println!("The value of five is: {}", five);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {}{}", x, unit_label)
}

// fn statement_fn() {
//     let x = (let y = 6);
// }

fn expression_fn() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of expression_fn y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
