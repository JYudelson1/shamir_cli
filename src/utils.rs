use shamir::{encode, decode, FieldElement, Hex, Share};

fn shares_to_hex<D: FieldElement + Hex>(shares: Share<D>) -> Vec<u8> {
    let mut x = D::to_bytes(&[shares.x]);
    x.extend(D::to_bytes(&shares.y));
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

fn fragment_to_share<D: FieldElement + Hex>(fragment: &String) -> Share<D> {
    let x_raw = &fragment[..D::LEN_IN_BYTES];
    let y_raw = &fragment[D::LEN_IN_BYTES..];

    let x = message_to_elements(x_raw);
    let ys = message_to_elements(y_raw);

    assert_eq!(1, x.len());
    let x = x[0];

    Share { x, y: ys }
}

pub fn decode_message<D: FieldElement + Hex>(fragments: Vec<String>) -> Option<String> {
    let all_shares = fragments
        .iter()
        .map(|fragment| fragment_to_share(fragment))
        .collect();

    decode(all_shares)
        .map(|elements| D::to_bytes(&elements))
        .map(|bytes| String::from_utf8(bytes).unwrap())
}
