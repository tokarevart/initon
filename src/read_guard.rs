use std::ops::Deref;
use tokio::sync::RwLockReadGuard;

pub struct SingletonReadGuard<'a, T: Sync>(pub(crate) RwLockReadGuard<'a, T>);

impl<'a, T: Sync> Deref for SingletonReadGuard<'a, T> {
    type Target = <RwLockReadGuard<'a, T> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
