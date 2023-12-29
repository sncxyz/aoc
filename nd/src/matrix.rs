mod iter;
mod linalg;
mod ops;

use core::fmt;

use num_traits::{One, Zero};

use crate::vector::{v, Vec2};

/// A matrix type for use in linear algebra and as a 2D integer grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<K> {
    rows: Vec<Row<K>>,
    pub(crate) dim: Vec2<usize>,
}

impl<K> Matrix<K> {
    /// Creates a new matrix from a nested iterator of elements in rows.
    #[track_caller]
    pub fn new(rows: impl IntoIterator<Item = impl IntoIterator<Item = K>>) -> Self {
        let mut rows: Vec<_> = rows
            .into_iter()
            .map(IntoIterator::into_iter)
            .map(Iterator::collect)
            .map(Row::new)
            .collect();
        rows.shrink_to_fit();
        let dim = get_dim(&rows);
        Self { rows, dim }
    }

    fn new_unchecked(
        dim: Vec2<usize>,
        rows_iter: impl IntoIterator<Item = impl IntoIterator<Item = K>>,
    ) -> Self {
        let mut rows = Vec::with_capacity(dim.y);
        for row in rows_iter {
            let mut elems = Vec::with_capacity(dim.x);
            for e in row {
                elems.push(e);
            }
            rows.push(Row::new(elems));
        }
        Self { rows, dim }
    }

    /// Creates a new matrix from an iterator of elements in row-major order.
    #[track_caller]
    #[allow(unused)]
    pub fn from_flat<T: TryInto<usize>>(dim: Vec2<T>, elems: impl IntoIterator<Item = K>) -> Self {
        let dim = dim
            .try_into_usize()
            .expect("could not convert dim to usize");
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
        Self { rows, dim }
    }

    /// Creates a new matrix by generating elements in row-major order using a function.
    #[track_caller]
    pub fn from_fn<T: TryInto<usize>>(dim: Vec2<T>, mut f: impl FnMut() -> K) -> Self {
        let dim = dim
            .try_into_usize()
            .expect("could not convert dim to usize");
        let mut rows = Vec::with_capacity(dim.y);
        for _ in 0..dim.y {
            let mut elems = Vec::with_capacity(dim.x);
            for _ in 0..dim.x {
                elems.push(f());
            }
            rows.push(Row::new(elems));
        }
        Self { rows, dim }
    }

    /// Creates a new column vector / matrix with one column.
    pub fn col(elems: impl IntoIterator<Item = K>) -> Self {
        let mut rows: Vec<_> = elems.into_iter().map(|elem| Row::new(vec![elem])).collect();
        rows.shrink_to_fit();
        let dim = v(1, rows.len());
        Self { rows, dim }
    }

    /// Creates a new row vector / matrix with one row.
    pub fn row(elems: impl IntoIterator<Item = K>) -> Self {
        let rows = vec![Row::new(elems.into_iter().collect())];
        let dim = v(rows[0].len(), 1);
        Self { rows, dim }
    }

    /// Returns a shared reference to the element at the given position, or `None` if the position is out of bounds.
    pub fn get<T: TryInto<usize>>(&self, pos: Vec2<T>) -> Option<&K> {
        self.get_in_bounds(pos)
            .map(|pos| &self.rows[pos.y].elems[pos.x])
    }

    /// Returns a mutable reference to the element at the given position, or `None` if the position is out of bounds.
    pub fn get_mut<T: TryInto<usize>>(&mut self, pos: Vec2<T>) -> Option<&mut K> {
        self.get_in_bounds(pos)
            .map(|pos| &mut self.rows[pos.y].elems[pos.x])
    }

    /// Returns whether the given position is within the bounds of the matrix.
    pub fn in_bounds<T: TryInto<usize>>(&self, pos: Vec2<T>) -> bool {
        self.get_in_bounds(pos).is_some()
    }

    #[inline(always)]
    fn get_in_bounds<T: TryInto<usize>>(&self, pos: Vec2<T>) -> Option<Vec2<usize>> {
        pos.try_into_usize()
            .and_then(|pos| (pos.x < self.dim.x && pos.y < self.dim.y).then_some(pos))
    }

    /// Returns the dimensions of the matrix.
    #[track_caller]
    pub fn dim<T>(&self) -> Vec2<T>
    where
        usize: TryInto<T>,
    {
        self.dim
            .try_from_usize()
            .expect("could not convert dim to type T")
    }

    /// Returns the number of columns in the matrix.
    #[track_caller]
    pub fn width<T>(&self) -> T
    where
        usize: TryInto<T>,
    {
        self.dim
            .x
            .try_into()
            .ok()
            .expect("could not convert width to type T")
    }

    /// Returns the number of rows in the matrix.
    #[track_caller]
    pub fn height<T>(&self) -> T
    where
        usize: TryInto<T>,
    {
        self.dim
            .y
            .try_into()
            .ok()
            .expect("could not convert height to type T")
    }

    pub fn map_into<L>(self, f: fn(K) -> L) -> Matrix<L> {
        Matrix::new_unchecked(self.dim, self.into_iter().map(|row| row.into_iter().map(f)))
    }

    #[track_caller]
    fn assert_dim_eq(&self, other: &Self) {
        if self.dim != other.dim {
            panic!("matrices not the same dimensions");
        }
    }
}

impl<K, R: IntoIterator<Item = K>> FromIterator<R> for Matrix<K> {
    #[track_caller]
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl<K: Default> Matrix<K> {
    #[track_caller]
    /// Returns a matrix initialised with the default value for every element.
    pub fn default<T: TryInto<usize>>(dim: Vec2<T>) -> Self {
        Self::from_fn(dim, Default::default)
    }
}

impl<K: Clone> Matrix<K> {
    /// Creates a new matrix with the given dimensions and all elements initialised to the given value.
    #[track_caller]
    pub fn init<T: TryInto<usize>>(dim: Vec2<T>, value: K) -> Self {
        let dim = dim
            .try_into_usize()
            .expect("could not convert dim to usize");
        Self {
            rows: vec![Row::new(vec![value; dim.x]); dim.y],
            dim,
        }
    }
}

impl<K: Clone + Zero> Matrix<K> {
    /// Creates a new matrix with all 0 values.
    #[track_caller]
    pub fn zero<T: TryInto<usize>>(dim: Vec2<T>) -> Self {
        Self::init(dim, K::zero())
    }
}

impl<K: Clone + One> Matrix<K> {
    /// Creates a new matrix with all 1 values.
    #[track_caller]
    pub fn one<T: TryInto<usize>>(dim: Vec2<T>) -> Self {
        Self::init(dim, K::one())
    }
}

impl<K: Clone + Zero + One> Matrix<K> {
    /// Creates an identity matrix.
    #[track_caller]
    pub fn id<T: TryInto<usize>>(dim: T) -> Self {
        let dim = dim.try_into().ok().expect("could not convert dim to usize");
        let mut mat = Self::zero(v(dim, dim));
        for i in 0..dim {
            mat[i][i] = K::one();
        }
        mat
    }
}

/// An individual row of a matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row<K> {
    elems: Vec<K>,
}

impl<K> Row<K> {
    #[inline(always)]
    fn new(elems: Vec<K>) -> Self {
        Self { elems }
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.elems.len()
    }
}

#[track_caller]
fn get_dim<K>(rows: &[Row<K>]) -> Vec2<usize> {
    for w in rows.windows(2) {
        if w[0].len() != w[1].len() {
            panic!("rows not all the same size");
        }
    }
    if rows.is_empty() {
        v(0, 0)
    } else {
        v(rows[0].len(), rows.len())
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.dim.x == 0 || self.dim.y == 0 {
            return write!(f, "{}x{}", self.width::<usize>(), self.height::<usize>());
        }
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
