// fn main() {
//     let user1 = User{
//         active: true,
//         username: String::from("seancraven314"),
//         eamil: String::from("someone@example.com"),
//         sign_in_count: 1,
//  // Note: the entire struct must be mutable for you to change a field.
//     };
//     let user2 = User{
//         active: user1.active
//         username: user1.username
//         email: String::from("Somerandom@domain.com"),
//         sign_in_count: user1.sign_in_count
//     };

//     let user3 = User{
//         email: String::from("an_even_more_random@domain.com"),
//         ..user1
//     };
// }

// fn make_user(email: String, username:string)-> User{
//     return User{
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// struct User{
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Using fn: The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "Using method: The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
