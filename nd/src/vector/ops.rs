use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::vector::{Vec2, Vec3, Vec4};

macro_rules! impl_term {
    ($tr:ident, $f:ident, $op:tt, $tr_a:ident, $f_a:ident, $op_a:tt) => {
        impl<'a, 'b, T> $tr<&'b Vec2<T>> for &'a Vec2<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Vec2<T>;

            fn $f(self, rhs: &'b Vec2<T>) -> Self::Output {
                Vec2::new(&self.x $op &rhs.x, &self.y $op &rhs.y)
            }
        }
        impl<'a, 'b, T> $tr<&'b Vec3<T>> for &'a Vec3<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Vec3<T>;

            fn $f(self, rhs: &'b Vec3<T>) -> Self::Output {
                Vec3::new(&self.x $op &rhs.x, &self.y $op &rhs.y, &self.z $op &rhs.z)
            }
        }
        impl<'a, 'b, T> $tr<&'b Vec4<T>> for &'a Vec4<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Vec4<T>;

            fn $f(self, rhs: &'b Vec4<T>) -> Self::Output {
                Vec4::new(&self.x $op &rhs.x, &self.y $op &rhs.y, &self.z $op &rhs.z, &self.w $op &rhs.w)
            }
        }

        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a Vec2<T>> for Vec2<T> {
            fn $f_a(&mut self, rhs: &'a Vec2<T>) {
                self.x $op_a &rhs.x;
                self.y $op_a &rhs.y;
            }
        }
        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a Vec3<T>> for Vec3<T> {
            fn $f_a(&mut self, rhs: &'a Vec3<T>) {
                self.x $op_a &rhs.x;
                self.y $op_a &rhs.y;
                self.z $op_a &rhs.z;
            }
        }
        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a Vec4<T>> for Vec4<T> {
            fn $f_a(&mut self, rhs: &'a Vec4<T>) {
                self.x $op_a &rhs.x;
                self.y $op_a &rhs.y;
                self.z $op_a &rhs.z;
                self.w $op_a &rhs.w;
            }
        }

        impl<T: $tr<Output = T>> $tr<Vec2<T>> for Vec2<T> {
            type Output = Vec2<T>;

            fn $f(self, rhs: Vec2<T>) -> Self::Output {
                Vec2::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }
        impl<T: $tr<Output = T>> $tr<Vec3<T>> for Vec3<T> {
            type Output = Vec3<T>;

            fn $f(self, rhs: Vec3<T>) -> Self::Output {
                Vec3::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z)
            }
        }
        impl<T: $tr<Output = T>> $tr<Vec4<T>> for Vec4<T> {
            type Output = Vec4<T>;

            fn $f(self, rhs: Vec4<T>) -> Self::Output {
                Vec4::new(self.x $op rhs.x, self.y $op rhs.y, self.z $op rhs.z, self.w $op rhs.w)
            }
        }

        impl<T: $tr_a> $tr_a<Vec2<T>> for Vec2<T> {
            fn $f_a(&mut self, rhs: Vec2<T>) {
                self.x $op_a rhs.x;
                self.y $op_a rhs.y;
            }
        }
        impl<T: $tr_a> $tr_a<Vec3<T>> for Vec3<T> {
            fn $f_a(&mut self, rhs: Vec3<T>) {
                self.x $op_a rhs.x;
                self.y $op_a rhs.y;
                self.z $op_a rhs.z;
            }
        }
        impl<T: $tr_a> $tr_a<Vec4<T>> for Vec4<T> {
            fn $f_a(&mut self, rhs: Vec4<T>) {
                self.x $op_a rhs.x;
                self.y $op_a rhs.y;
                self.z $op_a rhs.z;
                self.w $op_a rhs.w;
            }
        }
    };
}

impl_term!(Add, add, +, AddAssign, add_assign, +=);
impl_term!(Sub, sub, -, SubAssign, sub_assign, -=);

macro_rules! impl_factor {
    ($tr:ident, $f:ident, $op:tt, $tr_a:ident, $f_a:ident, $op_a:tt) => {
        impl<'a, 'b, T> $tr<&'b T> for &'a Vec2<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Vec2<T>;

            fn $f(self, rhs: &'b T) -> Self::Output {
                Vec2::new(&self.x $op rhs, &self.y $op rhs)
            }
        }
        impl<'a, 'b, T> $tr<&'b T> for &'a Vec3<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Vec3<T>;

            fn $f(self, rhs: &'b T) -> Self::Output {
                Vec3::new(&self.x $op rhs, &self.y $op rhs, &self.z $op rhs)
            }
        }
        impl<'a, 'b, T> $tr<&'b T> for &'a Vec4<T>
        where
            &'a T: $tr<&'b T, Output = T>,
        {
            type Output = Vec4<T>;

            fn $f(self, rhs: &'b T) -> Self::Output {
                Vec4::new(&self.x $op rhs, &self.y $op rhs, &self.z $op rhs, &self.w $op rhs)
            }
        }

        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a T> for Vec2<T> {
            fn $f_a(&mut self, rhs: &'a T) {
                self.x $op_a rhs;
                self.y $op_a rhs;
            }
        }
        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a T> for Vec3<T> {
            fn $f_a(&mut self, rhs: &'a T) {
                self.x $op_a rhs;
                self.y $op_a rhs;
                self.z $op_a rhs;
            }
        }
        impl<'a, T: $tr_a<&'a T>> $tr_a<&'a T> for Vec4<T> {
            fn $f_a(&mut self, rhs: &'a T) {
                self.x $op_a rhs;
                self.y $op_a rhs;
                self.z $op_a rhs;
                self.w $op_a rhs;
            }
        }

        impl<T: Copy + $tr<Output = T>> $tr<T> for Vec2<T> {
            type Output = Vec2<T>;

            fn $f(self, rhs: T) -> Self::Output {
                Vec2::new(self.x $op rhs, self.y $op rhs)
            }
        }
        impl<T: Copy + $tr<Output = T>> $tr<T> for Vec3<T> {
            type Output = Vec3<T>;

            fn $f(self, rhs: T) -> Self::Output {
                Vec3::new(self.x $op rhs, self.y $op rhs, self.z $op rhs)
            }
        }
        impl<T: Copy + $tr<Output = T>> $tr<T> for Vec4<T> {
            type Output = Vec4<T>;

            fn $f(self, rhs: T) -> Self::Output {
                Vec4::new(self.x $op rhs, self.y $op rhs, self.z $op rhs, self.w $op rhs)
            }
        }

        impl<T: Copy + $tr_a> $tr_a<T> for Vec2<T> {
            fn $f_a(&mut self, rhs: T) {
                self.x $op_a rhs;
                self.y $op_a rhs;
            }
        }
        impl<T: Copy + $tr_a> $tr_a<T> for Vec3<T> {
            fn $f_a(&mut self, rhs: T) {
                self.x $op_a rhs;
                self.y $op_a rhs;
                self.z $op_a rhs;
            }
        }
        impl<T: Copy + $tr_a> $tr_a<T> for Vec4<T> {
            fn $f_a(&mut self, rhs: T) {
                self.x $op_a rhs;
                self.y $op_a rhs;
                self.z $op_a rhs;
                self.w $op_a rhs;
            }
        }
    };
}

impl_factor!(Mul, mul, *, MulAssign, mul_assign, *=);
impl_factor!(Div, div, /, DivAssign, div_assign, /=);
impl_factor!(Rem, rem, %, RemAssign, rem_assign, %=);

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl<'a, T> Neg for &'a Vec2<T>
where
    &'a T: Neg<Output = T>,
{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::new(-&self.x, -&self.y)
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<'a, T> Neg for &'a Vec3<T>
where
    &'a T: Neg<Output = T>,
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-&self.x, -&self.y, -&self.z)
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Vec4<T>;

    fn neg(self) -> Self::Output {
        Vec4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<'a, T> Neg for &'a Vec4<T>
where
    &'a T: Neg<Output = T>,
{
    type Output = Vec4<T>;

    fn neg(self) -> Self::Output {
        Vec4::new(-&self.x, -&self.y, -&self.z, -&self.w)
    }
}
