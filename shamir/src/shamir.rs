use crate::fields::FieldElement;

#[derive(Debug, Copy, Clone)]
pub struct Share<D: FieldElement> {
    pub x: D,
    pub y: D,
}

/// datum: the field element to encode
/// m: the number of shares to create
/// k: the number of shares necessary to rebuild the datum
fn encode_one<D: FieldElement>(datum: D, m: usize, k: usize) -> Vec<Share<D>> {
    let polynomial = datum.sample_n_others(k - 1);

    let calculate_polynomial = |x: D| {
        let mut sum = datum.clone();
        for (i, coefficient) in polynomial.iter().enumerate() {
            let mut power = x.clone();
            for _ in 0..i {
                power = power.times(&x);
            }
            let term = power.times(coefficient);
            sum = sum.plus(&term);
        }
        sum
    };

    let mut shares = vec![];

    for x in datum.sample_n_others(m) {
        let share = Share {
            x: x.clone(),
            y: calculate_polynomial(x),
        };
        shares.push(share);
    }

    shares
}

/// data: all the field elements to encode
/// m: the number of shares to create
/// k: the number of shares necessary to rebuild the datum
pub fn encode<D: FieldElement>(data: Vec<D>, m: usize, k: usize) -> Vec<Vec<Share<D>>> {
    let mut all_shares = vec![];
    for _ in 0..m {
        all_shares.push(vec![]);
    }

    for (i, &datum) in data.iter().enumerate() {
        let shares = encode_one(datum, m, k);
        for (receiver, &share) in shares.iter().enumerate() {
            all_shares[receiver][i] = Some(share);
        }
    }

    // Unwrap all maybe_shares
    all_shares
        .into_iter()
        .map(|maybe_shares| maybe_shares.iter().map(|maybe| maybe.unwrap()).collect())
        .collect()
}
