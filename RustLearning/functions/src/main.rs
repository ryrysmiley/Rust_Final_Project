fn main() {
    println!("Hello, world!");
}

fn binary_search(list: &[i32], item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = list[mid];
        if guess == item {
            return Some(mid);
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1; //this is a statement but without a semicolon it as an expression
// }
