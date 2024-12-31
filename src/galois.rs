#[cfg(not(feature = "std"))]
use core::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "std")]
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Galois(u8);

impl From<u8> for Galois {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl PartialEq for Galois {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<u8> for Galois {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl Add for Galois {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.bitxor(rhs.0))
    }
}

impl AddAssign for Galois {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Galois {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl SubAssign for Galois {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Galois {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut a = self.0;
        let mut b = rhs.0;
        let mut p = 0;

        for _ in 0..u8::BITS {
            if b & 0b00000001 != 0 {
                p ^= a;
            }

            let a_set = a & 0b10000000 != 0;

            a <<= 1;

            if a_set {
                a ^= 0b00011011;
            }

            b >>= 1;
        }

        Self(p)
    }
}

impl MulAssign for Galois {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Galois {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        for b in 0..=u8::MAX {
            let gal = Self::from(b);

            if gal * rhs == self {
                return Some(gal);
            }
        }
        None
    }
}

impl DivAssign for Galois {
    fn div_assign(&mut self, rhs: Self) {
        *self = (*self / rhs).expect("quotient should not be None")
    }
}

impl Galois {
    pub fn pow(self, rhs: u8) -> Self {
        let mut p = 1.into();

        for _ in 0..rhs {
            p *= self;
        }

        p
    }

    pub fn inv(self) -> Self {
        match self {
            Self(0) => Self(0),
            c => (Self(1) / c).unwrap(),
        }
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }

    pub fn mix_column(a: [Galois; 4]) -> [Galois; 4] {
        const GAL_2: Galois = Galois(2);
        const GAL_3: Galois = Galois(3);

        [
            GAL_2 * a[0] + a[3] + a[2] + GAL_3 * a[1],
            GAL_2 * a[1] + a[0] + a[3] + GAL_3 * a[2],
            GAL_2 * a[2] + a[1] + a[0] + GAL_3 * a[3],
            GAL_2 * a[3] + a[2] + a[1] + GAL_3 * a[0],
        ]
    }

    pub fn inv_mix_column(d: [Galois; 4]) -> [Galois; 4] {
        const GAL_9: Galois = Galois(9);
        const GAL_11: Galois = Galois(11);
        const GAL_13: Galois = Galois(13);
        const GAL_14: Galois = Galois(14);

        [
            GAL_14 * d[0] + GAL_9 * d[3] + GAL_13 * d[2] + GAL_11 * d[1],
            GAL_14 * d[1] + GAL_9 * d[0] + GAL_13 * d[3] + GAL_11 * d[2],
            GAL_14 * d[2] + GAL_9 * d[1] + GAL_13 * d[0] + GAL_11 * d[3],
            GAL_14 * d[3] + GAL_9 * d[2] + GAL_13 * d[1] + GAL_11 * d[0],
        ]
    }
}
