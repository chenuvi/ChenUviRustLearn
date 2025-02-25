#[allow(dead_code)]
pub fn run() {
    let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    // let total: i32 = v1_iter.sum();
    // println!("total: {total}");

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
