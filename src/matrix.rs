use crate::{dot_product, Vector};
use anyhow::anyhow;
use anyhow::Result;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul};
use std::sync::mpsc;
use std::thread;

const NUM_THREADS: usize = 4;

// [[1, 2], [1, 2], [1, 2]] => [1, 2, 1, 2, 1, 2]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

/// 发送给子线程进行点积运算的 消息
pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}
/// 子线程计算完后，将值进行返回的消息
pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

///
pub struct Msg<T> {
    input: MsgInput<T>,

    // sender to send the result back
    // 这是子线程将结果发回来的 channel sender，主线程接收这个结果
    // 计算出结果后，可以通过这个 sender 发回来给我
    sender: oneshot::Sender<MsgOutput<T>>,
}

/// 为什么这里使用 引用，因为只是做乘法，读数据就可以了
/// 为什么还要实现 copy 因为在做乘法时，可以直接拿来用，不用再解引用
/// `T` cannot be sent between threads safely
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Send + 'static, // 为能在 线程之间传送
{
    if a.col != b.row {
        return Err(anyhow!("matrix multiply error: a.col != b.row"));
    }

    // let mut data = Vec::with_capacity(a.row * b.col);
    // 这里不能直接 使用 vec![0; a.row * b.col] 进行初始化，data的类型是 Vec<i32>，而不是 Vec<T>
    // let mut data = vec![0; a.row * b.col];
    // let mut data = vec![T::default(); a.row * b.col];
    // for i in 0..a.row {
    //     for j in 0..b.col {
    //         for k in 0..a.col {
    //             data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
    //         }
    //     }
    // }

    // 先创建 4 个 thread ，接收消息并进行点积运算
    // 每个 thread 都需要一个 channel，建立主线程 和 子线程的通信渠道
    let senders = (0..NUM_THREADS)
        // _ignore 代表是第几个线程，不重要
        .map(|_ignore| {
            // sender 由主线程使用，发送消息
            // receiver 在子线程中，接收消息，并进行点积运算
            let (sender, receiver) = mpsc::channel::<Msg<T>>();

            thread::spawn(move || {
                // receiver 收到 message，对 message 的 input 进行点积运算
                for msg in receiver {
                    let value = dot_product(msg.input.row, msg.input.col)?;

                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }

                Ok::<_, anyhow::Error>(())
            });

            sender
        })
        .collect::<Vec<_>>();

    // generate 4 threads which receive msg and do dot product
    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.row {
        for j in 0..b.col {
            // 先拿出 row 和 col 然后进行 点积运算
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();

            let col = Vector::new(col_data);

            let idx = i * b.col + j;
            // 用行和列生成消息
            let input = MsgInput::new(idx, row, col);

            // 创建了一个 oneshot::Sender（一次性的），每个线程计算完后，可以返回一个 结果 ，通过 receiver.recv 结果
            let (sender, receiver) = oneshot::channel();
            let msg = Msg::new(input, sender);

            // 发送的动作非常快，不用管他执行完
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }

            // 由 receiver 去收集结果
            receivers.push(receiver);
        }
    }

    // map reduce: reduce phase
    for receiver in receivers {
        let output = receiver.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> Matrix<T> {
    // 任何数据结构，只要能够 convert 成 Vec，就可以传入
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T: Display> Display for Matrix<T> {
    // display as 2x3 as [1 2 3, 4 5 6], 3x2 as [1 2, 3 4, 5 6]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T: Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Matrix<T>;
    // type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    #[test]
    fn test_matrix_new() {
        let arr = [1, 2, 3, 4, 5, 6];
        let matrix = Matrix::new(&arr, 2, 3);
    }

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);

        // let c = multiply(&a, &b)?;
        let c = a * b;

        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);

        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {22 28, 49 64})");

        Ok(())
    }

    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);

        // let c = multiply(&a, &b)?;

        let c = a * b;

        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![7, 10, 15, 22]);

        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {7 10, 15 22})");

        Ok(())
    }

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b);
        assert!(c.is_err())
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);

        let _c = a * b;
    }
}
