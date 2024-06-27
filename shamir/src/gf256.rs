use crate::{fields::Hex, FieldElement};

const INVERSES: [u8; 256] = [
    0, 1, 141, 246, 203, 82, 123, 209, 232, 79, 41, 192, 176, 225, 229, 199, 116, 180, 170, 75,
    153, 43, 96, 95, 88, 63, 253, 204, 255, 64, 238, 178, 58, 110, 90, 241, 85, 77, 168, 201, 193,
    10, 152, 21, 48, 68, 162, 194, 44, 69, 146, 108, 243, 57, 102, 66, 242, 53, 32, 111, 119, 187,
    89, 25, 29, 254, 55, 103, 45, 49, 245, 105, 167, 100, 171, 19, 84, 37, 233, 9, 237, 92, 5, 202,
    76, 36, 135, 191, 24, 62, 34, 240, 81, 236, 97, 23, 22, 94, 175, 211, 73, 166, 54, 67, 244, 71,
    145, 223, 51, 147, 33, 59, 121, 183, 151, 133, 16, 181, 186, 60, 182, 112, 208, 6, 161, 250,
    129, 130, 131, 126, 127, 128, 150, 115, 190, 86, 155, 158, 149, 217, 247, 2, 185, 164, 222,
    106, 50, 109, 216, 138, 132, 114, 42, 20, 159, 136, 249, 220, 137, 154, 251, 124, 46, 195, 143,
    184, 101, 72, 38, 200, 18, 74, 206, 231, 210, 98, 12, 224, 31, 239, 17, 117, 120, 113, 165,
    142, 118, 61, 189, 188, 134, 87, 11, 40, 47, 163, 218, 212, 228, 15, 169, 39, 83, 4, 27, 252,
    172, 230, 122, 7, 174, 99, 197, 219, 226, 234, 148, 139, 196, 213, 157, 248, 144, 107, 177, 13,
    214, 235, 198, 14, 207, 173, 8, 78, 215, 227, 93, 80, 30, 179, 91, 35, 56, 52, 104, 70, 3, 140,
    221, 156, 125, 160, 205, 26, 65, 28,
];

// The following is in the field GF(2^8)
// Todo: Log/Exp tables would be faster
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GF256(pub u8);
impl FieldElement for GF256 {
    const ELEMENTS: usize = 256;
    const ZERO: Self = Self(0);

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
            carry = a & 0b10000000 == 0b10000000;
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
        assert_ne!(self.0, 0, "Cannot divide by zero!");
        GF256(INVERSES[self.0 as usize])
    }

    fn sample_uniform() -> Self {
        Self(rand::random::<u8>())
    }
}

impl Hex for GF256 {
    const LEN_IN_BYTES: usize = 1;

    fn from_bytes<T: AsRef<[u8]>>(bytes: &T) -> Vec<Self> {
        bytes.as_ref().iter().map(|num| Self(*num)).collect()
    }

    fn to_bytes<T: AsRef<[Self]>>(data: &T) -> Vec<u8> {
        data.as_ref().iter().map(|datum| datum.0).collect()
    }
}

impl GF256 {
    pub fn explicit_inverse(&self) -> Self {
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
            .times(&self);

        // let mut other_inverse = self.clone();
        // for _ in 0..253 {
        //     self_inverse = self_inverse.times(self);
        // }

        self.times(&self_inverse)
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
        let a = GF256(46);
        let b = a.inverse();
        assert_eq!(a.times(&b).times(&a), a)
    }

    #[test]
    fn multiply() {
        let a = GF256(0b_01010111);
        let b = GF256(0b_00010011);
        assert_eq!(a.times(&b), GF256(0b_11111110));
    }
}
