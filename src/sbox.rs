use crate::galois::Galois;

#[derive(Debug)]
pub struct Sbox;

impl Sbox {
    #[allow(non_snake_case)]
    pub fn S(c: u8) -> u8 {
        let b = Galois::from(c).inv().into_inner();
        b ^ b.rotate_left(1) ^ b.rotate_left(2) ^ b.rotate_left(3) ^ b.rotate_left(4) ^ 99
    }
}
