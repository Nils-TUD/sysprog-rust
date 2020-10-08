/// Returns the number of upper case letters in `word`
pub fn count_upper(word: &str) -> usize {
    // TODO
    0
}

/// Returns a tuple with the number of words in `text` that contain upper case letters and the total
/// number of upper case letters
pub fn count_words_upper(text: &str) -> (usize, usize) {
    // TODO
    (0, 0)
}

#[test]
fn test_count_upper() {
    assert_eq!(count_upper(""), 0);
    assert_eq!(count_upper("abc"), 0);
    assert_eq!(count_upper("Abc"), 1);
    assert_eq!(count_upper("CCË"), 3);
}

#[test]
fn test_count_words_upper() {
    assert_eq!(count_words_upper(""), (0, 0));
    assert_eq!(count_words_upper("foo"), (0, 0));
    assert_eq!(count_words_upper("fOÅ"), (1, 2));
    assert_eq!(count_words_upper("Foo bar\tBaZ"), (2, 3));
    assert_eq!(count_words_upper("123\nbar   ABC"), (1, 3));
}
