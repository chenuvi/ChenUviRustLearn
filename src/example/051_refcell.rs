use std::cell::RefCell;
use std::rc::Rc;
struct Database {
    max_connections: u32,
}

// Rc 智能指针
#[allow(dead_code)]
struct AuthService {
    db: Rc<RefCell<Database>>,
}
#[allow(dead_code)]
struct ContentService {
    db: Rc<RefCell<Database>>,
}

#[allow(dead_code, unused_variables)]
pub fn run() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
    db.borrow_mut().max_connections = 200;
}
