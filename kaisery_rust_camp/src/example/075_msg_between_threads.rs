#[allow(unused_imports)]
use std::{sync::mpsc, thread};

#[allow(dead_code)]
pub fn run() {
    let (tx, rx) = mpsc::channel();

    let sentences = [
        "21".to_owned(),
        "31".to_owned(),
        "41".to_owned(),
        "51".to_owned(),
        "61".to_owned(),
    ];

    for s in sentences {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reversed = s.chars().rev().collect::<String>();
            tx_clone.send(reversed).unwrap();
        });
    }
    let _ = rx.recv();
}
