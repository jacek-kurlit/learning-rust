mod bubble;
mod insertion;
mod selection;

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;

    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn check_sort() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4]);
    }
}
