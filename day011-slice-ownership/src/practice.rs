//! Feature-gated learner exercise.

/// Starter sentinel: return only the first word as a borrowed slice.
pub fn learner_first_word(text: &str) -> &str {
    text
}

#[cfg(test)]
mod tests {
    use super::learner_first_word;

    #[test]
    #[ignore = "learning exercise"]
    fn learner_returns_utf8_safe_borrowed_first_word() {
        // Given
        let sentence = String::from("소유권 배우기");

        // When
        let word = learner_first_word(&sentence);

        // Then
        assert_eq!(word, "소유권");
    }
}
