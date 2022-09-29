use crate::memoized::Memoized;

#[test]
fn lifecycle() {
    let mut m = Memoized::new(|| 42);
    assert_eq!(None, m.try_get());
    assert_eq!(42, *m.get());
    assert_eq!(Some(&42), m.try_get());
}