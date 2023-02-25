// An enum is a data type where an object must be one of the kinds
// defined
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
// Enum can do something similar to this!
// Here we have typed enum with move having its own fields.
// An enum can only be one of these at once
// Enum is strictly OR struct can be and or or.
enum Action {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// The option enum is good example in that it indicates that something
// can either be a type or none.

enum Coin {
    Pound,
    TwoPound,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let absent_number: Option<i32> = None;

    let some_num: Option<i32> = Some(5);

    let one_and_some_num = add_option(some_num);

    let one_and_none = add_option(absent_number);

    println!("Experiments with match");
    println!("Adding Option {:#?}", one_and_some_num);
    println!("Adding Option with none {:#?}", one_and_none);

    // If you try add this to a i32 you get an error. As the add op
    // is not overloaded for option, this is nice because you can't
    // pass none when you aren't expecting it.

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.168.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// This allows function to take either type of the ip
fn route(ip_kind: IpAddrKind) {}

fn value_in_pennies(coin: Coin) -> u8 {
    // Match ensures that each field of the enum has a behaviour.
    match coin {
        Coin::Pound => {
            println!("One Whole Pound");
            100
        }
        Coin::TwoPound => 200,
    }
}

fn add_option(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
