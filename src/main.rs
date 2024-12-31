use rijndael::key::AES128Key;
use rijndael::schedule::KeySchedule128;

fn main() {
    let key = [
        0xa8, 0x19, 0x02, 0x76, 0x7e, 0x25, 0xdb, 0x17, 0x0f, 0x34, 0x49, 0xc5, 0xd9, 0x4b, 0x16,
        0x2f,
    ];
    //let key = hex!("7604543B46F4165E1865EA0EE96422D0");
    //let key = hex!("49 20 e2 99 a5 20 52 61 64 69 6f 47 61 74 75 6e");
    let key = AES128Key::from(key);

    let keys = KeySchedule128::expand(key);

    for key in &keys.to_dec() {
        println!("{key:X}");
    }
}
