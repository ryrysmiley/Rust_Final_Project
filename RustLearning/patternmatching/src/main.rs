// patterns and matching
fn main() {
    /*
    pattern consists of some combination of the following
    - literals
    - destructed arrays, enums, structs, or tuples
    - variables
    - wildcards
    - placeholders

    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
    */

    //refutable irrefutable
    //refutable patterns are patterns that have the possibility of failing to match
    //irrefutable patterns are patterns that are guaranteed to match

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let s = 5;

    match s {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
