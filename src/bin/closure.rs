use std::thread;

fn main(){
    let list = vec![1,2,3];
    println!("Before defining a closure {list:?}");

    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap()
}
