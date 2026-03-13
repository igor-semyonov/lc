use super::*;

#[test]
fn mcgt1() {
    let actual = max_chopped_in_given_time(2, 7);
    let expected = 2;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn mcgt2() {
    let actual = max_chopped_in_given_time(3, 11);
    let expected = 2;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn ex1() {
    let actual = Solution::min_number_of_seconds(
        4,
        vec![2, 1, 1],
    );
    let expected = 3;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn ex2() {
    let actual = Solution::min_number_of_seconds(
        10,
        vec![3, 2, 2, 4],
    );
    let expected = 12;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn ex3() {
    let actual = Solution::min_number_of_seconds(
        5,
        vec![1],
    );
    let expected = 15;
    assert_eq!(
        actual,
        expected
    );
}
