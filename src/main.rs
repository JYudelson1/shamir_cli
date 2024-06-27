mod utils;

use shamir::{decode, FieldElement, Hex, GF256};
use utils::decode_message;

fn main() {
    // let data = "This";
    // let shares = utils::encode_message::<GF256>(data, 5, 3);
    // let message = decode_message::<GF256>(shares).unwrap();
    // println!("Original: {data}\nDecoding: {message}");
    // let elements = GF256::from_bytes(&data.as_bytes());
    // let shares = shamir::encode(elements.clone(), 6, 4);
    // let decoded_raw = decode(vec![shares[0].clone(), shares[2].clone(), shares[1].clone(), shares[2].clone()]).unwrap();
    // println!("Original: {data}\nElements:     {elements:?}\nRaw decoding: {decoded_raw:?}");
    // let decoded = String::from_utf8(GF256::to_bytes(&decoded_raw)).unwrap();
    // println!("Decoding: {decoded}");
    let mut inv = [0_u8; 256];

    for i in 0..256 {
        inv[i] = GF256(i as u8).inverse().0;
    }
    println!("{inv:?}");
}
