use std::{sync::mpsc, thread};

fn main(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let rec = rx.recv().unwrap();

    println!("{rec}");
}
