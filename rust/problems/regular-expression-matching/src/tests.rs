use super::*;

#[test]
fn test1() {
    let actual = is_match(
        String::from("aa"),
        String::from("a"),
    );
    let expected = false;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn test2() {
    let actual = is_match(
        String::from("aa"),
        String::from("a*"),
    );
    let expected = true;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn test3() {
    let actual = is_match(
        String::from("ab"),
        String::from(".*"),
    );
    let expected = true;
    assert_eq!(
        actual,
        expected
    );
}
