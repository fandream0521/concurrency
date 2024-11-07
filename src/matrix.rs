use anyhow::Result;
use oneshot::Sender;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Deref, Index, IndexMut, Mul};
use std::sync::mpsc;
use std::thread;

const NUM_THREADS: usize = 4;
#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vec<T>,
    col: Vec<T>,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: Sender<MsgOutput<T>>,
}
pub struct MsgOutput<T> {
    idx: usize,
    result: T,
}
impl<T: Clone + Default + Copy> Matrix<T> {
    pub fn new(data: &[T], rows: usize, cols: usize) -> Self {
        Self {
            data: data.to_vec(),
            rows,
            cols,
        }
    }

    pub fn init(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    pub fn rows(&self, rows: usize) -> Vec<T> {
        self[rows].to_vec()
    }

    pub fn cols(&self, cols: usize) -> Vec<T> {
        self.data[cols..]
            .iter()
            .step_by(self.cols)
            .copied()
            .collect()
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + AddAssign + Default + Copy + Clone + Debug,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Invalid matrix dimensions"));
    }
    let mut result = Matrix::init(a.rows, b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = T::default();
            for k in 0..a.cols {
                sum += a[i][k] * b[k][j];
            }
            result[i][j] = sum;
        }
    }

    Ok(result)
}

pub fn multiply_thread<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + AddAssign + Default + Copy + Clone + Send + 'static + Debug,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Invalid matrix dimensions"));
    }

    let msg_senders = (0..NUM_THREADS)
        .map(|_| {
            let (sender, receiver) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in receiver {
                    let row = Vector::new(&msg.input.row);
                    let col = Vector::new(&msg.input.col);
                    let result = dot_product(row, col).unwrap();
                    msg.sender
                        .send(MsgOutput::new(msg.input.idx, result))
                        .map_err(|e| anyhow::anyhow!("Error sending message: {:?}", e))?;
                }
                Ok::<_, anyhow::Error>(())
            });
            sender
        })
        .collect::<Vec<_>>();

    let mut data = Matrix::init(a.rows, b.cols);
    let mut receivers = vec![];
    for i in 0..a.rows {
        for j in 0..b.cols {
            let idx = i * b.cols + j;
            let row = a.rows(i);
            let col = b.cols(j);
            let (sender, receiver) = oneshot::channel();
            msg_senders[idx % NUM_THREADS]
                .send(Msg::new(MsgInput::new(idx, row, col), sender))
                .map_err(|e| anyhow::anyhow!("Error sending message: {:?}", e))?;
            receivers.push(receiver);
        }
    }

    for receive in receivers {
        let result = receive.recv()?;
        data.data[result.idx] = result.result;
    }

    Ok(data)
}

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T: Clone> Vector<T> {
    pub fn new(data: &[T]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Mul<Output = T> + AddAssign + Default + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Invalid vector dimensions"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T: Debug> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            write!(f, "| ")?;
            for j in 0..self.cols {
                write!(f, "{:?} ", self.data[i * self.cols + j])?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + AddAssign + Default + Copy + Clone + Debug + Send + 'static,
{
    type Output = Result<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply_thread(&self, &rhs)
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> MsgOutput<T> {
    pub fn new(idx: usize, result: T) -> Self {
        Self { idx, result }
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vec<T>, col: Vec<T>) -> Self {
        Self { idx, row, col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[10, 11, 20, 21, 30, 31], 3, 2);
        let result = multiply(&a, &b).unwrap();
        assert_eq!(
            result.data,
            vec![140, 146, 320, 335],
            "Matrix multiplication failed"
        );
    }

    #[test]
    fn test_display() {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(
            format!("{}", a),
            "| 1 2 3 |\n| 4 5 6 |\n",
            "Matrix display failed"
        );
    }

    #[test]
    fn test_vector_dot_product() {
        let a = Vector::new(&[1, 2, 3]);
        let b = Vector::new(&[4, 5, 6]);
        let result = dot_product(a, b).unwrap();
        assert_eq!(result, 32, "Vector dot product failed");
    }

    #[test]
    fn test_multiply_thread() {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[10, 11, 20, 21, 30, 31], 3, 2);
        let result = multiply_thread(&a, &b).unwrap();
        assert_eq!(
            result.data,
            vec![140, 146, 320, 335],
            "Matrix multiplication failed"
        );
    }

    #[test]
    fn test_multiply_thread_large() {
        let a = Matrix::new(&[1; 1000], 100, 10);
        let b = Matrix::new(&[1; 1000], 10, 100);
        let result = multiply_thread(&a, &b).unwrap();
        assert_eq!(result.data, vec![10; 10000], "Matrix multiplication failed");
    }

    #[test]
    fn test_multiply_thread_invalid() {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[10, 11, 20, 21, 30, 31], 2, 2);
        let result = multiply_thread(&a, &b);
        assert!(result.is_err(), "Matrix multiplication should fail");
    }

    #[test]
    fn test_multiply_thread_invalid_vector() {
        let a = Vector::new(&[1, 2, 3]);
        let b = Vector::new(&[4, 5]);
        let result = dot_product(a, b);
        assert!(result.is_err(), "Vector dot product should fail");
    }
}
