//ownership is how Rust keeps track of memory
//Each value in Rust has an owner
//There can only be one owner at a time
//When the owner goes out of scope, the value will be dropped

fn main() {
    //s is not valid here, it's not yet declared
    let s = "hello"; //s is valid from this point forward
    //do stuff with s
} //this scope is now over, and s is no longer valid

fn stringliterals() {
    let mut s = String::from("hello");
    s.push_str(", world!"); //push_str() appends a literal to a String  
    println!("{}", s); //This will print `hello, world!`    
}

//string literals are immutable but they are fast and efficient
//special function called drop() is called when a variable goes out of scope

// example of both pointing at the same memory location IN OTHER LANGUAGES
// let s1 = String::from("hello");
// let s2 = s1; //correct copy let s2 = s1.clone();

// println!("{}, world!", s1);

//s2 now owns the value and s1 is no longer valid

/*
copy trait from
All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
*/

//functions can take ownership of a variable and return ownership but it can be tedious
//we can use a tuple to return ownership and multiple values

//THIS TEDIOUS WAY CAN BE COUNTERED BY REFERENCES AND BORROWING
//Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.