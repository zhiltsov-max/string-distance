use string_distance::*;

#[test]
fn test_ascii_is_ascii() {
    assert!(is_ascii(&"abc".to_owned()));
}

#[test]
fn test_unicode_is_not_ascii() {
    assert!(!is_ascii(&"\u{23F0}".to_owned()));
}

#[test]
fn test_can_compute_char_difference_for_ascii() {
    assert_eq!(count_char_diff(&"abc".to_owned(), &"abd".to_owned()), 2);
}

#[test]
fn test_can_compute_char_difference_for_unicode() {
    assert_eq!(
        count_char_diff(
            &"ab\u{23F0}\u{23F1}".to_owned(),
            &"\u{23F0}\u{23F2}ad".to_owned()
        ),
        4
    );
}

#[test]
fn test_can_compute_distance() {
    assert_eq!(
        levenshtein(&"a\u{23F0}c".to_owned(), &"ab\u{23F0}".to_owned()),
        2
    );
}
