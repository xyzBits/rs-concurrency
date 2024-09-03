use std::ops::Deref;
struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
#[test]
fn test_deref() {
    let x = DerefExample { value: 'a' };
    let result = *x;

    assert_eq!('a', result);

    let result = &x;

}