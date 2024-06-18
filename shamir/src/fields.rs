pub trait FieldElement: Sized + PartialEq + Clone + Copy {
    const ELEMENTS: usize;

    fn plus(&self, other: &Self) -> Self;
    fn minus(&self, other: &Self) -> Self;
    fn times(&self, other: &Self) -> Self;
    fn inverse(&self) -> Self;
    fn sample_uniform() -> Self;
    fn sample_n_others(&self, n: usize) -> Vec<Self> {
        assert!(n < Self::ELEMENTS - 1);
        let mut others = vec![];
        while others.len() < n {
            let candidate = Self::sample_uniform();
            if candidate == *self {
                continue;
            }
            let mut candidate_is_good = true;
            for other in others.iter() {
                if *other == candidate {
                    candidate_is_good = false;
                    break;
                }
            }
            if candidate_is_good {
                others.push(candidate);
            }
        }
        others
    }
}

pub trait Hex: Sized {
    fn from_bytes<T: AsRef<[u8]>>(bytes: &T) -> Vec<Self>;

    fn to_bytes<T: AsRef<[Self]>>(data: &T) -> Vec<u8>;
}
