mod utils;

use shamir::GF256;

fn main() {
    let data = "This is a test!! I am testing this!";
    println!("{:?}", utils::encode_message::<GF256>(data, 5, 3));
}
