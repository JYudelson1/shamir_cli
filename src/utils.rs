use hex;
use shamir::{encode, FieldElement, Hex, Share};

fn shares_to_hex<D: FieldElement + Hex>(shares: Vec<Share<D>>) -> Vec<u8> {
    let mut x = vec![];
    let mut y = vec![];
    for share in shares {
        x.extend(D::to_bytes(&[share.x]));
        y.extend(D::to_bytes(&[share.y]));
    }

    x.append(&mut y);
    x
}

fn hex_to_ascii(hex: Vec<u8>) -> String {
    hex::encode(hex)
}

fn message_to_elements<D: FieldElement + Hex>(message: &str) -> Vec<D> {
    let bytes = message.as_bytes();
    D::from_bytes(&bytes)
}

pub fn encode_message<D: FieldElement + Hex>(message: &str, m: usize, k: usize) -> Vec<String> {
    let elements = message_to_elements::<D>(message);
    let shares = encode(elements, m, k);

    let mut all_fragments = vec![];

    for fragment in shares {
        let hex_fragment = shares_to_hex(fragment);
        let hex_string = hex_to_ascii(hex_fragment);
        all_fragments.push(hex_string);
    }

    all_fragments
}
