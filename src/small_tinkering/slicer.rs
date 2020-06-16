pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn second_word(s: &str) -> &str {
    let first_word_len = first_word(&s).len();
    let useful_substring = &s[first_word_len..];
    let useful_substring = useful_substring.trim_start();

    let bytes = useful_substring.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &useful_substring[..i];
        }
    }

    &useful_substring[..]
}
