use super::*;

#[test]
fn m2x2() {
    let matrix = vec![vec![0, 1], vec![2, 3]];
    let actual = spiral_order(matrix);
    let expected = vec![0, 1, 3, 2];
    assert_eq!(
        actual,
        expected
    );
}
#[test]
fn m3x3() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let actual = spiral_order(matrix);
    let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
    assert_eq!(
        actual,
        expected
    );
}
