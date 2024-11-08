use std::thread;

use anyhow::Result;
use concurrency::DashMapMetrix;
use rand::Rng;
const NUM_THREADS: usize = 5;
fn main() -> Result<()> {
    let metrix = DashMapMetrix::new();
    for _ in 0..NUM_THREADS {
        let random_num = rand::thread_rng().gen_range(0..10);
        work(random_num, metrix.clone());
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

fn work(idx: usize, metrix: DashMapMetrix) {
    thread::spawn(move || {
        loop {
            // sleep for a random amount of time
            thread::sleep(std::time::Duration::from_secs(
                rand::thread_rng().gen_range(0..10),
            ));
            metrix.inc(format!("call.thread.work.{}", idx).as_str())?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
