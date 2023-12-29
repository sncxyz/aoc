// TODO: sort out use of Clone and references

use core::{
    mem,
    ops::{Add, Mul},
};

use num_traits::{NumAssign, Signed, Zero};

use crate::{
    matrix::{Matrix, Row},
    vector::v,
    Vec2,
};

impl<K> Matrix<K> {
    #[inline(always)]
    pub fn is_square(&self) -> bool {
        self.dim.x == self.dim.y
    }

    #[track_caller]
    pub fn augment(&mut self, other: Self) {
        if self.dim.y != other.dim.y {
            panic!("matrix heights are not the same");
        }
        self.dim.x += other.dim.x;
        for (row, mut add) in self.iter_mut().zip(other.into_iter()) {
            row.elems.append(&mut add.elems);
            row.elems.shrink_to_fit();
        }
    }

    fn remove_lhs(&mut self, len: usize) {
        for row in self.iter_mut() {
            row.elems.drain(..len);
        }
        self.dim.x -= len;
    }

    #[track_caller]
    fn assert_sq(&self) {
        if !self.is_square() {
            panic!("matrix is not square");
        }
    }
}

// TODO: avoid need for [`Clone`] and [`Zero`] here
impl<K: Clone + Zero> Matrix<K> {
    pub fn transpose(&mut self) {
        if self.is_square() {
            for r in 0..self.dim.y {
                for c in r + 1..self.dim.x {
                    let mut temp = mem::replace(&mut self[r][c], K::zero());
                    mem::swap(&mut self[c][r], &mut temp);
                    self[r][c] = temp;
                }
            }
            return;
        }
        let mut mat = Self::zero(v(self.dim.y, self.dim.x));
        for (r, row) in self.iter().enumerate() {
            for (c, elem) in row.iter().enumerate() {
                mat[c][r] = elem.clone();
            }
        }
        *self = mat;
    }
}

impl<K: Clone + PartialOrd + NumAssign + Signed> Matrix<K> {
    #[track_caller]
    pub fn det(&self) -> K {
        self.assert_sq();
        self.clone().row_ef_det()
    }

    #[inline(always)]
    pub fn row_ef(&mut self) {
        self.row_ef_det();
    }

    fn row_ef_det(&mut self) -> K {
        let mut det = K::one();
        let mut pivot: Vec2<usize> = Vec2::zero();
        while self.in_bounds(pivot) {
            let (mut row, mut value) = (pivot.y, K::zero());
            for r in pivot.y..self.dim.y {
                let v = self[r][pivot.x].clone();
                if v > value {
                    row = r;
                    value = v;
                }
            }
            if value.is_zero() {
                pivot.x += 1;
            } else {
                self.rows.swap(pivot.y, row);
                det = -det;
                for row in pivot.y + 1..self.dim.y {
                    let f = self[row][pivot.x].clone() / value.clone();

                    self[row][pivot.x] = K::zero();

                    for col in pivot.x + 1..self.dim.x {
                        let v = self[pivot.y][col].clone();
                        self[row][col] -= v * f.clone();
                    }
                }
                pivot += v(1, 1);
            }
        }
        for i in 0..self.dim.x.min(self.dim.y) {
            det *= self[i][i].clone();
        }
        det
    }

    pub fn rref(&mut self) {
        self.row_ef();
        for row in (0..self.dim.y).rev() {
            if let Some(col) = self[row].leading_coeff() {
                let value = mem::replace(&mut self[row][col], K::one());
                for c in col + 1..self.dim.x {
                    self[row][c] /= value.clone();
                }
                for r in 0..row {
                    let value = &self[r][col];
                    if !value.is_zero() {
                        let value = value.clone();
                        self[r][col] = K::zero();
                        for c in col + 1..self.dim.x {
                            let v = self[row][c].clone();
                            self[r][c] -= v * value.clone();
                        }
                    }
                }
            }
        }
    }

    #[track_caller]
    pub fn solve(&self, rhs: &Self) -> Option<Self> {
        self.assert_sq();
        let size = self.dim.x;
        let mut mat = self.clone();
        mat.augment(rhs.clone());
        mat.rref();
        if mat[size - 1][size - 1].is_zero() {
            return None;
        }
        mat.remove_lhs(size);
        Some(mat)
    }

    #[track_caller]
    pub fn invert(&mut self) {
        self.assert_sq();
        let size = self.dim.x;
        self.augment(Self::id(self.dim.x));
        self.rref();
        if self[size - 1][size - 1].is_zero() {
            panic!("matrix is not invertible");
        }
        self.remove_lhs(size);
    }

    pub fn adjugate(&mut self) {
        self.cofactor();
        self.transpose();
    }

    pub fn cofactor(&mut self) {
        todo!()
    }

    #[allow(unused)]
    #[track_caller]
    pub fn pow(&self, exponent: i32) -> Self {
        self.assert_sq();
        todo!()
    }
}

impl<K: Add<Output = K> + Mul<Output = K>> Mul<Matrix<K>> for Matrix<K> {
    type Output = Self;

    #[allow(unused)]
    #[track_caller]
    fn mul(self, rhs: Matrix<K>) -> Self::Output {
        todo!()
    }
}

impl<K: Zero> Row<K> {
    fn leading_coeff(&self) -> Option<usize> {
        for (col, val) in self.elems.iter().enumerate() {
            if !val.is_zero() {
                return Some(col);
            }
        }
        None
    }
}
