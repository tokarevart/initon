use initon::Singleton;

#[tokio::test]
async fn create_init() {
    static SINGLETON: Singleton<Vec<i32>> = Singleton::uninitialized();
    let life = SINGLETON.initialize(vec![0, 1]).await.unwrap();
    assert!(SINGLETON.is_initialized().await);
    drop(life);
    assert!(!SINGLETON.is_initialized().await);
}

#[tokio::test]
async fn create_with() {
    let singleton = Singleton::new_with(vec![0, 1]);
    assert!(singleton.is_initialized().await);
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

    let _ = Singleton::new_with(PanicOnDrop);
}
