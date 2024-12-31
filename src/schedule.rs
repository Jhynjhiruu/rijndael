use crate::{galois::Galois, key::AESKey, rounds::RoundKeys, sbox::Sbox};

#[derive(Debug)]
pub struct KeySchedule<const KEY_SIZE: usize, const ROUNDS: usize>;

impl<const N: usize, const R: usize> KeySchedule<N, R> {
    pub fn expand(key: AESKey<N>) -> RoundKeys<N, R> {
        let mut rounds = [AESKey::default(); R];

        #[allow(non_snake_case)]
        fn RotWord(word: u32) -> u32 {
            let bytes = word.to_ne_bytes();
            let bytes = [bytes[1], bytes[2], bytes[3], bytes[0]];
            u32::from_ne_bytes(bytes)
        }

        #[allow(non_snake_case)]
        fn SubWord(word: u32) -> u32 {
            let mut bytes = word.to_ne_bytes();
            for byte in &mut bytes {
                *byte = Sbox::S(*byte);
            }
            u32::from_ne_bytes(bytes)
        }

        fn rcon(round: usize) -> u32 {
            u32::from_ne_bytes([Galois::from(2).pow((round - 1) as u8).into_inner(), 0, 0, 0])
        }

        for i in 0..4 * R {
            let r = i / N;
            let w = i % N;

            rounds[r][w] = if r == 0 {
                key[w]
            } else if w == 0 {
                rounds[r - 1][0] ^ SubWord(RotWord(rounds[r - 1][N - 1])) ^ rcon(r)
            } else if N > 6 && w == 4 {
                rounds[r - 1][4] ^ SubWord(rounds[r][3])
            } else {
                rounds[r - 1][w] ^ rounds[r][w - 1]
            };
        }

        rounds.into()
    }
}

pub type KeySchedule128 = KeySchedule<4, 11>;
pub type KeySchedule192 = KeySchedule<6, 13>;
pub type KeySchedule256 = KeySchedule<8, 15>;
