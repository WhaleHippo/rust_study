#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

/// Returns the prefix before the first Unicode whitespace character.
pub fn first_word(text: &str) -> &str {
    text.char_indices()
        .find_map(|(index, character)| character.is_whitespace().then_some(index))
        .map_or(text, |index| &text[..index])
}

/// Returns a borrowed tail when `start` is within the slice boundary.
pub fn tail_from<T>(values: &[T], start: usize) -> Option<&[T]> {
    values.get(start..)
}

#[cfg(test)]
mod tests {
    use super::{first_word, tail_from};

    #[test]
    fn first_word_returns_borrowed_prefix_for_normal_text() {
        // Given
        let sentence = String::from("rust ownership");

        // When
        let word = first_word(&sentence);

        // Then
        assert_eq!(word, "rust");
    }

    #[test]
    fn first_word_handles_empty_text() {
        // Given
        let sentence = "";

        // When
        let word = first_word(sentence);

        // Then
        assert_eq!(word, "");
    }

    #[test]
    fn first_word_handles_leading_whitespace_boundary() {
        // Given
        let sentence = " rust";

        // When
        let word = first_word(sentence);

        // Then
        assert_eq!(word, "");
    }

    #[test]
    fn first_word_slices_only_at_utf8_boundaries() {
        // Given
        let sentence = "소유권 배우기";

        // When
        let word = first_word(sentence);

        // Then
        assert_eq!(word, "소유권");
    }

    #[test]
    fn first_word_returns_whole_text_without_separator() {
        // Given
        let sentence = "rust";

        // When
        let word = first_word(sentence);

        // Then
        assert_eq!(word, "rust");
    }

    #[test]
    fn tail_from_borrows_normal_empty_and_boundary_slices() {
        // Given
        let numbers = [10, 20, 30];

        // When
        let middle = tail_from(&numbers, 1);
        let boundary = tail_from(&numbers, numbers.len());
        let empty = tail_from::<i32>(&[], 0);

        // Then
        assert_eq!(middle, Some(&[20, 30][..]));
        assert_eq!(boundary, Some(&[][..]));
        assert_eq!(empty, Some(&[][..]));
    }
}
