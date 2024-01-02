// TODO: add docs

use num_traits::{Signed, Zero};

use crate::vector::{v, Vec2};

impl<T: Zero> Vec2<T> {
    #[inline(always)]
    pub fn zero() -> Self {
        v(T::zero(), T::zero())
    }
}

impl<T: Signed> Vec2<T> {
    #[inline(always)]
    pub fn e() -> Self {
        v(T::one(), T::zero())
    }

    #[inline(always)]
    pub fn n() -> Self {
        v(T::zero(), -T::one())
    }

    #[inline(always)]
    pub fn w() -> Self {
        v(-T::one(), T::zero())
    }

    #[inline(always)]
    pub fn s() -> Self {
        v(T::zero(), T::one())
    }

    #[inline(always)]
    pub fn ne() -> Self {
        v(T::one(), -T::one())
    }

    #[inline(always)]
    pub fn nw() -> Self {
        v(-T::one(), -T::one())
    }

    #[inline(always)]
    pub fn sw() -> Self {
        v(-T::one(), T::one())
    }

    #[inline(always)]
    pub fn se() -> Self {
        v(T::one(), T::one())
    }

    #[inline(always)]
    pub fn orth() -> [Self; 4] {
        [Self::e(), Self::n(), Self::w(), Self::s()]
    }

    #[inline(always)]
    pub fn diag() -> [Self; 4] {
        [Self::ne(), Self::nw(), Self::sw(), Self::se()]
    }

    #[inline(always)]
    pub fn adj() -> [Self; 8] {
        [
            Self::e(),
            Self::ne(),
            Self::n(),
            Self::nw(),
            Self::w(),
            Self::sw(),
            Self::s(),
            Self::se(),
        ]
    }

    #[inline(always)]
    pub fn orth_z() -> [Self; 5] {
        [Self::zero(), Self::e(), Self::n(), Self::w(), Self::s()]
    }

    #[inline(always)]
    pub fn diag_z() -> [Self; 5] {
        [Self::zero(), Self::ne(), Self::nw(), Self::sw(), Self::se()]
    }

    #[inline(always)]
    pub fn adj_z() -> [Self; 9] {
        [
            Self::zero(),
            Self::e(),
            Self::ne(),
            Self::n(),
            Self::nw(),
            Self::w(),
            Self::sw(),
            Self::s(),
            Self::se(),
        ]
    }
}
