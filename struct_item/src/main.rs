struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;

// fn main() {
//     let user = User {
//         active: true,
//         sign_in_count: 1,
//         username: String::from("your@rr.com"),
//         email: String::from("who you are")
//     }
// }

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("emil is: {}", user2.email)

    let subject = AlwaysEqual;
}

// fn build_user(email: String, username: String) -> User {
//     // User {
//     //     email: email,
//     //     username: username,
//     //     active: true,
//     //     sign_in_count: 1,
//     // }

//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }