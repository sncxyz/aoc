mod constants;
mod linalg;
mod ops;

use core::{
    fmt,
    ops::{Add, Sub},
};

use num_traits::{
    bounds::{LowerBounded, UpperBounded},
    Signed,
};

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
    #[inline]
    pub fn min_value() -> Self {
        Self::new(T::min_value(), T::min_value())
    }
}

impl<T: UpperBounded> Vec2<T> {
    #[inline]
    pub fn max_value() -> Self {
        Self::new(T::max_value(), T::max_value())
    }
}

impl<T: Signed> Vec2<T> {
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }
}

impl<T: Signed + Add<Output = T> + Sub<Output = T>> Vec2<T> {
    pub fn manhattan(self, other: Self) -> T {
        let diff = self - other;
        diff.x.abs() + diff.y.abs()
    }
}

impl<T: Ord> Vec2<T> {
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    #[inline]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// A convencience function for creating instances of `Vec2<T>`.
#[inline(always)]
pub const fn v3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}

impl<T> Vec3<T> {
    /// Creates a new vector with the given `x` and `y` values.
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: LowerBounded> Vec3<T> {
    #[inline]
    pub fn min_value() -> Self {
        Self::new(T::min_value(), T::min_value(), T::min_value())
    }
}

impl<T: UpperBounded> Vec3<T> {
    #[inline]
    pub fn max_value() -> Self {
        Self::new(T::max_value(), T::max_value(), T::max_value())
    }
}

impl<T: Signed> Vec3<T> {
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    pub fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum(), self.y.signum())
    }
}

impl<T: Signed + Add<Output = T> + Sub<Output = T>> Vec3<T> {
    pub fn manhattan(self, other: Self) -> T {
        let diff = self - other;
        diff.x.abs() + diff.y.abs() + diff.z.abs()
    }
}

impl<T: Ord> Vec3<T> {
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

/// A convencience function for creating instances of `Vec2<T>`.
#[inline(always)]
pub const fn v4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}

impl<T> Vec4<T> {
    /// Creates a new vector with the given `x` and `y` values.
    #[inline(always)]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: LowerBounded> Vec4<T> {
    #[inline]
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
    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    #[inline]
    pub fn signum(&self) -> Self {
        Self::new(
            self.x.signum(),
            self.y.signum(),
            self.y.signum(),
            self.w.signum(),
        )
    }
}

impl<T: Signed + Add<Output = T> + Sub<Output = T>> Vec4<T> {
    pub fn manhattan(self, other: Self) -> T {
        let diff = self - other;
        diff.x.abs() + diff.y.abs() + diff.z.abs() + diff.w.abs()
    }
}

impl<T: Ord> Vec4<T> {
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }

    #[inline]
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
