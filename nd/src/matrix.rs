// TODO: better error messages for index conversion to and from usize.

mod iter;
mod linalg;
mod ops;

use std::fmt;

use num_traits::{One, Zero};

use crate::{
    traits::Idx,
    vector::{v, Vec2},
};

/// A matrix type for use in linear algebra and as a 2D integer grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    rows: Vec<Row<T>>,
}

impl<T> Matrix<T> {
    /// Creates a new matrix from a nested iterator of elements in rows.
    ///
    /// Panics if the rows are not all the same size, or if the matrix would be empty.
    #[track_caller]
    pub fn new(rows: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        let mut rows: Vec<_> = rows
            .into_iter()
            .map(IntoIterator::into_iter)
            .map(Iterator::collect)
            .map(Row::new)
            .collect();
        assert_valid(&rows);
        rows.shrink_to_fit();
        Self { rows }
    }

    /// Creates a new matrix from an iterator of elements in row-major order.
    ///
    /// Panics if the dimensions fail to convert to `usize`, if the matrix would be empty,
    /// or if the iterator does not yield the right number of elements to fill the dimensions exactly.
    #[track_caller]
    #[allow(unused)]
    pub fn from_flat<I: Idx>(dim: Vec2<I>, elems: impl IntoIterator<Item = T>) -> Self {
        let dim = dim
            .try_into_usize()
            .expect("could not convert dim to usize");
        dim.assert_nonempty();
        let mut elems_iter = elems.into_iter();
        let mut rows = Vec::with_capacity(dim.y);
        for _ in 0..dim.y {
            let mut elems = Vec::with_capacity(dim.x);
            for _ in 0..dim.x {
                elems.push(elems_iter.next().expect("not enough elements in iterator"));
            }
            rows.push(Row::new(elems));
        }
        if elems_iter.next().is_some() {
            panic!("too many elements in iterator");
        }
        Self { rows }
    }

    /// Creates a new matrix by generating elements in row-major order using a function.
    ///
    /// Panics if the dimensions fail to convert to `usize`, or if the matrix would be empty.
    #[track_caller]
    pub fn from_fn<I: Idx>(dim: Vec2<I>, mut f: impl FnMut() -> T) -> Self {
        let dim = dim
            .try_into_usize()
            .expect("could not convert dim to usize");
        dim.assert_nonempty();
        let mut rows = Vec::with_capacity(dim.y);
        for _ in 0..dim.y {
            let mut elems = Vec::with_capacity(dim.x);
            for _ in 0..dim.x {
                elems.push(f());
            }
            rows.push(Row::new(elems));
        }
        Self { rows }
    }

    /// Creates a new column vector / matrix with one column.
    ///
    /// Panics if the matrix would be empty.
    pub fn col(elems: impl IntoIterator<Item = T>) -> Self {
        let mut rows: Vec<_> = elems.into_iter().map(|elem| Row::new(vec![elem])).collect();
        rows.shrink_to_fit();
        let dim = v(1, rows.len());
        dim.assert_nonempty();
        Self { rows }
    }

    /// Creates a new row vector / matrix with one row.
    ///
    /// Panics if the matrix would be empty.
    pub fn row(elems: impl IntoIterator<Item = T>) -> Self {
        let rows = vec![Row::new(elems.into_iter().collect())];
        let dim = v(rows[0].len(), 1);
        dim.assert_nonempty();
        Self { rows }
    }

    /// Returns a shared reference to the element at the given position, or `None` if the position is out of bounds.
    pub fn get<I: Idx>(&self, pos: Vec2<I>) -> Option<&T> {
        self.get_in_bounds(pos)
            .map(|pos| &self.rows[pos.y].elems[pos.x])
    }

    /// Returns a mutable reference to the element at the given position, or `None` if the position is out of bounds.
    pub fn get_mut<I: Idx>(&mut self, pos: Vec2<I>) -> Option<&mut T> {
        self.get_in_bounds(pos)
            .map(|pos| &mut self.rows[pos.y].elems[pos.x])
    }

    /// Returns whether the given position is within the bounds of the matrix.
    pub fn in_bounds<I: Idx>(&self, pos: Vec2<I>) -> bool {
        self.get_in_bounds(pos).is_some()
    }

    fn get_in_bounds<I: Idx>(&self, pos: Vec2<I>) -> Option<Vec2<usize>> {
        let dim = self.get_dim();
        pos.try_into_usize()
            .and_then(|pos| (pos.x < dim.x && pos.y < dim.y).then_some(pos))
    }

    #[inline]
    pub(crate) fn get_dim(&self) -> Vec2<usize> {
        v(self.rows[0].len(), self.rows.len())
    }

    /// Returns the dimensions of the matrix.
    ///
    /// Panics if the dimensions cannot be converted to the type `T`.
    #[track_caller]
    pub fn dim<I: Idx>(&self) -> Vec2<I> {
        self.get_dim()
            .try_from_usize()
            .expect("could not convert dim to type T")
    }

    /// Returns the number of columns in the matrix.
    ///
    /// Panics if the value cannot be converted to the type `T`.
    #[track_caller]
    pub fn width<I: Idx>(&self) -> I {
        self.rows[0]
            .len()
            .try_into()
            .ok()
            .expect("could not convert width to type T")
    }

    /// Returns the number of rows in the matrix.
    ///
    /// Panics if the value cannot be converted to the type `T`.
    #[track_caller]
    pub fn height<I: Idx>(&self) -> I {
        self.rows
            .len()
            .try_into()
            .ok()
            .expect("could not convert height to type T")
    }

    #[track_caller]
    fn assert_dim_eq(&self, other: &Self) {
        if self.get_dim() != other.get_dim() {
            panic!("matrices not the same dimensions");
        }
    }
}

impl<T, R: IntoIterator<Item = T>> FromIterator<R> for Matrix<T> {
    #[track_caller]
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl<T: Default> Matrix<T> {
    /// Returns a matrix initialised with the default value for every element.
    ///
    /// Panics if the dimensions fail to convert to `usize`, or if the matrix would be empty.
    #[track_caller]
    pub fn default<I: Idx>(dim: Vec2<I>) -> Self {
        Self::from_fn(dim, Default::default)
    }
}

impl<T: Clone> Matrix<T> {
    /// Creates a new matrix with the given dimensions and all elements initialised to the given value.
    ///
    /// Panics if the dimensions fail to convert to `usize`, or if the matrix would be empty.
    #[track_caller]
    pub fn init<I: Idx>(dim: Vec2<I>, value: T) -> Self {
        let dim = dim
            .try_into_usize()
            .expect("could not convert dim to usize");
        dim.assert_nonempty();
        Self {
            rows: vec![Row::new(vec![value; dim.x]); dim.y],
        }
    }
}

impl<T: Clone + Zero + One> Matrix<T> {
    /// Creates an identity matrix.
    ///
    /// Panics if the dimensions fail to convert to `usize`, or if the matrix would be empty.
    #[track_caller]
    pub fn id<I: Idx>(dim: I) -> Self {
        let dim = dim.try_into().ok().expect("could not convert dim to usize");
        let dim = v(dim, dim);
        let mut mat = Self::init(dim, T::zero());
        for i in 0..dim.x {
            mat[i][i] = T::one();
        }
        mat
    }
}

/// An individual row of a matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row<T> {
    elems: Vec<T>,
}

impl<T> Row<T> {
    #[inline(always)]
    fn new(elems: Vec<T>) -> Self {
        Self { elems }
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.elems.len()
    }
}

impl Vec2<usize> {
    #[track_caller]
    fn assert_nonempty(self) {
        if self.x == 0 || self.y == 0 {
            panic!("matrix has zero size");
        }
    }
}

#[track_caller]
fn assert_valid<T>(rows: &[Row<T>]) {
    for w in rows.windows(2) {
        if w[0].len() != w[1].len() {
            panic!("rows not all the same size");
        }
    }
    let dim = if rows.is_empty() {
        v(0, 0)
    } else {
        v(rows[0].len(), rows.len())
    };
    dim.assert_nonempty();
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings: Matrix<_> = self
            .iter()
            .map(|row| row.iter().map(ToString::to_string))
            .collect();
        let longest = strings.iter_all().map(|s| s.chars().count()).max().unwrap();

        writeln!(f, "{}x{}", self.width::<usize>(), self.height::<usize>())?;

        for (i, row) in strings.iter().enumerate() {
            for string in row {
                write!(f, "{}{string} ", " ".repeat(longest - string.len()))?;
            }
            if i < strings.height::<usize>() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
