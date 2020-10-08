/// Returns the number of upper case letters in `word`
pub fn count_upper(word: &str) -> usize {
    let mut num = 0;
    for c in word.chars() {
        if c.is_uppercase() {
            num += 1;
        }
    }
    num

    // alternatively:
    // word.chars().filter(|c| c.is_uppercase()).count()
}

/// Returns a tuple with the number of words in `text` that contain upper case letters and the total
/// number of upper case letters
pub fn count_words_upper(text: &str) -> (usize, usize) {
    let mut num = (0, 0);
    for word in text.split_whitespace() {
        let upper = count_upper(word);
        if upper > 0 {
            num.0 += 1;
            num.1 += upper;
        }
    }
    num

    // alternatively:
    // text.split_whitespace()
    //     .fold((0, 0), |num, word| match count_upper(word) {
    //         0 => num,
    //         n => (num.0 + 1, num.1 + n),
    //     })
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
