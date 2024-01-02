use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use crate::{
    matrix::{Matrix, Row},
    traits::Pos,
    vector::Vec2,
};

impl<T> Matrix<T> {
    #[track_caller]
    fn new_unchecked(&self, rows_iter: impl Iterator<Item = impl Iterator<Item = T>>) -> Self {
        let dim = self.get_dim();
        let mut rows = Vec::with_capacity(dim.y);
        for row in rows_iter {
            let mut elems = Vec::with_capacity(dim.x);
            for e in row {
                elems.push(e);
            }
            rows.push(Row::new(elems));
        }
        Self { rows }
    }

    #[track_caller]
    fn assert_dim_eq(&self, other: &Self) {
        if self.get_dim() != other.get_dim() {
            panic!(
                "dimensions of self ({}x{}) do not match dimensions of other ({}x{})",
                self.get_width(),
                self.get_height(),
                other.get_width(),
                other.get_height()
            );
        }
    }

    #[track_caller]
    fn pos_in_bounds<P: Pos>(&self, pos: Vec2<P>) -> Vec2<usize> {
        if let Some(i) = self.get_in_bounds(pos.clone()) {
            i
        } else {
            panic!(
                "position out of bounds: dimensions are {} but position is ({:?}, {:?})",
                self.get_dim(),
                pos.x,
                pos.y
            );
        }
    }

    #[track_caller]
    fn row_in_bounds<P: Pos>(&self, row: P) -> usize {
        if let Some(i) = row
            .get_index()
            .and_then(|i| (i < self.get_height()).then_some(i))
        {
            i
        } else {
            panic!(
                "row index out of bounds: height is {} but row index is {row:?}",
                self.get_height()
            );
        }
    }
}

impl<T> Row<T> {
    #[track_caller]
    fn col_in_bounds<P: Pos>(&self, col: P) -> usize {
        if let Some(i) = col.get_index().and_then(|i| (i < self.len()).then_some(i)) {
            i
        } else {
            panic!(
                "column index out of bounds: width is {} but column index is {col:?}",
                self.len()
            );
        }
    }
}

impl<T, P: Pos> Index<Vec2<P>> for Matrix<T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: Vec2<P>) -> &Self::Output {
        let pos = self.pos_in_bounds(index);
        &self.rows[pos.y].elems[pos.x]
    }
}

impl<T, P: Pos> IndexMut<Vec2<P>> for Matrix<T> {
    #[track_caller]
    fn index_mut(&mut self, index: Vec2<P>) -> &mut Self::Output {
        let pos = self.pos_in_bounds(index);
        &mut self.rows[pos.y].elems[pos.x]
    }
}

impl<T, P: Pos> Index<P> for Matrix<T> {
    type Output = Row<T>;

    #[track_caller]
    fn index(&self, index: P) -> &Self::Output {
        let row = self.row_in_bounds(index);
        &self.rows[row]
    }
}

impl<T, P: Pos> IndexMut<P> for Matrix<T> {
    #[track_caller]
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        let row = self.row_in_bounds(index);
        &mut self.rows[row]
    }
}

impl<T, P: Pos> Index<P> for Row<T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: P) -> &Self::Output {
        &self.elems[self.col_in_bounds(index)]
    }
}

impl<T, P: Pos> IndexMut<P> for Row<T> {
    #[track_caller]
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        let col = self.col_in_bounds(index);
        &mut self.elems[col]
    }
}

macro_rules! impl_term {
    ($tr:ident, $f:ident, $op:tt, $tr_a:ident, $f_a:ident, $op_a:tt) => {
        impl<'a, 'b, T> $tr<&'b Matrix<T>> for &'a Matrix<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            #[track_caller]
            fn $f(self, rhs: &'b Matrix<T>) -> Self::Output {
                self.assert_dim_eq(rhs);
                self.new_unchecked(
                    self.iter()
                        .zip(rhs)
                        .map(|(a, b)| a.iter().zip(b).map(|(a, b)| a $op b)),
                )
            }
        }

        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a Matrix<T>> for Matrix<T> {
            #[track_caller]
            fn $f_a(&mut self, rhs: &'a Matrix<T>) {
                self.assert_dim_eq(rhs);
                for (a, b) in self.iter_mut().zip(rhs) {
                    for (a, b) in a.iter_mut().zip(b) {
                        *a $op_a b;
                    }
                }
            }
        }
    };
}

impl_term!(Add, add, +, AddAssign, add_assign, +=);
impl_term!(Sub, sub, -, SubAssign, sub_assign, -=);

macro_rules! impl_factor {
    ($tr:ident, $f:ident, $op:tt, $tr_a:ident, $f_a:ident, $op_a:tt) => {
        impl<'a, 'b, T> $tr<&'b T> for &'a Matrix<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            #[track_caller]
            fn $f(self, rhs: &'b T) -> Self::Output {
                self.new_unchecked(self.iter().map(|r| r.iter().map(|e| e $op rhs)))
            }
        }

        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a T> for Matrix<T> {
            #[track_caller]
            fn $f_a(&mut self, rhs: &'a T) {
                for row in self {
                    for e in row {
                        *e $op_a rhs;
                    }
                }
            }
        }

        impl<'a, T: Copy + $tr<Output = T>> $tr<T> for &'a Matrix<T> {
            type Output = Matrix<T>;

            #[track_caller]
            fn $f(self, rhs: T) -> Self::Output {
                self.new_unchecked(self.iter().map(|r| r.iter().map(|e| *e $op rhs)))
            }
        }

        impl<T: Copy + $tr_a> $tr_a<T> for Matrix<T> {
            #[track_caller]
            fn $f_a(&mut self, rhs: T) {
                for row in self {
                    for e in row {
                        *e $op_a rhs;
                    }
                }
            }
        }
    };
}

impl_factor!(Mul, mul, *, MulAssign, mul_assign, *=);
impl_factor!(Div, div, /, DivAssign, div_assign, /=);
impl_factor!(Rem, rem, %, RemAssign, rem_assign, %=);

impl<'a, T> Neg for &'a Matrix<T>
where
    &'a T: Neg<Output = T>,
{
    type Output = Matrix<T>;

    #[track_caller]
    fn neg(self) -> Self::Output {
        self.new_unchecked(self.iter().map(|r| r.iter().map(|e| -e)))
    }
}
