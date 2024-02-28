use core::cell::RefCell;

pub(crate) struct UpSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UpSafeCell<T> {}
unsafe impl<T> Send for UpSafeCell<T> {}

impl<T> UpSafeCell<T> {
    fn new(value: T) -> Self {
        UpSafeCell {
            inner: RefCell::new(value),
        }
    }

    fn borrow(&self) -> core::cell::Ref<T> {
        self.inner.borrow()
    }

    fn borrow_mut(&self) -> core::cell::RefMut<T> {
        self.inner.borrow_mut()
    }
}