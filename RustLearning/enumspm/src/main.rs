// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddr::V6(String::from("::1"));

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
// enums give you a way of saying a value is one of a possible set of values
fn main() {
    //concise flow control with if let
    let coin = Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    //or this syntax
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
// enum Option<T> {
//     None,
//     Some(T),
// }
// let some_number = Some(5);
// let some_char = Some('e');

// let absent_number: Option<i32> = None;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//matches are exaustive and the arms' patterns must cover all possible values
//use catch all patterns _

//code will not build it is a lot of snippets