// use [`Float`] for rotations with sin, cos
use core::ops::Neg;

use num_traits::{real::Real, Num, Zero};

use crate::{
    vector::{v, Vec2, Vec3, Vec4},
    Matrix,
};

impl<T: Zero> Vec2<T> {
    pub fn extend_3d(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, T::zero())
    }

    pub fn extend_4d(self) -> Vec4<T> {
        Vec4::new(self.x, self.y, T::zero(), T::zero())
    }
}

impl<T: Num> Vec2<T> {
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn perp_dot(self, other: Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl<T: Clone + Num> Vec2<T> {
    pub fn len_sq(self) -> T {
        self.clone().dot(self)
    }
}

impl<T: Neg<Output = T>> Vec2<T> {
    pub fn perp(self) -> Self {
        Self::new(-self.y, self.x)
    }
}

impl<T: Real> Vec2<T> {
    pub fn len(self) -> T {
        self.len_sq().sqrt()
    }

    pub fn normalise(self) -> Self {
        let len_sq = self.len_sq();
        if len_sq.is_zero() {
            return self;
        }
        self / len_sq.sqrt()
    }

    pub fn lerp(self, other: Self, t: T) -> Self {
        self * (T::one() - t) + other * t
    }

    pub fn rotate(self, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.x * cos - self.y * sin, self.y * cos + self.x * sin)
    }
}

impl<T> Vec3<T> {
    pub fn truncate_2d(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }
}

impl<T: Zero> Vec3<T> {
    pub fn extend_4d(self) -> Vec4<T> {
        Vec4::new(self.x, self.y, self.z, T::zero())
    }
}

impl<T: Num> Vec3<T> {
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Clone + Num> Vec3<T> {
    pub fn len_sq(self) -> T {
        self.clone().dot(self)
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
            self.z * other.x.clone() - self.x.clone() * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T: Real> Vec3<T> {
    pub fn len(self) -> T {
        self.len_sq().sqrt()
    }

    pub fn normalise(self) -> Self {
        let len_sq = self.len_sq();
        if len_sq.is_zero() {
            return self;
        }
        self / len_sq.sqrt()
    }

    pub fn lerp(self, other: Self, t: T) -> Self {
        self * (T::one() - t) + other * t
    }

    // TODO: rotations about each axis
}

impl<T> Vec4<T> {
    pub fn truncate_2d(self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub fn truncate_3d(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<T: Num> Vec4<T> {
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl<T: Clone + Num> Vec4<T> {
    pub fn len_sq(self) -> T {
        self.clone().dot(self)
    }
}

impl<T: Real> Vec4<T> {
    pub fn len(self) -> T {
        self.len_sq().sqrt()
    }

    pub fn normalise(self) -> Self {
        let len_sq = self.len_sq();
        if len_sq.is_zero() {
            return self;
        }
        self / len_sq.sqrt()
    }

    pub fn lerp(self, other: Self, t: T) -> Self {
        self * (T::one() - t) + other * t
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
