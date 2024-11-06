use anyhow::Result;
use concurrency::{multiply, Matrix};

fn main() -> Result<()> {
    let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
    println!("{}", a);
    let b = Matrix::new(&[10, 11, 20, 21, 30, 31], 3, 2);
    println!("{}", b);

    let result = multiply(&a, &b)?;
    println!("{}", result);
    Ok(())
}
