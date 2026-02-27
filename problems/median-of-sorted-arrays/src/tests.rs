use super::*;

#[test]
fn simple1x1() {
    let nums1 = vec![1];
    let nums2 = vec![2];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 1.5;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn simple2x2() {
    let nums1 = vec![0, 1];
    let nums2 = vec![2, 3];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 1.5;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn simple3x3() {
    let nums1 = vec![0, 1, 2];
    let nums2 = vec![2, 3, 4];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 2.0;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn not_so_simple10x2() {
    let nums1 = (0..10).collect();
    let nums2 = vec![3, 4];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 4.0;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn not_so_simple11x3() {
    let nums1 = (0..11).collect();
    let nums2 = vec![3, 4, 5];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 4.5;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn not_so_simple3x11() {
    let nums1 = vec![3, 4, 5];
    let nums2 = (0..11).collect();
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 4.5;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn not_so_simple10x3() {
    let nums1 = (0..10).collect();
    let nums2 = vec![0, 2, 8];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 4.0;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn not_so_simple10x3v2() {
    let nums1 = (0..10).collect();
    let nums2 = vec![0, 5, 8];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 5.0;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn not_so_simple10x3_topheavy() {
    let nums1 = (0..10).collect();
    let nums2 = vec![0, 6, 8];
    let actual = find_median_sorted_arrays(
        nums1, nums2,
    );
    let expected = 5.0;
    assert_eq!(
        actual,
        expected
    );
}
