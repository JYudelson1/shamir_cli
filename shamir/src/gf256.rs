use crate::{fields::Hex, FieldElement};

// The following is in the field GF(2^8)
// Todo: Log/Exp tables would be faster
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GF256(pub u8);
impl FieldElement for GF256 {
    const ELEMENTS: usize = 256;

    fn plus(&self, other: &Self) -> Self {
        GF256(self.0 ^ other.0)
    }

    fn minus(&self, other: &Self) -> Self {
        self.plus(other)
    }

    fn times(&self, other: &Self) -> Self {
        // The [Peasant's algorithm](https://en.wikipedia.org/wiki/Finite_field_arithmetic#Rijndael's_(AES)_finite_field)
        // The gist is that at the end of each step, (a*b + product) is the  true product
        let (mut a, mut b) = (self.0, other.0);
        let mut product = 0;
        let mut carry;
        for _ in 0..8 {
            // If either a or b is 0, the product has fully accumulated
            if a == 0 || b == 0 {
                break;
            }
            // Step 1
            // If b is odd: product += a
            // I.E. product += (a * last_digit_of_b)
            if b & 1 == 1 {
                product ^= a;
            }
            // Step 2
            b >>= 1;
            // Step 3
            carry = a & 0b10000000 == 1;
            // Step 4
            a <<= 1;
            // Step 5
            // Subtract the irreducible polynomial of the field from a
            if carry {
                a ^= 0b_00011011;
            }
        }
        GF256(product)
    }

    fn inverse(&self) -> Self {
        // The inverse of an element of this finite field a would be a*254
        let self_2 = self.times(self);
        let self_4 = self_2.times(&self_2);
        let self_8 = self_4.times(&self_4);
        let self_16 = self_8.times(&self_8);
        let self_32 = self_16.times(&self_16);
        let self_64 = self_32.times(&self_32);
        let self_128 = self_64.times(&self_64);

        let self_inverse = self_128
            .times(&self_64)
            .times(&self_32)
            .times(&self_16)
            .times(&self_8)
            .times(&self_4)
            .times(&self_2);

        // let mut other_inverse = self.clone();
        // for _ in 0..253 {
        //     self_inverse = self_inverse.times(self);
        // }

        self.times(&self_inverse)
    }

    fn sample_uniform() -> Self {
        Self(rand::random::<u8>())
    }
}

impl Hex for GF256 {
    fn from_bytes<T: AsRef<[u8]>>(bytes: &T) -> Vec<Self> {
        bytes.as_ref().iter().map(|num| Self(*num)).collect()
    }

    fn to_bytes<T: AsRef<[Self]>>(data: &T) -> Vec<u8> {
        data.as_ref().iter().map(|datum| datum.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addition() {
        assert_eq!(GF256(0b_1000).plus(&GF256(0b_1110)), GF256(0b_0110));
    }

    #[test]
    fn identity() {
        assert_eq!(GF256(0b_10011010).times(&GF256(0b_1)), GF256(0b_10011010));
    }

    #[test]
    fn zero_element() {
        assert_eq!(GF256(0b_011010).times(&GF256(0b_0)), GF256(0b_0));
    }

    #[test]
    fn inverse() {
        let a = GF256(0b_01010101);
        let b = a.inverse();
        assert_eq!(a.times(&b).times(&a), a)
    }
}
