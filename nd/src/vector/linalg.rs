use std::borrow::Borrow;

use num_traits::{real::Real, Zero};

use crate::{
    traits::{Field, FieldOps},
    vector::{v, Vec2, Vec3, Vec4},
    Matrix,
};

impl<T: Clone + Zero> Vec2<T> {
    /// Returns a new `Vec3<T>` with an `x` and `y` value equal to that of `self`, and a `z` value equal to `0`.
    pub fn extend_3d(&self) -> Vec3<T> {
        Vec3::new(self.x.clone(), self.y.clone(), T::zero())
    }

    /// Returns a new `Vec4<T>` with an `x` and `y` value equal to that of `self`, and `z` and `w` values equal to `0`.
    pub fn extend_4d(&self) -> Vec4<T> {
        Vec4::new(self.x.clone(), self.y.clone(), T::zero(), T::zero())
    }
}

impl<T: Field> Vec2<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the dot product of `self` and `other`.
    pub fn dot(&self, other: impl Borrow<Self>) -> T {
        let other = other.borrow();
        &self.x * &other.x + &self.y * &other.y
    }

    /// Returns the square of the magnitude of `self`.
    pub fn len_sq(&self) -> T {
        self.dot(self)
    }

    /// Returns the perpendicular dot product of `self` and `other`.
    ///
    /// Equal to `self.perp().dot(other)`.
    pub fn perp_dot(&self, other: impl Borrow<Self>) -> T {
        let other = other.borrow();
        &self.x * &other.y - &self.y * &other.x
    }

    /// Returns `self` rotated anti-clockwise by a quarter turn.
    pub fn perp(&self) -> Self {
        Self::new(-&self.y, self.x.clone())
    }
}

impl<T: Field + Real> Vec2<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the magnitude of `self`.
    pub fn len(&self) -> T {
        self.len_sq().sqrt()
    }

    /// Returns the unit vector with the same direction as `self`,
    /// or the zero vector if `self` has magnitude `0`.
    pub fn normalise(&self) -> Self {
        let len_sq = self.len_sq();
        if !len_sq.is_zero() {
            return self.clone();
        }
        self / &len_sq.sqrt()
    }

    /// Linearly interpolates between `self` and `other` with a weight of `t`.
    pub fn lerp(&self, other: impl Borrow<Self>, t: impl Borrow<T>) -> Self {
        let t = t.borrow();
        self * &(&T::one() - t) + other.borrow() * t
    }

    /// Returns `self` rotated anti-clockwise by `angle` radians.
    pub fn rotate(&self, angle: impl Borrow<T>) -> Self {
        let (sin, cos) = angle.borrow().sin_cos();
        Self::new(
            &self.x * &cos - &self.y * &sin,
            &self.y * &cos + &self.x * &sin,
        )
    }
}

impl<T: Clone> Vec3<T> {
    /// Returns a new `Vec2<T>` with an `x` and `y` value equal to that of `self`.
    pub fn truncate_2d(&self) -> Vec2<T> {
        Vec2::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Clone + Zero> Vec3<T> {
    /// Returns a new `Vec4<T>` with an `x`, `y` and `z` value equal to that of `self`, and a `w` value equal to `0`.
    pub fn extend_4d(&self) -> Vec4<T> {
        Vec4::new(self.x.clone(), self.y.clone(), self.z.clone(), T::zero())
    }
}

impl<T: Field> Vec3<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the dot product of `self` and `other`.
    pub fn dot(&self, other: impl Borrow<Self>) -> T {
        let other = other.borrow();
        &self.x * &other.x + &self.y * &other.y + &self.z * &other.z
    }

    /// Returns the square of the magnitude of `self`.
    pub fn len_sq(&self) -> T {
        self.dot(self)
    }

    /// Returns the cross product of `self` and `other`.
    pub fn cross(&self, other: impl Borrow<Self>) -> Self {
        let other = other.borrow();
        Self::new(
            &self.y * &other.z - &self.z * &other.y,
            &self.z * &other.x - &self.x * &other.z,
            &self.x * &other.y - &self.y * &other.x,
        )
    }
}

impl<T: Field + Real> Vec3<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the magnitude of `self`.
    pub fn len(&self) -> T {
        self.len_sq().sqrt()
    }

    /// Returns the unit vector with the same direction as `self`,
    /// or the zero vector if `self` has magnitude `0`.
    pub fn normalise(&self) -> Self {
        let len_sq = self.len_sq();
        if len_sq.is_zero() {
            return self.clone();
        }
        self / &len_sq.sqrt()
    }

    /// Linearly interpolates between `self` and `other` with a weight of `t`.
    pub fn lerp(&self, other: impl Borrow<Self>, t: impl Borrow<T>) -> Self {
        let t = t.borrow();
        self * &(&T::one() - t) + other.borrow() * t
    }

    // TODO: rotations about each axis
}

impl<T: Clone> Vec4<T> {
    /// Returns a new `Vec2<T>` with an `x` and `y` value equal to that of `self`.
    pub fn truncate_2d(&self) -> Vec2<T> {
        Vec2::new(self.x.clone(), self.y.clone())
    }

    /// Returns a new `Vec3<T>` with an `x`, `y` and `z` value equal to that of `self`.
    pub fn truncate_3d(&self) -> Vec3<T> {
        Vec3::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Field> Vec4<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the dot product of `self` and `other`.
    pub fn dot(&self, other: impl Borrow<Self>) -> T {
        let other = other.borrow();
        &self.x * &other.x + &self.y * &other.y + &self.z * &other.z + &self.w * &other.w
    }

    /// Returns the square of the magnitude of `self`.
    pub fn len_sq(&self) -> T {
        self.dot(self)
    }
}

impl<T: Field + Real> Vec4<T>
where
    for<'a> &'a T: FieldOps<T>,
{
    /// Returns the magnitude of `self`.
    pub fn len(&self) -> T {
        self.len_sq().sqrt()
    }

    /// Returns the unit vector with the same direction as `self`,
    /// or the zero vector if `self` has magnitude `0`.
    pub fn normalise(&self) -> Self {
        let len_sq = self.len_sq();
        if len_sq.is_zero() {
            return self.clone();
        }
        self / &len_sq.sqrt()
    }

    /// Linearly interpolates between `self` and `other` with a weight of `t`.
    pub fn lerp(&self, other: impl Borrow<Self>, t: impl Borrow<T>) -> Self {
        let t = t.borrow();
        self * &(&T::one() - t) + other.borrow() * t
    }
}

impl<T> From<Vec2<T>> for Matrix<T> {
    fn from(value: Vec2<T>) -> Self {
        Self::col([value.x, value.y])
    }
}

impl<T> TryFrom<Matrix<T>> for Vec2<T> {
    type Error = ();

    fn try_from(value: Matrix<T>) -> Result<Self, Self::Error> {
        let dim = value.get_dim();
        if dim == v(1, 2) || dim == v(2, 1) {
            let mut elems = value.into_iter_all();
            Ok(Self::new(elems.next().unwrap(), elems.next().unwrap()))
        } else {
            Err(())
        }
    }
}

impl<T> From<Vec3<T>> for Matrix<T> {
    fn from(value: Vec3<T>) -> Self {
        Self::col([value.x, value.y, value.z])
    }
}

impl<T> TryFrom<Matrix<T>> for Vec3<T> {
    type Error = ();

    fn try_from(value: Matrix<T>) -> Result<Self, Self::Error> {
        let dim = value.get_dim();
        if dim == v(1, 3) || dim == v(3, 1) {
            let mut elems = value.into_iter_all();
            Ok(Self::new(
                elems.next().unwrap(),
                elems.next().unwrap(),
                elems.next().unwrap(),
            ))
        } else {
            Err(())
        }
    }
}

impl<T> From<Vec4<T>> for Matrix<T> {
    fn from(value: Vec4<T>) -> Self {
        Self::col([value.x, value.y, value.z, value.w])
    }
}

impl<T> TryFrom<Matrix<T>> for Vec4<T> {
    type Error = ();

    fn try_from(value: Matrix<T>) -> Result<Self, Self::Error> {
        let dim = value.get_dim();
        if dim == v(1, 4) || dim == v(4, 1) {
            let mut elems = value.into_iter_all();
            Ok(Self::new(
                elems.next().unwrap(),
                elems.next().unwrap(),
                elems.next().unwrap(),
                elems.next().unwrap(),
            ))
        } else {
            Err(())
        }
    }
}
