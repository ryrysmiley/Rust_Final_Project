// errors
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //recoverable errors with are like a file that cannot be found
    //unrecoverable errors are like array index out of bounds
    //most programming languages have exceptions
    //Rust does not have exceptions instead it has Result for recoverable errors and panic for unrecoverable errors
    
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    /*let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");*/

    //important whether to panic or whether to result
    //if you have a situation where you can recover from the error then you should return a result
    //if you have a situation where you cannot recover from the error then you should panic
}
