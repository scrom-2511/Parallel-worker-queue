use parallel_worker_queue::WorkProcessor;
use std::time::Instant;

#[derive(Clone)]
struct MyData {
    some_num: i32,
}

fn some_heavy_fn(data: MyData) {
    let mut x = data.some_num;
    for _ in 0..50000 {
        x = x.wrapping_mul(33).wrapping_add(7);
    }
}

fn main() {
    let start = Instant::now();

    let processor = WorkProcessor::new(some_heavy_fn);

    for i in 0..200000 {
        processor.push(MyData { some_num: i });
    }

    processor.stop();

    let elapsed = start.elapsed();
    println!("Total time: {:?}", elapsed);
}
