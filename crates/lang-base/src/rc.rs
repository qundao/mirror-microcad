// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Short-cut definition of `Rc<std::cell::RefCell<T>>` and `Rc<T>`

pub use std::rc::Rc;

use std::{
    cell::RefCell,
    sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use derive_more::{Deref, DerefMut};

/// Just a short cut definition
#[derive(Deref, DerefMut)]
pub struct RcMut<T>(Rc<RefCell<T>>);

impl<T> RcMut<T> {
    /// Create new instance
    pub fn new(t: T) -> Self {
        Self(Rc::new(RefCell::new(t)))
    }
}

impl<T> Clone for RcMut<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for RcMut<T> {
    fn from(value: T) -> Self {
        RcMut::new(value)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for RcMut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RcMut").field(&self.0.borrow()).finish()
    }
}

/// An object that can be safely shared and mutated across contexts and threads.
#[repr(transparent)]
#[derive(Debug)]
pub struct Shared<T>(Arc<RwLock<T>>);

impl<T> Shared<T> {
    pub fn new(val: T) -> Self {
        Self(Arc::new(RwLock::new(val)))
    }

    #[inline]
    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        self.0.read()
    }

    #[inline]
    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        self.0.write()
    }

    #[inline]
    pub fn read_unwrap(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().expect("Shared lock poisoned on read")
    }

    #[inline]
    pub fn write_unwrap(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().expect("Shared lock poisoned on write")
    }

    #[inline]
    pub fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.read_unwrap();
        f(&*guard)
    }

    #[inline]
    pub fn with_write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.write_unwrap();
        f(&mut *guard)
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> From<T> for Shared<T> {
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<Arc<RwLock<T>>> for Shared<T> {
    fn from(arc: Arc<RwLock<T>>) -> Self {
        Self(arc)
    }
}

impl<T> From<Shared<T>> for Arc<RwLock<T>> {
    fn from(shared: Shared<T>) -> Self {
        shared.0
    }
}

impl<T> Shared<T> {
    /// Returns true if two `Shared` pointers point to the same allocation.
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        Arc::ptr_eq(&this.0, &other.0)
    }
}
