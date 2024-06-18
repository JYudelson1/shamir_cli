use shamir::{encode, GF256};

fn main() {
    let data = [GF256(0b_1001), GF256(0b_1)];
    println!("{:?}", encode(data, 2, 1));
}
