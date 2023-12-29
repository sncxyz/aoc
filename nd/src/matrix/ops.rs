use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use crate::{
    matrix::{Matrix, Row},
    vector::Vec2,
};

impl<K> Matrix<K> {
    #[track_caller]
    fn zip<K1, K2, I1, I2, R1, R2>(
        dim: Vec2<usize>,
        left: I1,
        right: I2,
        f: fn((K1, K2)) -> K,
    ) -> Self
    where
        I1: IntoIterator<Item = R1>,
        I2: IntoIterator<Item = R2>,
        R1: IntoIterator<Item = K1>,
        R2: IntoIterator<Item = K2>,
    {
        Self::new_unchecked(
            dim,
            left.into_iter()
                .zip(right.into_iter())
                .map(|(l, r)| l.into_iter().zip(r.into_iter()).map(f)),
        )
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
        impl<K: $tr<Output = K>> $tr<Matrix<K>> for Matrix<K> {
            type Output = Self;

            #[track_caller]
            fn $f(self, rhs: Matrix<K>) -> Self::Output {
                self.assert_dim_eq(&rhs);
                Matrix::zip(self.dim, self, rhs, |(a, b)| a $op b)
            }
        }
        impl<'a, K: $tr<&'a K, Output = K>> $tr<&'a Matrix<K>> for Matrix<K> {
            type Output = Self;

            #[track_caller]
            fn $f(self, rhs: &'a Matrix<K>) -> Self::Output {
                self.assert_dim_eq(&rhs);
                Matrix::zip(self.dim, self, rhs, |(a, b)| a $op b)
            }
        }
        impl<'a, K> $tr<Matrix<K>> for &'a Matrix<K>
        where
            &'a K: $tr<K, Output = K>,
        {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: Matrix<K>) -> Self::Output {
                self.assert_dim_eq(&rhs);
                Matrix::zip(self.dim, self, rhs, |(a, b)| a $op b)
            }
        }
        impl<'a, 'b, K> $tr<&'b Matrix<K>> for &'a Matrix<K>
        where
            &'a K: $tr<&'b K, Output = K>,
        {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: &'b Matrix<K>) -> Self::Output {
                self.assert_dim_eq(&rhs);
                Matrix::zip(self.dim, self, rhs, |(a, b)| a $op b)
            }
        }
        impl<K: $tr_a<K>> $tr_a<Matrix<K>> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: Matrix<K>) {
                self.assert_dim_eq(&rhs);
                for (a, b) in self.iter_mut_all().zip(rhs.into_iter_all()) {
                    *a $op_a b;
                }
            }
        }
        impl<'a, K: $tr_a<&'a K>> $tr_a<&'a Matrix<K>> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: &'a Matrix<K>) {
                self.assert_dim_eq(&rhs);
                for (a, b) in self.iter_mut_all().zip(rhs.iter_all()) {
                    *a $op_a b;
                }
            }
        }
    };
}

impl_term!(Add, add, +, AddAssign, add_assign, +=);
impl_term!(Sub, sub, -, SubAssign, sub_assign, -=);

macro_rules! impl_factor {
    ($tr:ident, $f:ident, $op:tt, $tr_a:ident, $f_a:ident, $op_a:tt) => {
        impl<K: Clone + $tr<Output = K>> $tr<K> for Matrix<K> {
            type Output = Self;

            #[track_caller]
            fn $f(self, rhs: K) -> Self::Output {
                Matrix::new_unchecked(
                    self.dim,
                    self.into_iter()
                        .map(|row| row.into_iter().map(|e| e $op rhs.clone())),
                )
            }
        }
        impl<'a, K: $tr<&'a K, Output = K>> $tr<&'a K> for Matrix<K> {
            type Output = Self;

            #[track_caller]
            fn $f(self, rhs: &'a K) -> Self::Output {
                Matrix::new_unchecked(
                    self.dim,
                    self.into_iter()
                        .map(|row| row.into_iter().map(|e| e $op rhs)),
                )
            }
        }
        impl<'a, K> $tr<K> for &'a Matrix<K>
        where
            K: Clone,
            &'a K: $tr<K, Output = K>,
        {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: K) -> Self::Output {
                Matrix::new_unchecked(
                    self.dim,
                    self.into_iter()
                        .map(|row| row.into_iter().map(|e| e $op rhs.clone())),
                )
            }
        }
        impl<'a, 'b, K> $tr<&'b K> for &'a Matrix<K>
        where
            &'a K: $tr<&'b K, Output = K>,
        {
            type Output = Matrix<K>;

            #[track_caller]
            fn $f(self, rhs: &'b K) -> Self::Output {
                Matrix::new_unchecked(
                    self.dim,
                    self.into_iter()
                        .map(|row| row.into_iter().map(|e| e $op rhs)),
                )
            }
        }
        impl<K: Clone + $tr_a<K>> $tr_a<K> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: K) {
                for e in self.iter_mut_all() {
                    *e $op_a rhs.clone();
                }
            }
        }
        impl<'a, K: $tr_a<&'a K>> $tr_a<&'a K> for Matrix<K> {
            #[track_caller]
            fn $f_a(&mut self, rhs: &'a K) {
                for e in self.iter_mut_all() {
                    *e $op_a rhs;
                }
            }
        }
    };
}

impl_factor!(Mul, mul, *, MulAssign, mul_assign, *=);
impl_factor!(Div, div, /, DivAssign, div_assign, /=);
impl_factor!(Rem, rem, %, RemAssign, rem_assign, %=);

impl<K: Neg<Output = K>> Neg for Matrix<K> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Matrix::new_unchecked(
            self.dim,
            self.into_iter().map(|row| row.into_iter().map(|e| -e)),
        )
    }
}

impl<'a, K> Neg for &'a Matrix<K>
where
    &'a K: Neg<Output = K>,
{
    type Output = Matrix<K>;

    fn neg(self) -> Self::Output {
        Matrix::new_unchecked(
            self.dim,
            self.into_iter().map(|row| row.into_iter().map(|e| -e)),
        )
    }
}
