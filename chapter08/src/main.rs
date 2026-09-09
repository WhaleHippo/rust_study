use std::collections::HashMap;

fn vector_lesson() -> Vec<i32> {
    let mut values = vec![1, 2, 3];
    values.push(4);
    values
}

fn character_count(text: &str) -> usize {
    text.chars().count()
}

fn word_frequencies(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    counts
}

fn main() {
    println!("Chapter 08: Common Collections");
    println!("vector: {:?}", vector_lesson());
    println!("characters in Rust: {}", character_count("Rust"));
    println!("frequencies: {:?}", word_frequencies("rust rust book"));
}

#[cfg(test)]
mod tests {
    use super::word_frequencies;

    #[test]
    fn word_frequencies_when_words_repeat() {
        // Given: repeated words in borrowed text.
        // When: frequencies are collected.
        let counts = word_frequencies("rust rust book");
        // Then: each occurrence contributes to its entry.
        assert_eq!(counts.get("rust"), Some(&2));
    }

    #[test]
    fn vector_lesson_when_a_value_is_pushed() {
        // Given: a vector with three values.
        // When: the lesson pushes another value.
        let values = super::vector_lesson();
        // Then: Vec retains the appended value in insertion order.
        assert_eq!(values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn character_count_when_text_has_utf8_characters() {
        // Given: a string slice containing a multi-byte Unicode character.
        // When: its characters are counted.
        let count = super::character_count("hi🌍");
        // Then: the count is characters rather than UTF-8 bytes.
        assert_eq!(count, 3);
    }

    #[test]
    fn word_frequencies_when_text_is_empty() {
        // Given: borrowed text without words.
        // When: entries are counted.
        let counts = word_frequencies("");
        // Then: no HashMap entries are created.
        assert!(counts.is_empty());
    }

    #[test]
    fn word_frequencies_when_one_word_occurs_three_times() {
        // Given: one borrowed word repeated three times.
        // When: frequencies are collected.
        let counts = word_frequencies("rust rust rust book");
        // Then: all three occurrences contribute to the entry.
        assert_eq!(counts.get("rust"), Some(&3));
    }
}
