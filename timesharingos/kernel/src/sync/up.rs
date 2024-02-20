use core::cell::{RefCell, RefMut};

pub(crate) struct UpSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UpSafeCell<T> {}

impl<T> UpSafeCell<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self {
            inner: RefCell::new(inner),
        }
    }

    pub(crate) fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
