use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;

fn main() {
    let l = Arc::new(RwLock::new(String::new()));

    let l1 = l.clone();
    let l2 = l;

    println!("start");
    let t1 = thread::spawn(move || {
        println!("t1 trying first read");
        let r1 = l1.read();
        println!("t1 trying second read");
        let r2 = l1.read_recursive();
        println!("t1 obtained two read locks");

        drop(r2);
        drop(r1);
    });
    let t2 = thread::spawn(move || {
        println!("t2 trying write");
        let w = l2.write();
        println!("t2 obtained write lock");
        drop(w);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
