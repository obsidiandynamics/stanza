use crate::lazy::Lazy;

#[test]
fn lifecycle() {
    let mut lazy = Lazy::new(|| 42);
    assert_eq!(None, lazy.try_get());
    assert_eq!(42, *lazy.get());
    assert_eq!(Some(&42), lazy.try_get());
}