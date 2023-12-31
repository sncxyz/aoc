use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use num_traits::{One, Zero};

/// Trait for types supporting arithmetic operations returning `Output`.
pub trait FieldOps<Output = Self>:
    Sized
    + Add<Output = Output>
    + Sub<Output = Output>
    + Mul<Output = Output>
    + Div<Output = Output>
    + Rem<Output = Output>
    + Neg<Output = Output>
{
}

impl<T, Output> FieldOps<Output> for T where
    T: Add<Output = Output>
        + Sub<Output = Output>
        + Mul<Output = Output>
        + Div<Output = Output>
        + Rem<Output = Output>
        + Neg<Output = Output>
{
}

/// Trait for types supporting arithmetic assignment operations with a right-hand side of `Self` or `&Self`.
pub trait FieldAssign: Sized + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
where
    for<'a> Self: AddAssign<&'a Self>
        + SubAssign<&'a Self>
        + MulAssign<&'a Self>
        + DivAssign<&'a Self>
        + RemAssign<&'a Self>,
{
}

impl<T> FieldAssign for T
where
    T: AddAssign + SubAssign + MulAssign + DivAssign + RemAssign,
    for<'a> T: AddAssign<&'a T>
        + SubAssign<&'a T>
        + MulAssign<&'a T>
        + DivAssign<&'a T>
        + RemAssign<&'a T>,
{
}

/// Trait for field types.
pub trait Field: Clone + Zero + One + FieldOps + FieldAssign {}

impl<T> Field for T where T: Clone + Zero + One + FieldOps + FieldAssign {}

/// Trait for types that can be used to index a collection via conversion to and from `usize`.
pub trait Pos: fmt::Debug + Clone + TryInto<usize> + TryFrom<usize> {
    fn get_index(&self) -> Option<usize> {
        self.clone().try_into().ok()
    }

    fn get_pos(index: usize) -> Option<Self> {
        index.try_into().ok()
    }

    #[track_caller]
    fn index(&self, from: &str) -> usize {
        if let Some(index) = self.get_index() {
            index
        } else {
            panic!("could not convert {from} to usize: {self:?}");
        }
    }

    #[track_caller]
    fn pos(index: usize, from: &str, to: &str) -> Self {
        if let Some(pos) = Self::get_pos(index) {
            pos
        } else {
            panic!("could not convert {from} to {to}: {index}");
        }
    }
}

impl<I> Pos for I where I: fmt::Debug + Clone + TryInto<usize> + TryFrom<usize> {}
