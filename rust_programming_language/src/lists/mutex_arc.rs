use std::{
    sync::{Arc, Mutex},
    thread,
};

#[allow(dead_code)]
pub fn run() {
    one_thread();
    run_arc();
}

#[allow(dead_code)]
fn run_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

#[allow(dead_code)]
fn one_thread() {
    let m = Mutex::new(0);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
