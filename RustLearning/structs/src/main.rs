struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
/*tuples are similar to structs but structs will have names for each piece of data */

fn main() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    /*print user info*/
    println!("user1 email: {}", user1.email);
    println!("user1 username: {}", user1.username);
    println!("user1 active: {}", user1.active);
    println!("user1 sign_in_count: {}", user1.sign_in_count);

    /*changing values does not work it is not mut*/
    // user1.email = String::from("usernew1@example.com");
    // println!("user1 email: {}", user1.email);

    /*create a new user with the same values as user1*/
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    /*print user2 info*/
    println!("user2 email: {}", user2.email);
    println!("user2 username: {}", user2.username);
    println!("user2 active: {}", user2.active);
    println!("user2 sign_in_count: {}", user2.sign_in_count);

    /*
    tuple structs without named fields
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    /*unit like structs without any fields*/
    struct AlwaysEqual;
    let a = AlwaysEqual;
    //more on structs later

    /*
    structs and ownership
    In our user definition we used String instead of &str string slice type
    because we want instances of this struct to own all of its data and for 
    that data to be valid for as long as the entire struct is valid.
    Structs can store references but it requires the use of lifetimes.
    Lifetimes ensure that the data referenced by a struct is valid for as long
    as the struct is.
    */
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// implementing functions on structs
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// associated functions
// impl Rectangle {
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }