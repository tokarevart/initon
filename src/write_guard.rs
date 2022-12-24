use std::ops::{Deref, DerefMut};
use tokio::sync::RwLockMappedWriteGuard;

pub struct SingletonWriteGuard<'a, T: Sync>(pub(crate) RwLockMappedWriteGuard<'a, T>);

impl<'a, T: Sync> Deref for SingletonWriteGuard<'a, T> {
    type Target = <RwLockMappedWriteGuard<'a, T> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, T: Sync> DerefMut for SingletonWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
