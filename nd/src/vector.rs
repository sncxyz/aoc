mod constants;
mod linalg;
mod ops;

use std::{borrow::Borrow, fmt};

use num_traits::{
    bounds::{LowerBounded, UpperBounded},
    Signed,
};

use crate::traits::{Field, FieldOps};

/// A 2D vector type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// A convencience function for creating instances of `Vec2<T>`.
#[inline(always)]
pub const fn v<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}

impl<T> Vec2<T> {
    /// Creates a new vector with the given `x` and `y` values.
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: LowerBounded> Vec2<T> {
    /// Returns a vector with the minimum value of `x` and `y`.
    #[inline]
    pub fn min_value() -> Self {
        Self::new(T::min_value(), T::min_value())
    }
}

impl<T: UpperBounded> Vec2<T> {
    /// Returns a vector with the maximum value of `x` and `y`.
    #[inline]
    pub fn max_value() -> Self {
        Self::new(T::max_value(), T::max_value())
    }
}

impl<T: Signed> Vec2<T> {
    /// Computes the absolute value component-wise.
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    /// Computes the signum component-wise.
    #[inline]
    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }
}

impl<T: Signed + Field> Vec2<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: impl Borrow<Self>) -> T {
        let diff = self - other.borrow();
        diff.x.abs() + diff.y.abs()
    }
}

impl<T: Ord> Vec2<T> {
    /// Computes the minimum of `self` and `other` component-wise.
    pub fn min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Computes the minimum of `self` and `other` component-wise.
    pub fn max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Clamps `self` between `min` and `max` component-wise.
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
}

impl<T: TryInto<usize>> Vec2<T> {
    pub(crate) fn try_into_usize(self) -> Option<Vec2<usize>> {
        self.x
            .try_into()
            .ok()
            .and_then(|x| self.y.try_into().ok().map(|y| Vec2::new(x, y)))
    }
}

impl Vec2<usize> {
    pub(crate) fn try_from_usize<T>(self) -> Option<Vec2<T>>
    where
        usize: TryInto<T>,
    {
        self.x
            .try_into()
            .ok()
            .and_then(|x| self.y.try_into().ok().map(|y| Vec2::new(x, y)))
    }
}

/// A 3D vector type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// A convencience function for creating instances of `Vec3<T>`.
#[inline(always)]
pub const fn v3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}

impl<T> Vec3<T> {
    /// Creates a new vector with the given `x`, `y` and `z` values.
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: LowerBounded> Vec3<T> {
    /// Returns a vector with the minimum value of `x`, `y` and `z`.
    pub fn min_value() -> Self {
        Self::new(T::min_value(), T::min_value(), T::min_value())
    }
}

impl<T: UpperBounded> Vec3<T> {
    /// Returns a vector with the maximum value of `x`, `y` and `z`.
    pub fn max_value() -> Self {
        Self::new(T::max_value(), T::max_value(), T::max_value())
    }
}

impl<T: Signed> Vec3<T> {
    /// Computes the absolute value component-wise.
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    /// Computes the signum component-wise.
    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.y.signum())
    }
}

impl<T: Signed + Field> Vec3<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: impl Borrow<Self>) -> T {
        let diff = self - other.borrow();
        diff.x.abs() + diff.y.abs() + diff.z.abs()
    }
}

impl<T: Ord> Vec3<T> {
    /// Computes the minimum of `self` and `other` component-wise.
    pub fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Computes the minimum of `self` and `other` component-wise.
    pub fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Clamps `self` between `min` and `max` component-wise.
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }
}

/// A 4D vector type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

/// A convencience function for creating instances of `Vec4<T>`.
#[inline(always)]
pub const fn v4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}

impl<T> Vec4<T> {
    /// Creates a new vector with the given `x`, `y`, `z` and `w` values.
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: LowerBounded> Vec4<T> {
    /// Returns a vector with the minimum value of `x`, `y`, `z` and `w`.
    pub fn min_value() -> Self {
        Self::new(
            T::min_value(),
            T::min_value(),
            T::min_value(),
            T::min_value(),
        )
    }
}

impl<T: UpperBounded> Vec4<T> {
    /// Returns a vector with the maximum value of `x`, `y`, `z` and `w`.
    #[inline]
    pub fn max_value() -> Self {
        Self::new(
            T::max_value(),
            T::max_value(),
            T::max_value(),
            T::max_value(),
        )
    }
}

impl<T: Signed> Vec4<T> {
    /// Computes the absolute value component-wise.
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    /// Computes the signum component-wise.
    pub fn signum(&self) -> Self {
        Self::new(
            self.x.signum(),
            self.y.signum(),
            self.y.signum(),
            self.w.signum(),
        )
    }
}

impl<T: Signed + Field> Vec4<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the manhattan distance between `self` and `other`.
    pub fn manhattan(&self, other: impl Borrow<Self>) -> T {
        let diff = self - other.borrow();
        diff.x.abs() + diff.y.abs() + diff.z.abs() + diff.w.abs()
    }
}

impl<T: Ord> Vec4<T> {
    /// Computes the minimum of `self` and `other` component-wise.
    pub fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    /// Computes the minimum of `self` and `other` component-wise.
    pub fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }

    /// Clamps `self` between `min` and `max` component-wise.
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
            self.w.clamp(min.w, max.w),
        )
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: fmt::Display> fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
