#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),+) => {{
        let size = count_map!($($key),*);
        let mut m = ::std::collections::HashMap::with_capacity(size);
        $(m.insert($key, $value);)+
        m
    }};
    () => {{
        ::std::collections::HashMap::new()
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count_map {
    ($($key:expr),*) => {
        <[()]>::len(&[$($crate::count_map!(@SUBSTR; $key)),*])
    };
    (@SUBSTR; $key:expr) => {
        ()
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    #[test]
    fn create_empty_map() {
        let m: HashMap<usize, usize> = map!();
        assert!(m.is_empty());
    }

    #[test]
    fn create_single_element() {
        let m = map!("abc" => 5);
        assert_eq!(m.len(), 1);
        assert_eq!(m["abc"], 5);
    }

    #[test]
    fn create_multiple_elements() {
        let m = map!("abc" => 5, "def" => 10, "ghi" => 15);
        assert_eq!(m.len(), 3);
        assert_eq!(m.capacity(), 3);
        assert_eq!(m["abc"], 5);
        assert_eq!(m["def"], 10);
        assert_eq!(m["ghi"], 15);
    }
}
