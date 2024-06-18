use std::iter::zip;

use crate::fields::FieldElement;

#[derive(Debug, Clone)]
pub struct Share<D: FieldElement> {
    pub x: D,
    pub y: Vec<D>,
}

/// datum: the field element to encode
/// xs: the x values at which to sample from the polynomial
/// k: the number of shares necessary to rebuild the datum
///
/// Returns: One y-value for each x in xs
fn encode_one_with_x<D: FieldElement>(datum: D, xs: &Vec<D>, k: usize) -> Vec<D> {
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

    for x in xs {
        shares.push(calculate_polynomial(*x));
    }

    shares
}

/// data: all the field elements to encode
/// m: the number of shares to create
/// k: the number of shares necessary to rebuild the datum
pub fn encode<D: FieldElement>(data: Vec<D>, m: usize, k: usize) -> Vec<Share<D>> {
    let mut all_shares = vec![];
    let xs = data[0].sample_n_others(m);
    for x in xs.clone() {
        all_shares.push(Share { x, y: vec![] });
    }

    for datum in data {
        let ys = encode_one_with_x(datum, &xs, k);
        for (y, share) in zip(ys, &mut all_shares) {
            share.y.push(y);
        }
    }

    all_shares
}

/// TODO
pub fn decode<D: FieldElement>(shares: Vec<Share<D>>) -> Option<Vec<D>> {
    let xs: Vec<D> = shares.iter().map(|share| share.x.clone()).collect();

    let mut decoded_ys = vec![];
    let messages_length = shares[0].y.len();

    // Calculate lagrange polynomials
    let mut lagrange = vec![];

    for (i, x_i) in xs.iter().enumerate() {
        let other_xs = [&xs[0..i], &xs[i + 1..]].concat();
        // Calculate products of other xs
        let mut other_xs_product = other_xs[0];
        for other_x in &other_xs[1..] {
            other_xs_product = other_xs_product.times(other_x);
        }
        // Calculate inverse of product of all (x_j - x_i)
        let mut difference_product = other_xs[0].minus(x_i);
        for &other_x in &other_xs[1..] {
            let difference = other_x.minus(x_i);
            difference_product = difference_product.times(&difference);
        }
        let inverse = difference_product.inverse();
        lagrange.push(other_xs_product.times(&inverse));
    }

    for m in 0..messages_length {
        let ys: Vec<D> = shares.iter().map(|share| share.y[m].clone()).collect();
        let mut p_0 = D::ZERO;

        for (i, y_i) in ys.iter().enumerate() {
            let term = lagrange[i].times(y_i);
            p_0 = p_0.plus(&term);
        }

        decoded_ys.push(p_0);
    }

    Some(decoded_ys)
}
