use super::*;

#[test]
fn s2() {
    let s1 = "0";
    let actual = find_next_nth_string(&s1);
    let expected = "011";
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn s4() {
    let actual = find_nth_string(4);
    let expected = "011100110110001";
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn n3k1(){
    let expected = '0';
    let actual = find_kth_bit(3, 1);
    assert_eq!(expected, actual);
}

#[test]
fn n4k11(){
    let expected = '1';
    let actual = find_kth_bit(4, 11);
    assert_eq!(expected, actual);
}

#[test]
fn n2(){
    let actual = find_next_nth_num(0);
    let expected = 0b011;
    assert_eq!(
        actual,
        expected
    );
}

#[test]
fn n4(){
    let actual = find_nth_num(4);
    let expected = 0b011100110110001;
    assert_eq!(
        actual,
        expected
    );
}
