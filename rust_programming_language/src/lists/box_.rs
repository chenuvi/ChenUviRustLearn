#[allow(unused_imports)]
use std::ops::Deref;

#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

// struct MyBox<T>(T);

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

#[allow(dead_code)]
pub fn run() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, *y);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created")

    // CustomSmartPointers created.
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!
}
