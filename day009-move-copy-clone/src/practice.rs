//! Feature-gated learner exercise.

/// Starter sentinel: move `text`, append `"acean"`, and return its new owner.
pub fn learner_append_owned(text: String) -> String {
    text
}

#[cfg(test)]
mod tests {
    use super::learner_append_owned;

    #[test]
    #[ignore = "learning exercise"]
    fn learner_moves_and_modifies_owned_string() {
        // Given
        let text = String::from("rust");

        // When
        let result = learner_append_owned(text);

        // Then
        assert_eq!(result, "rustacean");
    }
}
