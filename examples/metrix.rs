use std::thread;

use anyhow::Result;
use concurrency::AtomicMetrix;
use rand::Rng;
const NUM_THREADS: usize = 5;
const METRIX_TYPES: [&str; 5] = [
    "call.thread.work.0",
    "call.thread.work.1",
    "call.thread.work.2",
    "call.thread.work.3",
    "call.thread.work.4",
];
fn main() -> Result<()> {
    let metrix = AtomicMetrix::new(&METRIX_TYPES);
    for _ in 0..NUM_THREADS {
        work(metrix.clone());
    }
    let read = thread::spawn(move || {
        loop {
            thread::sleep(std::time::Duration::from_secs(2));
            println!("{}", metrix);
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    let _ = read.join().unwrap();
    Ok(())
}

fn work(metrix: AtomicMetrix) {
    thread::spawn(move || {
        loop {
            // sleep for a random amount of time
            thread::sleep(std::time::Duration::from_secs(
                rand::thread_rng().gen_range(0..10),
            ));
            let random_num = rand::thread_rng().gen_range(0..5);
            metrix.inc(METRIX_TYPES[random_num])?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
