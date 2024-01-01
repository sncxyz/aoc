use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use crate::{
    matrix::{Matrix, Row},
    vector::Vec2,
};

impl<K> Matrix<K> {
    #[track_caller]
    fn new_unchecked(&self, rows_iter: impl Iterator<Item = impl Iterator<Item = K>>) -> Self {
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

impl<K, T: TryInto<usize>> Index<Vec2<T>> for Matrix<K> {
    type Output = K;

    #[track_caller]
    fn index(&self, index: Vec2<T>) -> &Self::Output {
        self.get(index).expect("position out of bounds")
    }
}

impl<K, T: TryInto<usize>> IndexMut<Vec2<T>> for Matrix<K> {
    #[track_caller]
    fn index_mut(&mut self, index: Vec2<T>) -> &mut Self::Output {
        self.get_mut(index).expect("position out of bounds")
    }
}

impl<K, T: TryInto<usize>> Index<T> for Matrix<K> {
    type Output = Row<K>;

    #[track_caller]
    fn index(&self, index: T) -> &Self::Output {
        let index = index.try_into().ok().expect("row index out of bounds");
        if index >= self.rows.len() {
            panic!("row index out of bounds");
        }
        &self.rows[index]
    }
}

impl<K, T: TryInto<usize>> IndexMut<T> for Matrix<K> {
    #[track_caller]
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let index = index.try_into().ok().expect("row index out of bounds");
        if index >= self.rows.len() {
            panic!("row index out of bounds");
        }
        &mut self.rows[index]
    }
}

impl<K, T: TryInto<usize>> Index<T> for Row<K> {
    type Output = K;

    #[track_caller]
    fn index(&self, index: T) -> &Self::Output {
        let index = index.try_into().ok().expect("column index out of bounds");
        if index >= self.len() {
            panic!("column index out of bounds");
        }
        &self.elems[index]
    }
}

impl<K, T: TryInto<usize>> IndexMut<T> for Row<K> {
    #[track_caller]
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let index = index.try_into().ok().expect("column index out of bounds");
        if index >= self.len() {
            panic!("column index out of bounds");
        }
        &mut self.elems[index]
    }
}

macro_rules! impl_term {
    ($tr:ident, $f:ident, $op:tt, $tr_a:ident, $f_a:ident, $op_a:tt) => {
        impl<'a, 'b, K> $tr<&'b Matrix<K>> for &'a Matrix<K>
        where
            &'a K: $tr<&'b K, Output = K>,
        {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: &'b Matrix<K>) -> Self::Output {
                self.assert_dim_eq(rhs);
                self.new_unchecked(
                    self.iter()
                        .zip(rhs)
                        .map(|(a, b)| a.iter().zip(b).map(|(a, b)| a $op b)),
                )
            }
        }

        impl<'a, K: $tr_a<&'a K>> $tr_a<&'a Matrix<K>> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: &'a Matrix<K>) {
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
        impl<'a, 'b, K> $tr<&'b K> for &'a Matrix<K>
        where
            &'a K: $tr<&'b K, Output = K>,
        {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: &'b K) -> Self::Output {
                self.new_unchecked(self.iter().map(|r| r.iter().map(|e| e $op rhs)))
            }
        }

        impl<'a, K: $tr_a<&'a K>> $tr_a<&'a K> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: &'a K) {
                for row in self {
                    for e in row {
                        *e $op_a rhs;
                    }
                }
            }
        }

        impl<'a, K: Copy + $tr<Output = K>> $tr<K> for &'a Matrix<K> {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: K) -> Self::Output {
                self.new_unchecked(self.iter().map(|r| r.iter().map(|e| *e $op rhs)))
            }
        }

        impl<K: Copy + $tr_a> $tr_a<K> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: K) {
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

impl<'a, K> Neg for &'a Matrix<K>
where
    &'a K: Neg<Output = K>,
{
    type Output = Matrix<K>;

    #[track_caller]
    fn neg(self) -> Self::Output {
        self.new_unchecked(self.iter().map(|r| r.iter().map(|e| -e)))
    }
}
