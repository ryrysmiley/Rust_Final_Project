/*collections*/
use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    // let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    /*print contents of vector*/
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    /*strings*/
    let mut s = String::new();
    s.push_str("Hello");
    println!("{}", s);

    /*Hash Maps*/
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}: {}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /*adding only when a key isnt present use entry*/
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}
