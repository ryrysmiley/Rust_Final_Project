/*Iterators and closures*/
//will discuss how Rust is similar to other languages considered to be functional
fn main() {
    /*
    Closures, a function-like construct you can store in a variable
    Iterators, a way of processing a series of elements
    */
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //iters are lazy meaning they have no effect until they're consumed by a method that turns them into something else
    let list = vec![1, 2, 3];
    let mut v1_iter = list.iter();
    // for i in iter {
    //     println!("{}", i);
    // }
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    //print sum
    let total: i32 = list.iter().sum();
    println!("{}", total);
}
