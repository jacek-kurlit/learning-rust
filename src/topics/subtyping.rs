use std::fmt::Debug;

//NOTE: clippy wants to elide lifetime 'a but I didn't remove it since this shows how it works
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        //NOTE:: we must add len_utf8 because it may take more than 1 char
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

// covariance
// let x: &'a str
// x = &'a str
// x = &'static

// contravariance
// fn foo(Fn(&'a str) -> ()) {
//  bar("" /* &'a */);
// }
// foo(fn(&'static str)) this wont compile because fn(&'static str) is more strict than Fn(&'a str)
//

// invariance
// fn foo(s: &mut &'a str, x: &'a str) {
// *s = x;
// }
// let mut x: &'static str = "hello world";
// let z = String::new();
// foo(&mut x, &z);
// drop(z);
// println("{}", x); this should not compile because z was dropped and x was point to it!
// and it wont compile because &mut is invariant meaning you must provide exactly the same type

// 'a mut T is covariance in 'a but invariant in T

struct TouchDrop<T: Debug>(T);

impl<T: Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

#[allow(unused_mut, unused_variables)]
pub fn bar() {
    let mut y = true;
    let mut z = &mut y;

    let x = Box::new(true);
    let x: &'static bool = Box::leak(x);

    // z is &'y mut while x is &'static mut
    // but this works since types are exact and lifetimes are covariant
    let z = x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = "hello world";
        //NOTE: if we uncomment these 2 lines it wont compile becase compiler will require x to be
        //static while it is not
        //That's because mut ref must be invariant in T but covariant in lifetime
        //here we have compiler making lifetime as 'static (covariant) but that requires x to live
        //longer than this function scope (it on stack, it's not static), ergo 'x' dont live long enough
        //
        // fn check_is_static(_: &'static str) {}
        // check_is_static(x);
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        // assert_eq!(s, "world");
    }

    #[test]
    fn lifetime_subtyping() {
        let s = String::new();
        let x: &'static str = "hello world";
        let mut y = &*s;
        //NOTE: 'static is at least useful as 'a lifetime and therefore you can assign x to y
        y = x;
        assert_eq!(y, "hello world");
    }

    #[test]
    fn drop_check() {
        let x = String::new();
        let z = vec![&x];
        drop(x);
        // this will not compile because x was dropped and z can access in it Drop trait
        // drop(z);

        let x = String::new();
        let z = vec![TouchDrop(&x)];
        // This is wont compile as TouchDrop can access x in drop
        // drop(x);
        // I think the difference is that vec is not implementing Drop directly but with some magic
        // You can also achive this with PhantomData
        // PhantomData<T> is different than PhantomData<fn()-> T> the second one does not triggers
        // drop check on type. For example Deserializer uses this approach
        // One note: PhantomData<fn() -> T> is covariant while PhantomData<fn(T)> is contravariant
        // PhantomData<fn(T> -> T> is invariant
    }
}
