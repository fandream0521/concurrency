use anyhow::Result;
fn main() -> Result<()> {
    let join = std::thread::spawn(|| {
        println!("Hello from the spawned thread!");
    });
    join.join().unwrap();
    Ok(())
}
