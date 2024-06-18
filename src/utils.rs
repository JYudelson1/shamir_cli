use std::iter::zip;

use shamir::{encode, decode, FieldElement, Hex, Share};

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

fn fragment_to_shares<D: FieldElement + Hex>(fragment: &str) -> Vec<Share<D>> {
    let x_raw = &fragment[..(fragment.len() / 2)];
    let y_raw = &fragment[(fragment.len() / 2)..];

    let xs = message_to_elements(x_raw);
    let ys = message_to_elements(y_raw);

    let mut shares = vec![];

    for (x, y) in zip(xs, ys) {
        shares.push(Share { x, y });
    }

    shares
}

fn decode_fragments<D: FieldElement + Hex>(fragments: Vec<&str>) -> Option<String> {
    let all_shares = fragments
        .iter()
        .map(|fragment| fragment_to_shares(fragment))
        .collect();

    decode(all_shares)
        .map(|elements| D::to_bytes(&elements))
        .map(|bytes| String::from_utf8(bytes).unwrap())
}
