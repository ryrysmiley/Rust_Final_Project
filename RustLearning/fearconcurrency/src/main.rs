//fearless concurrency
fn main() {
    //concurrent programming is where different parts of a program execute independently
    //use threads to run code simultaneously
    //race conditions occur when threads access data at the same time
    //deadlocks are where two threads are waiting for eachother preventing both threads from continuing
    
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    //message passing through channels
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //shared state concurrency
    //use mutexes to allow only one thread to access data at a time
    //use multiple ownership with Rc<T> and RefCell<T> to allow multiple owners of mutable data
    //use multiple ownership with Rc<T> to allow multiple owners of immutable data
    //use multiple ownership with RefCell<T> to allow multiple owners of mutable data
}
