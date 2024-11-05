use std::{sync::mpsc::Sender, thread, time::Duration};

use anyhow::Result;
use rand::Rng;

fn main() -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    for idx in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || producer(idx, tx));
    }

    drop(tx);

    for msg in rx {
        println!("Received: {:?}", msg);
    }
    Ok(())
}

fn producer(idx: i32, tx: Sender<Msg>) -> Result<()> {
    loop {
        let random_num = rand::random::<usize>();
        if random_num % 10 == 0 {
            println!("{}: Exiting", idx);
            break;
        }
        tx.send(Msg::new(idx, random_num))?;
        let sleep_time = rand::thread_rng().gen_range(100..3000);
        thread::sleep(Duration::from_millis(sleep_time));
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: i32,
    random_num: usize,
}

impl Msg {
    fn new(idx: i32, random_num: usize) -> Self {
        Self { idx, random_num }
    }
}
