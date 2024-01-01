use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use crate::{
    matrix::{Matrix, Row},
    traits::Idx,
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
}

impl<T, I: Idx> Index<Vec2<I>> for Matrix<T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: Vec2<I>) -> &Self::Output {
        self.get(index).expect("position out of bounds")
    }
}

impl<T, I: Idx> IndexMut<Vec2<I>> for Matrix<T> {
    #[track_caller]
    fn index_mut(&mut self, index: Vec2<I>) -> &mut Self::Output {
        self.get_mut(index).expect("position out of bounds")
    }
}

impl<T, I: Idx> Index<I> for Matrix<T> {
    type Output = Row<T>;

    #[track_caller]
    fn index(&self, index: I) -> &Self::Output {
        let index = index.try_into().ok().expect("row index out of bounds");
        if index >= self.rows.len() {
            panic!("row index out of bounds");
        }
        &self.rows[index]
    }
}

impl<T, I: Idx> IndexMut<I> for Matrix<T> {
    #[track_caller]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = index.try_into().ok().expect("row index out of bounds");
        if index >= self.rows.len() {
            panic!("row index out of bounds");
        }
        &mut self.rows[index]
    }
}

impl<T, I: Idx> Index<I> for Row<T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: I) -> &Self::Output {
        let index = index.try_into().ok().expect("column index out of bounds");
        if index >= self.len() {
            panic!("column index out of bounds");
        }
        &self.elems[index]
    }
}

impl<T, I: Idx> IndexMut<I> for Row<T> {
    #[track_caller]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = index.try_into().ok().expect("column index out of bounds");
        if index >= self.len() {
            panic!("column index out of bounds");
        }
        &mut self.elems[index]
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
