#[allow(unused_macros, clippy::vec_init_then_push)]
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        const C: usize = $crate::count![$($element),*];
        let mut vs = Vec::with_capacity(C);
        $(vs.push($element);)+
        vs
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        // ::std is used for hygienic since lib user can have std mod in his project!
        // vs.extend(::std::iter::repeat($element).take($count));
        vs.resize($count, $element);
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    ($($element:expr),*) => {
        //this is a bit crazy but it create array of unit type with length of number of arguments
        //then we return length of it
        //compiler can calculate it at compile time
        <[()]>::len(&[$($crate::count!(@SUBSTR; $element)),*])

    };
    //@ does not mean anything it is only used for internal pattern matching
    //This pattern can be called by user!
    //you cannot disallow this the only thing you can do is
    //and add #[doc(hidden)]
    (@SUBSTR; $element:expr) => {
    //it just returns unit
        ()
    }
}

#[test]
fn empty_avec() {
    let v: Vec<u32> = avec![];
    assert!(v.is_empty());
}

#[test]
#[allow(clippy::vec_init_then_push)]
fn single() {
    let v: Vec<u32> = avec![32];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 32);
}

#[test]
#[allow(clippy::vec_init_then_push)]
fn double() {
    let v: Vec<u32> = avec![32, 33];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 32);
    assert_eq!(v[1], 33);
}

#[test]
#[allow(clippy::vec_init_then_push)]
fn trailing() {
    let v: Vec<u32> = avec![32, 33,];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 32);
    assert_eq!(v[1], 33);
}

#[test]
#[allow(clippy::vec_init_then_push)]
fn count() {
    let v: Vec<u32> = avec![1; 3];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 1);
    assert_eq!(v[2], 1);
}

#[test]
#[allow(clippy::vec_init_then_push)]
fn count_expression() {
    let mut y = Some(1);
    let v: Vec<u32> = avec![y.take().unwrap(); 3];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 1);
    assert_eq!(v[2], 1);
}

/// ```compile_fail
/// let x: Vec<u32> = learning_rust::avec![1; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
