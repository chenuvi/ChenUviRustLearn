use std::sync::Mutex;

#[derive(Debug)]
struct Database {
    connections: Vec<u32>,
}

impl Database {
    fn new() -> Self {
        Self {
            connections: { vec![] },
        }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

#[allow(dead_code)]
pub fn run() {
    let db = Mutex::new(Database::new());
    {
        let mut db_lock = db.lock().unwrap();
        db_lock.connect(1);
    }
}
