use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct Boks<T> {
    p: NonNull<T>,
    _p: PhantomData<fn() -> T>,
}

impl<T> Boks<T> {
    pub fn new(t: T) -> Self {
        Boks {
            //Safety Box never creates null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _p: PhantomData,
        }
    }
}

//NOTE: normaly Drop assumes that value will be accessed in drop metod
//dropck_eyepatch and may_dangle assures compiler it will not
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {
            // Safety: p was constructed from Box in the first place, and hasn't been freed
            // otherwise because self still exists(drop could been done otherwise)
            let _ = Box::from_raw(self.p.as_mut());
        }
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers and hasn't been freed
        // since self is alive
        unsafe { self.p.as_ref() }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers and hasn't been freed
        // since self is alive
        // Also since we have &mut self no other mutable reference has been given out to p
        unsafe { &mut *self.p.as_mut() }
    }
}

struct UpsiDaisy<T: Debug>(T);

impl<T: Debug> Drop for UpsiDaisy<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_works() {
        let x = 42;
        let b = Boks::new(x);
        assert_eq!(*b, 42);
    }

    #[test]
    fn mut_deref_works() {
        let mut x = 42;
        let b = Boks::new(&mut x);
        //this wont compile because compiler assumes that drop access T
        //if Drop impl is removed this will compile
        // println!("{:?}", x);
        drop(b);
        //other way to compile this is to active #[feature(dropck_eyepatch)] on Boks type
        //this is unstable feature from nightly, it just says that Drop is not accessing T
    }

    #[test]
    fn upsi_daisy() {
        let mut z = 42;
        let b = Boks::new(UpsiDaisy(&mut z));
        // this won't compile but it will if we remove _p from Boks
        // println!("{:?}", z);
    }

    #[test]
    fn covariance() {
        let z = String::from("hi!");
        let mut boks1 = Boks::new(&*z);
        let boks2: Boks<&'static str> = Boks::new("this si static");
        boks1 = boks2;
    }
}
