use std::cell::UnsafeCell;

pub struct Cell<T> {
    //NOTE: Why UnsafeCell not just pointers?
    //Because rust will not allow to get exclusive ref to shared reference ever!
    //This is the only way
    value: UnsafeCell<T>,
}

//NOTE: implied by use of UnsafeCell as if any field is !Sync then all struct is.
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, new_value: T) {
        // SAFETY: we know no-one else is concurrently mutatning self.value because !Sync
        // SAFETY: we know we're not invalidating any references, because we never give any
        unsafe {
            *self.value.get() = new_value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        //SAFETY: we know no one else is modifying this value, since this is singl threaded
        //(!Sync) and current thread is executing this fn instead
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cell() {
        let x = Cell::new(15);
        assert_eq!(x.get(), 15);
        x.set(44);
        assert_eq!(x.get(), 44);
    }

    // #[test]
    // fn bad() {
    //     let x = Arc::new(Cell::new(15));
    //     thread::spawn(|| {
    //         x.set(44);
    //     });
    // }

    // #[test]
    // fn bad2() {
    //     let x = Cell::new(vec![44]);
    //     //NOTE: this does not compile because for safety reason get only works for Copy types
    //     //if not the below code would cause use after free error!
    //     let first = &x.get()[0];
    //     x.set(vec![]);
    //     eprintln!("{}", first);
    // }
}
