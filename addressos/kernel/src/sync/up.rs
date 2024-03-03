use core::cell::{Ref, RefCell, RefMut};

pub struct UpSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UpSafeCell<T> {}

impl<T> UpSafeCell<T> {
    pub(crate) fn new(value: T) -> Self {
        UpSafeCell {
            inner: RefCell::new(value),
        }
    }

    pub(crate) fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    pub(crate) fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
