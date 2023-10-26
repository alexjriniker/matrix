use std::{
    fmt::{Display, Formatter},
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    n_rows: usize,
    n_cols: usize,
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

    pub fn set(&mut self, row: usize, col: usize, value: &mut f64) -> Option<()> {
        match self.get_mut(row, col) {
            Some(v) => {
                *v = *value;

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
