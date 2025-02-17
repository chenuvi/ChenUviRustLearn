#[allow(dead_code)]
pub fn run() {
    let mut v = Vec::new();
    v.push(String::from("One"));

    // let v2 = vec![1, 2, 3, String::from("asdf")];

    // let s = &v[0]; // can panic
    let s = v.get(0);

    if let Some(e) = s {
        println!("e: {e}");
    }
    println!("d:");
}
