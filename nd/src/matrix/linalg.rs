// TODO: sort out use of Clone and references

use std::{
    mem,
    ops::{Add, AddAssign, Mul},
};

use num_traits::{NumAssign, Signed, Zero};

use crate::{
    matrix::{Matrix, Row},
    vector::v,
    Vec2,
};

impl<K> Matrix<K> {
    /// Returns whether `self` is square.
    #[inline(always)]
    pub fn is_square(&self) -> bool {
        self.get_dim().x == self.get_dim().y
    }

    /// Augments another matrix of the same height onto the right of `self`.
    ///
    /// Panics if `self` and `other` do not have the same height.
    #[track_caller]
    pub fn augment(&mut self, other: Self) {
        if self.get_dim().y != other.get_dim().y {
            panic!("matrix heights are not the same");
        }
        for (row, mut add) in self.iter_mut().zip(other) {
            row.elems.append(&mut add.elems);
        }
    }

    /// Transposes `self`.
    pub fn transpose(&mut self) {
        let dim = self.get_dim();
        let dim = v(dim.y, dim.x);
        let mut rows = Vec::with_capacity(dim.y);
        for _ in 0..dim.y {
            rows.push(Row::new(Vec::with_capacity(dim.x)));
        }
        for row in self.rows.drain(..) {
            for (i, e) in row.into_iter().enumerate() {
                rows[i].elems.push(e);
            }
        }
        self.rows = rows;
    }

    fn remove_lhs(&mut self, len: usize) {
        for row in self.iter_mut() {
            row.elems.drain(..len);
        }
    }

    #[track_caller]
    fn assert_sq(&self) {
        if !self.is_square() {
            panic!("matrix is not square");
        }
    }
}

impl<K: Clone + PartialOrd + NumAssign + Signed> Matrix<K> {
    /// Returns the determinant of `self`.
    ///
    /// Panics if `self` is not square.
    #[track_caller]
    pub fn det(&self) -> K {
        // TODO: allow this to work with integers
        self.assert_sq();
        self.clone().row_ef_det()
    }

    /// Transforms `self` into row echelon form.
    #[inline(always)]
    pub fn row_ef(&mut self) {
        self.row_ef_det();
    }

    fn row_ef_det(&mut self) -> K {
        let dim = self.get_dim();
        let mut det = K::one();
        let mut pivot = Vec2::zero();
        while self.in_bounds(pivot) {
            let (mut row, mut value) = (pivot.y, K::zero());
            for r in pivot.y..dim.y {
                let v = self[r][pivot.x].abs();
                if v > value {
                    row = r;
                    value = v;
                }
            }
            let value = self[row][pivot.x].clone();
            if value.is_zero() {
                pivot.x += 1;
            } else {
                self.rows.swap(pivot.y, row);
                det = -det;
                for row in pivot.y + 1..dim.y {
                    let f = self[row][pivot.x].clone() / value.clone();

                    self[row][pivot.x] = K::zero();

                    for col in pivot.x + 1..dim.x {
                        let v = self[pivot.y][col].clone();
                        self[row][col] -= v * f.clone();
                    }
                }
                pivot += v(1, 1);
            }
        }
        for i in 0..dim.x.min(dim.y) {
            det *= self[i][i].clone();
        }
        det
    }

    /// Transforms `self` into reduced row echelon form.
    pub fn rref(&mut self) {
        self.row_ef();
        let dim = self.get_dim();
        for row in (0..dim.y).rev() {
            if let Some(col) = self[row].leading_coeff() {
                let value = mem::replace(&mut self[row][col], K::one());
                for c in col + 1..dim.x {
                    self[row][c] /= value.clone();
                }
                for r in 0..row {
                    let value = &self[r][col];
                    if !value.is_zero() {
                        let value = value.clone();
                        self[r][col] = K::zero();
                        for c in col + 1..dim.x {
                            let v = self[row][c].clone();
                            self[r][c] -= v * value.clone();
                        }
                    }
                }
            }
        }
    }

    /// Returns the solution to the equation `self * x = rhs`, or `None` if `self` is singular.
    ///
    /// Panics if `self` is not square.
    #[track_caller]
    pub fn solve(mut self, rhs: Self) -> Option<Self> {
        self.assert_sq();
        let size = self.get_dim().x;
        self.augment(rhs.clone());
        self.rref();
        if self[size - 1][size - 1].is_zero() {
            return None;
        }
        self.remove_lhs(size);
        Some(self)
    }

    /// Inverts `self`.
    ///
    /// Panics if `self` is not square, or if it is singular.
    #[track_caller]
    pub fn invert(&mut self) {
        self.assert_sq();
        let size = self.get_dim().x;
        self.augment(Self::id(size));
        self.rref();
        if self[size - 1][size - 1].is_zero() {
            panic!("matrix is not invertible");
        }
        self.remove_lhs(size);
    }

    /// Transforms `self` into the adjugate of itself,
    /// where the adjugate is defined to be the transpose of the matrix of cofactors.
    pub fn adjugate(&mut self) {
        self.cofactor();
        self.transpose();
    }

    /// Transforms `self` into its matrix of cofactors.
    pub fn cofactor(&mut self) {
        todo!()
    }

    /// Returns `self` raised to the given power.
    ///
    /// Panics if `self` is not square.
    #[allow(unused)]
    #[track_caller]
    pub fn pow(&self, exponent: i32) -> Self {
        self.assert_sq();
        todo!()
    }
}

impl<'a, 'b, K: AddAssign> Mul<&'b Matrix<K>> for &'a Matrix<K>
where
    &'a K: Mul<&'b K, Output = K>,
{
    type Output = Matrix<K>;

    #[track_caller]
    fn mul(self, rhs: &'b Matrix<K>) -> Self::Output {
        let dim_l = self.get_dim();
        let dim_r = rhs.get_dim();
        if dim_l.x != dim_r.y {
            panic!("width of LHS does not match height of RHS");
        }
        let len = dim_l.x;
        let dim = v(dim_r.x, dim_l.y);
        let mut rows = Vec::with_capacity(dim.y);
        for r in 0..dim.y {
            let mut elems = Vec::with_capacity(dim.x);
            for c in 0..dim.x {
                let mut total = &self[r][0] * &rhs[0][c];
                for k in 1..len {
                    total += &self[r][k] * &rhs[k][c];
                }
                elems.push(total);
            }
            rows.push(Row::new(elems));
        }
        Matrix { rows }
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
