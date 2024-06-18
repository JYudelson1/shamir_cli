mod utils;

use shamir::{decode, FieldElement, Hex, GF256};

fn main() {
    let data = "This";
    // let shares = utils::encode_message::<GF256>(data, 5, 3);
    let elements = GF256::from_bytes(&data.as_bytes());
    let shares = shamir::encode(elements.clone(), 6, 4);
    let decoded_raw = decode(shares).unwrap();
    println!("Original: {data}\nElements:     {elements:?}\nRaw decoding: {decoded_raw:?}");
    let decoded = String::from_utf8(GF256::to_bytes(&decoded_raw)).unwrap();
    println!("Decoding: {decoded}");
}
