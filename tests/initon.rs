use std::fmt::Display;

use initon::Singleton;

#[tokio::test]
async fn create_uninit_then_init() {
    static SINGLETON: Singleton<i32> = Singleton::uninitialized();
    let life = SINGLETON.initialize(42).await.unwrap();
    assert!(SINGLETON.is_initialized().await);
    assert_eq!(*SINGLETON.read().await.unwrap(), 42);
    drop(life);
    assert!(!SINGLETON.is_initialized().await);
}

#[tokio::test]
async fn create_init() {
    let singleton = Singleton::initialized(42);
    assert!(singleton.is_initialized().await);
    assert_eq!(*singleton.read().await.unwrap(), 42);
    let life = singleton.lifetime_guard().await.unwrap();
    drop(life);
    assert!(!singleton.is_initialized().await);
}

#[tokio::test]
#[should_panic]
async fn freed_resources() {
    struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!()
        }
    }

    let _ = Singleton::initialized(PanicOnDrop);
}

#[tokio::test]
async fn create_with_boxed() {
    let singleton: Singleton<Box<dyn Display + Send + Sync>> = Singleton::uninitialized();
    let life = singleton.initialize(Box::new(42)).await.unwrap();
    assert!(singleton.is_initialized().await);
    assert_eq!(&singleton.read().await.unwrap().to_string(), "42");
    drop(life);
    assert!(!singleton.is_initialized().await);
}
