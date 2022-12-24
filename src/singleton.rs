use crate::{
    lifetime_guard::SingletonLifetimeGuard, read_guard::SingletonReadGuard,
    write_guard::SingletonWriteGuard,
};
use anyhow::bail;
use std::thread;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct Singleton<T: Sync + Send>(RwLock<Option<T>>);

impl<T: Sync + Send> Singleton<T> {
    pub const fn uninitialized() -> Self {
        Self(RwLock::const_new(None))
    }

    pub const fn new_with(value: T) -> Self {
        Self(RwLock::const_new(Some(value)))
    }

    pub async fn lifetime_guard(&self) -> Option<SingletonLifetimeGuard<T>> {
        self.0
            .read()
            .await
            .as_ref()
            .map(|_| SingletonLifetimeGuard(self))
    }

    pub async fn initialize(&self, value: T) -> anyhow::Result<SingletonLifetimeGuard<T>> {
        let mut opt = self.0.write().await;
        if opt.is_none() {
            *opt = Some(value);
            Ok(SingletonLifetimeGuard(self))
        } else {
            bail!("initialization has already been done");
        }
    }

    pub async fn is_initialized(&self) -> bool {
        self.0.read().await.is_some()
    }

    pub(crate) fn blocking_uninitialize(&self) {
        thread::scope(|s| s.spawn(|| self.0.blocking_write().take()).join().unwrap());
    }

    pub async fn read(&self) -> anyhow::Result<SingletonReadGuard<T>> {
        let guard = self.0.read().await;
        if guard.is_none() {
            bail!("initialization hasn't been done");
        }
        Ok(SingletonReadGuard(RwLockReadGuard::map(guard, |x| {
            x.as_ref().unwrap()
        })))
    }

    pub async fn write(&self) -> anyhow::Result<SingletonWriteGuard<T>> {
        let guard = self.0.write().await;
        if guard.is_none() {
            bail!("initialization hasn't been done");
        }
        Ok(SingletonWriteGuard(RwLockWriteGuard::map(guard, |x| {
            x.as_mut().unwrap()
        })))
    }
}
