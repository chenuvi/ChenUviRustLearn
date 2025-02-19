use std::rc::Rc;
struct Database {}

// Rc 智能指针
#[allow(dead_code)]
struct AuthService {
    db: Rc<Database>,
}
#[allow(dead_code)]
struct ContentService {
    db: Rc<Database>,
}

#[allow(dead_code, unused_variables)]
pub fn run() {
    let db = Rc::new(Database {});
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
}
