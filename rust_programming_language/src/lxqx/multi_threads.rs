use std::any;
use std::time::Instant;

#[allow(dead_code)]
pub fn run() {
    count_time(serialized_calculate);
}

const TOTAL: u64 = 1000000000;
#[allow(dead_code)]
fn count_time<F: FnOnce()>(f: F) {
    let name = any::type_name::<F>();
    let start = Instant::now();
    f();
    println!("[{name}] 总耗时 {} ms", start.elapsed().as_millis());
}
#[allow(dead_code)]
fn serialized_calculate() {
    let mut sum = 0;
    for i in 0..TOTAL {
        sum += i;
    }
    println!("sum: {}", sum);
}

// fn parallel_calculate() {}
