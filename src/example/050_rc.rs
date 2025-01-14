use std::rc::Rc;
struct Database {}

// Rc 智能指针
struct AuthService {
    db: Rc<Database>,
}

struct ContentService {
    db: Rc<Database>,
}

pub fn run() {
    let db = Rc::new(Database {});
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
}
