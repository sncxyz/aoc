use std::{
    mem,
    ops::{AddAssign, Mul},
};

use num_traits::{Signed, Zero};

use crate::{
    matrix::{Matrix, Row},
    traits::{Field, FieldOps},
    vector::v,
    Vec2,
};

impl<T> Matrix<T> {
    /// Returns whether `self` is square.
    #[inline(always)]
    pub fn is_square(&self) -> bool {
        self.get_width() == self.get_height()
    }

    /// Augments another matrix of the same height onto the right of `self`.
    ///
    /// Panics if `self` and `other` do not have the same height.
    #[track_caller]
    pub fn augment(&mut self, other: Self) {
        if self.get_height() != other.get_height() {
            panic!(
                "height of self ({}) does not match height of other ({})",
                self.get_height(),
                other.get_height()
            );
        }
        for (row, mut add) in self.iter_mut().zip(other) {
            row.elems.append(&mut add.elems);
        }
    }

    /// Returns the transpose of `self`.
    pub fn transpose(self) -> Self {
        let dim = self.get_dim();
        let dim = v(dim.y, dim.x);
        let mut rows = Vec::with_capacity(dim.y);
        for _ in 0..dim.y {
            rows.push(Row::new(Vec::with_capacity(dim.x)));
        }
        for row in self.rows {
            for (i, e) in row.into_iter().enumerate() {
                rows[i].elems.push(e);
            }
        }
        Self { rows }
    }

    fn rem_lhs(&mut self, len: usize) {
        for row in self.iter_mut() {
            row.elems.drain(..len);
        }
    }

    #[track_caller]
    fn assert_sq(&self) {
        if !self.is_square() {
            panic!(
                "matrix is not square ({}x{})",
                self.get_width(),
                self.get_height()
            );
        }
    }
}

impl<T: Field + Signed + PartialOrd> Matrix<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the determinant of `self`.
    ///
    /// Panics if `self` is not square.
    #[track_caller]
    pub fn det(&self) -> T {
        // TODO: allow this to work with integers
        self.assert_sq();
        self.clone().row_ef_det()
    }

    /// Transforms `self` into row echelon form.
    #[inline(always)]
    pub fn row_ef(&mut self) {
        self.row_ef_det();
    }

    fn row_ef_det(&mut self) -> T {
        let dim = self.get_dim();
        let mut det = T::one();
        let mut pivot = Vec2::zero();
        while self.in_bounds(pivot) {
            let (mut max_row, mut max_abs_value) = (pivot.y, T::zero());
            for row in pivot.y..dim.y {
                let abs_value = self[row][pivot.x].abs();
                if abs_value > max_abs_value {
                    max_row = row;
                    max_abs_value = abs_value;
                }
            }
            let max_value = &self[max_row][pivot.x];
            if max_value.is_zero() {
                pivot.x += 1;
            } else {
                let pivot_coeff = max_value.clone();
                self.rows.swap(pivot.y, max_row);
                det = -det;
                for row in pivot.y + 1..dim.y {
                    let coeff = &self[row][pivot.x];
                    if !coeff.is_zero() {
                        let ratio = coeff / &pivot_coeff;
                        self[row][pivot.x] = T::zero();
                        for col in pivot.x + 1..dim.x {
                            let sub = &self[pivot.y][col] * &ratio;
                            self[row][col] -= sub;
                        }
                    }
                }
                pivot += v(1, 1);
            }
        }
        for i in 0..dim.x.min(dim.y) {
            det *= &self[i][i];
        }
        det
    }

    /// Transforms `self` into reduced row echelon form.
    pub fn rref(&mut self) {
        self.row_ef();
        let dim = self.get_dim();
        for row in (0..dim.y).rev() {
            if let Some(col) = self[row].leading_coeff() {
                let coeff = mem::replace(&mut self[row][col], T::one());
                for c in col + 1..dim.x {
                    self[row][c] /= &coeff;
                }
                for r in 0..row {
                    if !self[r][col].is_zero() {
                        let ratio = mem::replace(&mut self[r][col], T::zero());
                        for c in col + 1..dim.x {
                            let diff = &self[row][c] * &ratio;
                            self[r][c] -= diff;
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
        let size = self.get_width();
        self.augment(rhs);
        self.rref();
        if self[size - 1][size - 1].is_zero() {
            return None;
        }
        self.rem_lhs(size);
        Some(self)
    }

    /// Returns the inverse of `self`, or `None` if `self` is singular.
    ///
    /// Panics if `self` is not square.
    #[track_caller]
    pub fn inverse(self) -> Option<Self> {
        let size = self.get_width();
        self.solve(Self::id(size))
    }

    /// Returns the transpose of the matrix of cofactors of `self`.
    pub fn adjugate(self) -> Self {
        self.cofactor().transpose()
    }

    /// Returns the matrix of cofactors of `self`.
    #[allow(unused)]
    pub fn cofactor(mut self) -> Self {
        todo!()
    }

    /// Returns `self` raised to the given power.
    ///
    /// Panics if `self` is not square.
    #[allow(unused)]
    #[track_caller]
    pub fn pow(mut self, exponent: i32) -> Self {
        self.assert_sq();
        todo!()
    }
}

impl<'a, 'b, T: AddAssign> Mul<&'b Matrix<T>> for &'a Matrix<T>
where
    &'a T: Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    #[track_caller]
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let dim_l = self.get_dim();
        let dim_r = rhs.get_dim();
        if dim_l.x != dim_r.y {
            panic!(
                "width of LHS ({}) does not match height of RHS ({})",
                dim_l.x, dim_r.y
            );
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

impl<T: Zero> Row<T> {
    fn leading_coeff(&self) -> Option<usize> {
        for (col, val) in self.elems.iter().enumerate() {
            if !val.is_zero() {
                return Some(col);
            }
        }
        None
    }
}
