use std::{sync::mpsc, thread, time::Duration};

enum Message {
    NumberFloat(f64),
    Text(String),
}

#[allow(dead_code)]
pub fn create_channel() {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![1.1, 1.2, 1.3, 1.4, 1.5];
        for val in vals {
            tx.send(Message::NumberFloat(val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2.1 more"),
            String::from("2.2 messages"),
            String::from("2.3 Just"),
            String::from("2.4 for"),
            String::from("2.5 you"),
        ];
        for val in vals {
            tx2.send(Message::Text(val)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        handle_message(received);
    }
}

fn handle_message(msg: Message) {
    match msg {
        Message::NumberFloat(num) => println!("Number: {}", num),
        Message::Text(text) => println!("Text: {}", text),
    }
}
