use std::{marker, slice, vec};

use crate::{
    matrix::{Matrix, Row},
    traits::Idx,
    vector::{v, Vec2},
};

impl<T> Matrix<T> {
    /// Iterator over shared references to rows in the matrix.
    #[inline(always)]
    pub fn iter(&self) -> slice::Iter<Row<T>> {
        self.into_iter()
    }

    /// Iterator over mutable references to rows in the matrix.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> slice::IterMut<Row<T>> {
        self.into_iter()
    }

    /// Iterator over owned elements in row-major order.
    pub fn into_iter_all(self) -> impl Iterator<Item = T> {
        self.into_iter().flat_map(IntoIterator::into_iter)
    }

    /// Iterator over shared references to elements in row-major order.
    pub fn iter_all(&self) -> impl Iterator<Item = &T> {
        self.into_iter().flat_map(IntoIterator::into_iter)
    }

    /// Iterator over mutable references to elements in row-major order.
    pub fn iter_mut_all(&mut self) -> impl Iterator<Item = &mut T> {
        self.into_iter().flat_map(IntoIterator::into_iter)
    }

    /// Iterator over positions in the matrix in row-major order.
    #[inline]
    pub fn positions<I: Idx>(&self) -> Positions<I> {
        Positions::new(self.get_dim())
    }

    /// Iterator over owned elements and their positions in row-major order.
    pub fn into_enumerate<I: Idx>(self) -> impl Iterator<Item = (Vec2<I>, T)> {
        Positions::new(self.get_dim()).zip(self.into_iter_all())
    }

    /// Iterator over shared references to elements and their positions in row-major order.
    pub fn enumerate<I: Idx>(&self) -> impl Iterator<Item = (Vec2<I>, &T)> {
        Positions::new(self.get_dim()).zip(self.iter_all())
    }

    /// Iterator over get_dim()utable references to elements and their positions in row-major order.
    pub fn enumerate_mut<I: Idx>(&mut self) -> impl Iterator<Item = (Vec2<I>, &mut T)> {
        Positions::new(self.get_dim()).zip(self.iter_mut_all())
    }
}

impl<T> Row<T> {
    /// Iterator over shared references to elements in the row.
    #[inline(always)]
    pub fn iter(&self) -> slice::Iter<T> {
        self.into_iter()
    }

    /// Iterator over mutable references to elements in the row.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = Row<T>;
    type IntoIter = vec::IntoIter<Row<T>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a Row<T>;
    type IntoIter = slice::Iter<'a, Row<T>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut Row<T>;
    type IntoIter = slice::IterMut<'a, Row<T>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter_mut()
    }
}

impl<T> IntoIterator for Row<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Row<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Row<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter_mut()
    }
}

/// Iterator over positions in a matrix in row-major order.
pub struct Positions<I> {
    pos: Vec2<usize>,
    dim: Vec2<usize>,
    _phantom: marker::PhantomData<I>,
}

impl<I> Positions<I> {
    #[inline]
    fn new(dim: Vec2<usize>) -> Self {
        Self {
            pos: v(0, 0),
            dim,
            _phantom: marker::PhantomData,
        }
    }
}

impl<I: Idx> Iterator for Positions<I> {
    type Item = Vec2<I>;

    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        if self.dim.x == 0 || self.pos.y == self.dim.y {
            return None;
        }
        let ret = self.pos;
        self.pos.x += 1;
        if self.pos.x == self.dim.x {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        Some(
            ret.try_from_usize()
                .expect("could not convert position to type T"),
        )
    }
}
