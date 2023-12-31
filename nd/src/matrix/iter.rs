use std::{marker, slice, vec};

use crate::{
    matrix::{Matrix, Row},
    vector::{v, Vec2},
};

impl<K> Matrix<K> {
    /// Iterator over shared references to rows in the matrix.
    #[inline(always)]
    pub fn iter(&self) -> slice::Iter<Row<K>> {
        self.into_iter()
    }

    /// Iterator over mutable references to rows in the matrix.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> slice::IterMut<Row<K>> {
        self.into_iter()
    }

    /// Iterator over owned elements in row-major order.
    pub fn into_iter_all(self) -> impl Iterator<Item = K> {
        self.into_iter().flat_map(IntoIterator::into_iter)
    }

    /// Iterator over shared references to elements in row-major order.
    pub fn iter_all(&self) -> impl Iterator<Item = &K> {
        self.into_iter().flat_map(IntoIterator::into_iter)
    }

    /// Iterator over mutable references to elements in row-major order.
    pub fn iter_mut_all(&mut self) -> impl Iterator<Item = &mut K> {
        self.into_iter().flat_map(IntoIterator::into_iter)
    }

    /// Iterator over positions in the matrix in row-major order.
    #[inline]
    pub fn positions<T>(&self) -> Positions<T>
    where
        usize: TryInto<T>,
    {
        Positions::new(self.get_dim())
    }

    /// Iterator over owned elements and their positions in row-major order.
    pub fn into_enumerate<T>(self) -> impl Iterator<Item = (Vec2<T>, K)>
    where
        usize: TryInto<T>,
    {
        Positions::new(self.get_dim()).zip(self.into_iter_all())
    }

    /// Iterator over shared references to elements and their positions in row-major order.
    pub fn enumerate<T>(&self) -> impl Iterator<Item = (Vec2<T>, &K)>
    where
        usize: TryInto<T>,
    {
        Positions::new(self.get_dim()).zip(self.iter_all())
    }

    /// Iterator over get_dim()utable references to elements and their positions in row-major order.
    pub fn enumerate_mut<T>(&mut self) -> impl Iterator<Item = (Vec2<T>, &mut K)>
    where
        usize: TryInto<T>,
    {
        Positions::new(self.get_dim()).zip(self.iter_mut_all())
    }
}

impl<K> Row<K> {
    /// Iterator over shared references to elements in the row.
    #[inline(always)]
    pub fn iter(&self) -> slice::Iter<K> {
        self.into_iter()
    }

    /// Iterator over mutable references to elements in the row.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> slice::IterMut<K> {
        self.into_iter()
    }
}

impl<K> IntoIterator for Matrix<K> {
    type Item = Row<K>;
    type IntoIter = vec::IntoIter<Row<K>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a, K> IntoIterator for &'a Matrix<K> {
    type Item = &'a Row<K>;
    type IntoIter = slice::Iter<'a, Row<K>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

impl<'a, K> IntoIterator for &'a mut Matrix<K> {
    type Item = &'a mut Row<K>;
    type IntoIter = slice::IterMut<'a, Row<K>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter_mut()
    }
}

impl<K> IntoIterator for Row<K> {
    type Item = K;
    type IntoIter = vec::IntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl<'a, K> IntoIterator for &'a Row<K> {
    type Item = &'a K;
    type IntoIter = slice::Iter<'a, K>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter()
    }
}

impl<'a, K> IntoIterator for &'a mut Row<K> {
    type Item = &'a mut K;
    type IntoIter = slice::IterMut<'a, K>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter_mut()
    }
}

/// Iterator over positions in a matrix in row-major order.
pub struct Positions<T> {
    pos: Vec2<usize>,
    dim: Vec2<usize>,
    _phantom: marker::PhantomData<T>,
}

impl<T> Positions<T> {
    #[inline]
    fn new(dim: Vec2<usize>) -> Self {
        Self {
            pos: v(0, 0),
            dim,
            _phantom: marker::PhantomData,
        }
    }
}

impl<T> Iterator for Positions<T>
where
    usize: TryInto<T>,
{
    type Item = Vec2<T>;

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
