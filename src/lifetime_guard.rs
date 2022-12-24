use crate::singleton::Singleton;

pub struct SingletonLifetimeGuard<'a, T: Sync + Send>(pub(crate) &'a Singleton<T>);

impl<T: Sync + Send> Drop for SingletonLifetimeGuard<'_, T> {
    fn drop(&mut self) {
        self.0.blocking_uninitialize()
    }
}
