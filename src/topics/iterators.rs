use std::collections::HashSet;

#[allow(dead_code)]
pub trait ToSet: Iterator {
    fn to_set(self) -> HashSet<Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + std::hash::Hash,
    {
        self.collect::<HashSet<Self::Item>>()
    }
}

impl<I: Iterator> ToSet for I {}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::ToSet;

    #[test]
    fn should_create_hash_set() {
        let data = [1, 2, 1, 3, 2, 2, 4];
        assert_eq!(data.iter().copied().to_set(), HashSet::from([1, 2, 3, 4]));
    }
}
