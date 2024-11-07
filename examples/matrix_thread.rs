use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    // generate a matrix of 20x30
    let data = (0..20 * 30).collect::<Vec<_>>();
    let a = Matrix::new(&data, 20, 30);
    println!("{}", a);
    // generate a matrix of 30x40
    let data = (0..30 * 40).collect::<Vec<_>>();
    let b = Matrix::new(&data, 30, 40);
    println!("{}", b);

    let result = (a * b)?;
    println!("{}", result);
    Ok(())
}
