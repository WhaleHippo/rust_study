fn first_word(text: &str) -> &str {
    for (index, byte) in text.bytes().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }
    text
}

fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    println!("Chapter 04: Understanding Ownership");

    let greeting = String::from("hello world");
    let moved_greeting = greeting;
    let cloned_greeting = moved_greeting.clone();
    let borrowed_length = moved_greeting.len();
    let first = first_word(&moved_greeting);
    let greeting_slice = &moved_greeting[..5];

    let mut editable = String::from("borrowed mutably");
    add_exclamation(&mut editable);

    println!("move: {moved_greeting}; clone: {cloned_greeting}");
    println!("borrow: length={borrowed_length}; slice={greeting_slice}; first={first}");
    println!("mutable borrow: {editable}");
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn returns_prefix_when_text_contains_space() {
        // Given: text containing a word boundary.
        let text = "hello world";
        // When: its first word is requested.
        let word = first_word(text);
        // Then: the slice stops before the first space.
        assert_eq!(word, "hello");
    }

    #[test]
    fn returns_entire_slice_when_text_has_one_word() {
        // Given: text without a word boundary.
        let text = "ownership";
        // When: its first word is requested.
        let word = first_word(text);
        // Then: the whole borrowed slice is returned.
        assert_eq!(word, "ownership");
    }

    #[test]
    fn returns_empty_slice_when_text_starts_with_space() {
        // Given: text whose first character is a space.
        let text = " leading";
        // When: its first word is requested.
        let word = first_word(text);
        // Then: no characters precede the boundary.
        assert_eq!(word, "");
    }
}
