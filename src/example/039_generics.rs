#[allow(dead_code)]
struct BrowserCommand<T> {
    name: String,
    payload: T,
}
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        return BrowserCommand { name, payload };
    }
    fn get_payload(&self) -> &T {
        &self.payload
    }
}
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload)
    }
}
#[allow(unused_variables)]
fn serialize_payload<T>(payload: T) -> String {
    "tst".to_owned()
}
#[allow(dead_code)]
pub fn run() {
    let cmd1 = BrowserCommand::new("refresh".to_owned(), "www.abc.com".to_owned());
    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);

    cmd1.print_payload();
    // cmd2.print_payload();
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();
    serialize_payload(p1);
    serialize_payload(p2);
}
