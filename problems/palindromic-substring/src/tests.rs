use super::*;

#[test]
fn empty(){
    let actual = longest_palindrome(String::from(""));
    let expected = String::from("");
    assert_eq!(actual, expected);
}

#[test]
fn singleton(){
    let actual = longest_palindrome(String::from("a"));
    let expected = String::from("a");
    assert_eq!(actual, expected);
}

#[test]
fn two_in_two(){
    let actual = longest_palindrome(String::from("aa"));
    let expected = String::from("aa");
    assert_eq!(actual, expected);
}

#[test]
fn threee_in_three(){
    let actual = longest_palindrome(String::from("aba"));
    let expected = String::from("aba");
    assert_eq!(actual, expected);
}

#[test]
fn two_in_three(){
    let actual = longest_palindrome(String::from("aab"));
    let expected = String::from("aa");
    assert_eq!(actual, expected);
}

#[test]
fn three_in_six(){
    let actual = longest_palindrome(String::from("abcdce"));
    let expected = String::from("cdc");
    assert_eq!(actual, expected);
}
