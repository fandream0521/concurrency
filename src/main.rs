use std::thread;

fn main() {
    let join = thread::spawn(|| {
        println!("Hello from the spawned thread!");
    });
    join.join().unwrap();
}
