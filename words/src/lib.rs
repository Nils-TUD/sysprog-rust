/// Returns the number of upper case letters in `word`
pub fn count_upper(word: &str) -> usize {
    let mut upper = 0;
    for c in word.chars() {
        if c.is_uppercase() {
            upper += 1;
        }
    }
    upper
}

/// Returns a tuple with the number of words in `text` that contain upper case letters and the total
/// number of upper case letters
pub fn count_words_upper(text: &str) -> (usize, usize) {
    let mut total_upper = 0;
    let mut upper_words = 0;
    for w in text.split_whitespace() {
        let upper = count_upper(w);
        if upper > 0 {
            total_upper += upper;
            upper_words += 1;
        }
    }
    (upper_words, total_upper)
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
