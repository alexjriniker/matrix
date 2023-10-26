use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub n_rows: usize,
    pub n_cols: usize,
}

#[allow(dead_code)]
impl Matrix {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        Matrix::from_iter(n_rows, n_cols, (0..).map(|_| 0.0))
    }

    pub fn from_iter(n_rows: usize, n_cols: usize, data: impl IntoIterator<Item = f64>) -> Self {
        let data: Vec<f64> = data.into_iter().take(n_cols * n_rows).collect();

        assert_eq!(
            data.len(),
            n_rows * n_cols,
            "Matrix data size does not match n_rows * n_cols"
        );

        Matrix {
            data,
            n_cols,
            n_rows,
        }
    }

    pub fn same_size_as(&self, mat: &Matrix) -> bool {
        self.n_rows == mat.n_rows && self.n_cols == mat.n_cols
    }

    pub fn is_square(&self) -> bool {
        self.n_rows == self.n_cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&f64> {
        if row < self.n_rows && col < self.n_cols {
            Some(&self.data[col + row * self.n_cols])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut f64> {
        if row < self.n_rows && col < self.n_cols {
            Some(&mut self.data[col + row * self.n_cols])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Option<()> {
        match self.get_mut(row, col) {
            Some(v) => {
                *v = value;

                Some(())
            }
            None => None,
        }
    }

    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &f64>> {
        if row < self.n_rows {
            Some((0..self.n_cols).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &f64>> {
        if col < self.n_cols {
            Some((0..self.n_rows).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn apply<F: FnMut(&f64)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    pub fn apply_mut<F: FnMut(&mut f64)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }

    pub fn dot(&self, rhs: &Matrix) -> Option<Matrix> {
        if self.n_cols != rhs.n_rows {
            None
        } else {
            let mut result = Matrix::new(self.n_rows, rhs.n_cols);

            for i in 0..self.n_rows {
                for j in 0..rhs.n_cols {
                    for k in 0..self.n_cols {
                        let cell = result.get_mut(i, j).unwrap();
                        let value = self.get(i, k).unwrap() * rhs.get(k, j).unwrap();

                        *cell += value
                    }
                }
            }

            Some(result)
        }
    }

    pub fn transpose(&self) -> Matrix {
        let data = (0..self.n_rows)
            .map(|row| self.get_row(row).unwrap().map(|v| *v))
            .flatten();
        Matrix::from_iter(self.n_cols, self.n_rows, data)
    }

    // fn submatrix(data: &Vec<Vec<f64>>, row: usize, col: usize) -> Vec<Vec<f64>> {
    //     data.iter()
    //         .enumerate()
    //         .filter(|&(i, _)| i != row)
    //         .map(|(_, row)| {
    //             row.iter()
    //                 .enumerate()
    //                 .filter(|&(j, _)| j != col)
    //                 .map(|(_, &element)| element)
    //                 .collect()
    //         })
    //         .collect()
    // }

    // fn calculate_determinate(data: &Vec<Vec<f64>>, size: usize) -> f64 {
    //     if size == 1 {
    //         data[0][0]
    //     } else if size == 2 {
    //         data[0][0] * data[1][1] + data[0][1] * data[1][0]
    //     } else {
    //         let mut result = 0.0;

    //         for i in 0..size {
    //             let mat = Matrix::submatrix(data, 0, i);
    //             let det = Matrix::calculate_determinate(&mat, size - 1);

    //             result += data[0][i] * det * if i % 2 == 0 { 1.0 } else { -1.0 };
    //         }

    //         result
    //     }
    // }

    // TODO: implement in less junky way
    pub fn invert(&self) -> Option<Matrix> {
        let matrix = (0..self.n_rows)
            .map(|row| self.get_row(row).unwrap().map(|v| *v).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = matrix.len();

        // Check if the matrix is square
        if size != matrix.iter().filter(|row| row.len() == size).count() {
            println!("NOT SQUIRE");
            return None; // Matrix is not square
        }

        let mut augmented_matrix = vec![vec![0.0; size * 2]; size];

        for i in 0..size {
            for j in 0..size {
                augmented_matrix[i][j] = matrix[i][j];
                augmented_matrix[i][j + size] = if i == j { 1.0 } else { 0.0 };
            }
        }

        // Perform Gaussian elimination to transform the left half into the identity matrix
        for col in 0..size {
            let mut pivot_row = col;
            for row in (col + 1)..size {
                if augmented_matrix[row][col].abs() > augmented_matrix[pivot_row][col].abs() {
                    pivot_row = row;
                }
            }

            augmented_matrix.swap(col, pivot_row);
            let pivot = augmented_matrix[col][col];

            if pivot == 0.0 {
                println!("BAD PIVOT");
                return None; // Matrix is not invertible
            }

            for j in 0..(size * 2) {
                augmented_matrix[col][j] /= pivot;
            }

            for i in 0..size {
                if i != col {
                    let factor = augmented_matrix[i][col];
                    for j in 0..(size * 2) {
                        augmented_matrix[i][j] -= factor * augmented_matrix[col][j];
                    }
                }
            }
        }

        let mut inverse = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                inverse[i][j] = augmented_matrix[i][j + size];
            }
        }

        Some(Matrix::from_iter(
            self.n_rows,
            self.n_cols,
            inverse.iter().map(|v| v.iter().map(|v| *v)).flatten(),
        ))
        // let data = (0..self.n_rows)
        //     .map(|row| self.get_row(row).unwrap().map(|v| *v).collect::<Vec<_>>())
        //     .collect::<Vec<_>>();
        // let det = Matrix::calculate_determinate(&data, self.n_rows);

        // if det == 0.0 {
        //     None
        // } else {
        //     let mut result = Matrix::new(self.n_rows, self.n_cols);

        //     for i in 0..self.n_rows {
        //         for j in 0..self.n_rows {
        //             let mat = Matrix::submatrix(&data, i, j);

        //             result.set(j, i, Matrix::calculate_determinate(&mat, mat.len()) / det);
        //         }
        //     }

        //     Some(result)
        // }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}x{})", self.n_rows, self.n_cols)
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Option<Matrix>;

    fn add(self, rhs: &Matrix) -> Self::Output {
        if self.same_size_as(rhs) {
            Some(Matrix::from_iter(
                self.n_rows,
                self.n_cols,
                self.data.iter().enumerate().map(|(i, v)| v + rhs.data[i]),
            ))
        } else {
            None
        }
    }
}

impl Add<f64> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: f64) -> Self::Output {
        Matrix::from_iter(self.n_rows, self.n_cols, self.data.iter().map(|v| v + rhs))
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Option<Matrix>;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        if self.same_size_as(rhs) {
            Some(Matrix::from_iter(
                self.n_rows,
                self.n_cols,
                self.data.iter().enumerate().map(|(i, v)| v - rhs.data[i]),
            ))
        } else {
            None
        }
    }
}

impl Sub<f64> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: f64) -> Self::Output {
        Matrix::from_iter(self.n_rows, self.n_cols, self.data.iter().map(|v| v - rhs))
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Option<Matrix>;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        if self.same_size_as(rhs) {
            Some(Matrix::from_iter(
                self.n_rows,
                self.n_cols,
                self.data.iter().enumerate().map(|(i, v)| v * rhs.data[i]),
            ))
        } else {
            None
        }
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        Matrix::from_iter(self.n_rows, self.n_cols, self.data.iter().map(|v| v * rhs))
    }
}

impl Div<&Matrix> for &Matrix {
    type Output = Option<Matrix>;

    fn div(self, rhs: &Matrix) -> Self::Output {
        if self.same_size_as(rhs) {
            Some(Matrix::from_iter(
                self.n_rows,
                self.n_cols,
                self.data.iter().enumerate().map(|(i, v)| v / rhs.data[i]),
            ))
        } else {
            None
        }
    }
}

impl Div<f64> for &Matrix {
    type Output = Matrix;

    fn div(self, rhs: f64) -> Self::Output {
        Matrix::from_iter(self.n_rows, self.n_cols, self.data.iter().map(|v| v / rhs))
    }
}
