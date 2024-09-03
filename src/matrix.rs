use anyhow::anyhow;
use anyhow::Result;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul};

// [[1, 2], [1, 2], [1, 2]] => [1, 2, 1, 2, 1, 2]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

/// 为什么这里使用 引用，因为只是做乘法，读数据就可以了
/// 为什么还要实现 copy 因为在做乘法时，可以直接拿来用，不用再解引用
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if a.col != b.row {
        return Err(anyhow!("matrix multiply error: a.col != b.row"));
    }

    // let mut data = Vec::with_capacity(a.row * b.col);
    // 这里不能直接 使用 vec![0; a.row * b.col] 进行初始化，data的类型是 Vec<i32>，而不是 Vec<T>
    // let mut data = vec![0; a.row * b.col];
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);

        let c = multiply(&a, &b)?;

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

        let c = multiply(&a, &b)?;

        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![7, 10, 15, 22]);

        assert_eq!(format!("{:?}", c), "Matrix(row=2, col=2, {7 10, 15 22})");

        Ok(())
    }
}
