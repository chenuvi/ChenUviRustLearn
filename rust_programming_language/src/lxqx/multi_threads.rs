use std::sync::{mpsc, Arc, Mutex};
use std::time::Instant;
use std::{any, thread};

#[allow(dead_code)]
pub fn run() {
    count_time(serialized_calculate);
    count_time(parallel_calculate);
    count_time(scoped_calculate);
    count_time(channel_calculate);
}

const TOTAL: u64 = 100000000;
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

fn parallel_calculate() {
    let chunk_size = TOTAL / 16;
    let sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..16 {
        let start = i * chunk_size;
        let end = if i == 15 { TOTAL } else { chunk_size * (i + 1) };

        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            for i in start..end {
                local_sum += i;
            }
            let mut sum = sum_clone.lock().unwrap();
            *sum += local_sum;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("sum: {}", sum.lock().unwrap());
}

#[allow(dead_code)]
fn scoped_calculate() {
    let chunk_size = TOTAL / 16;
    let sum = Mutex::new(0);
    thread::scope(|scope| {
        for i in 0..16 {
            let start = i * chunk_size;
            let end = if i == 15 { TOTAL } else { chunk_size * (i + 1) };

            let sum_ref = &sum;
            scope.spawn(move || {
                let mut local_sum = 0;
                for i in start..end {
                    local_sum += i;
                }

                let mut sum = sum_ref.lock().unwrap();
                *sum += local_sum;
            });
        }
    });

    println!("sum: {}", sum.lock().unwrap());
}

#[allow(dead_code)]
fn channel_calculate() {
    let chunk_size = TOTAL / 16;
    let (tx, rx) = mpsc::channel();

    for i in 0..16 {
        let start = i * chunk_size;
        let end = if i == 15 { TOTAL } else { chunk_size * (i + 1) };
        let tx = tx.clone();
        thread::spawn(move || {
            let mut local_sum = 0;
            for i in start..end {
                local_sum += i;
            }
            tx.send(local_sum).unwrap();
        });
    }
    
    // 丢弃原始发送器，这样当所有线程完成时，通道会关闭
    drop(tx);

    let mut sum = 0;
    while let Ok(local_sum) = rx.recv() {
        sum += local_sum;
    }
    println!("sum: {}", sum);
}
