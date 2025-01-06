use super::cell::Cell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

pub struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
pub struct Rc<T> {
    shared: NonNull<RcInner<T>>,
    //NOTE: This is for Drop check to tell rust that when we drop Rc then RcInner<T> might be
    //droped and it needs to check it
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let shared = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Self {
            // SAFETY: box does not give null ptr
            shared: unsafe { NonNull::new_unchecked(Box::into_raw(shared)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let shared = unsafe { self.shared.as_ref() };
        let refcount = shared.refcount.get();
        shared.refcount.set(refcount + 1);
        Self {
            shared: self.shared,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: salfe.shared is a Box that is only deallocated when last RC goes away
        // we have Rc therefore the Box has not be deallocated so dref is safe
        &unsafe { self.shared.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let shared = unsafe { self.shared.as_ref() };
        let refcount = shared.refcount.get();
        if refcount == 1 {
            // drop(shared); clippy says it does nothing
            // SAFETY: we are the only Rc and we are being droped
            // therefore after us there will be Rc's and no references to T
            let _ = unsafe { Box::from_raw(self.shared.as_ptr()) };
        } else {
            //There are other Rc's so dont drop the Box
            shared.refcount.set(refcount - 1);
        }
    }
}
