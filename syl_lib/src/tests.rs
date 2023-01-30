use crate::util::singular_plural;

#[test]
fn test_plural() {
    assert_eq!(singular_plural("balls", 0), "balls");
    assert_eq!(singular_plural("balls", 1), "ball");
    assert_eq!(singular_plural("balls", 2), "balls");
}
