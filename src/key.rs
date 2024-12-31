#[cfg(not(feature = "std"))]
use core::fmt;
use core::mem::size_of;
#[cfg(not(feature = "std"))]
use core::ops::{Index, IndexMut};
#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::format;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct AESKey<const KEY_SIZE: usize>(pub(crate) [u32; KEY_SIZE]);

impl<const N: usize> Index<usize> for AESKey<N> {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<const N: usize> IndexMut<usize> for AESKey<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize> Default for AESKey<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<const N: usize> fmt::UpperHex for AESKey<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .map(|w| w.to_ne_bytes().map(|b| format!("{b:02X}")).join(""))
                .join("")
        )
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<const N: usize> fmt::LowerHex for AESKey<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .map(|w| w.to_ne_bytes().map(|b| format!("{b:02X}")).join(""))
                .join("")
        )
    }
}

impl<const N: usize> TryFrom<&[u8]> for AESKey<N> {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != N * 4 {
            return Err(());
        }
        Ok(Self(core::array::from_fn(|i| {
            u32::from_ne_bytes(
                value[i * size_of::<u32>()..(i + 1) * size_of::<u32>()]
                    .try_into()
                    .unwrap(),
            )
        })))
    }
}

pub type AES128Key = AESKey<4>;
pub type AES192Key = AESKey<6>;
pub type AES256Key = AESKey<8>;

impl From<[u8; 16]> for AES128Key {
    fn from(value: [u8; 16]) -> Self {
        value.as_slice().try_into().unwrap()
    }
}

impl From<[u8; 24]> for AES192Key {
    fn from(value: [u8; 24]) -> Self {
        value.as_slice().try_into().unwrap()
    }
}

impl From<[u8; 32]> for AES256Key {
    fn from(value: [u8; 32]) -> Self {
        value.as_slice().try_into().unwrap()
    }
}
