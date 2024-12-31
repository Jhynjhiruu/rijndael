use crate::{galois::Galois, key::AESKey};

#[derive(Debug, Clone, Copy)]
pub struct RoundKeys<const KEY_SIZE: usize, const ROUNDS: usize>([AESKey<KEY_SIZE>; ROUNDS]);

impl<const N: usize, const R: usize> From<[AESKey<N>; R]> for RoundKeys<N, R> {
    fn from(value: [AESKey<N>; R]) -> Self {
        Self(value)
    }
}

pub struct Iter<const N: usize, const R: usize> {
    rounds: RoundKeys<N, R>,
    index: usize,
}

impl<const N: usize, const R: usize> Iterator for Iter<N, R> {
    type Item = AESKey<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < R {
            let item = self.rounds.0[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<const N: usize, const R: usize> IntoIterator for RoundKeys<N, R> {
    type Item = AESKey<N>;

    type IntoIter = Iter<N, R>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            rounds: self,
            index: 0,
        }
    }
}

pub struct IterRef<'a, const N: usize, const R: usize> {
    rounds: &'a RoundKeys<N, R>,
    index: usize,
}

impl<'a, const N: usize, const R: usize> Iterator for IterRef<'a, N, R> {
    type Item = &'a AESKey<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < R {
            let item = &self.rounds.0[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, const N: usize, const R: usize> IntoIterator for &'a RoundKeys<N, R> {
    type Item = &'a AESKey<N>;

    type IntoIter = IterRef<'a, N, R>;

    fn into_iter(self) -> Self::IntoIter {
        IterRef {
            rounds: self,
            index: 0,
        }
    }
}

impl<const N: usize, const R: usize> RoundKeys<N, R> {
    pub fn iter(&self) -> IterRef<'_, N, R> {
        self.into_iter()
    }

    pub fn to_dec(mut self) -> Self {
        self.0.reverse();

        for key in &mut self.0[1..R - 1] {
            for word in key.0.iter_mut() {
                let column = word.to_ne_bytes().map(Galois::from);
                let mixed = Galois::inv_mix_column(column).map(Galois::into_inner);
                *word = u32::from_ne_bytes(mixed);
            }
        }

        self
    }
}
