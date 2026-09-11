#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

/// Takes ownership of a string and returns the modified owner.
pub fn append_owned(mut text: String, suffix: &str) -> String {
    text.push_str(suffix);
    text
}

/// Copies a `Copy` value into two independently usable values.
pub const fn copy_pair(number: u32) -> (u32, u32) {
    (number, number)
}

#[cfg(test)]
mod tests {
    use super::{append_owned, copy_pair};
    use std::rc::Rc;

    #[test]
    fn ownership_moves_when_owned_string_is_passed() {
        // Given
        let label = String::from("rust");

        // When
        let label = append_owned(label, "acean");

        // Then
        assert_eq!(label, "rustacean");
    }

    #[test]
    fn integer_remains_usable_when_copied() {
        // Given
        let score = 21;

        // When
        let pair = copy_pair(score);

        // Then
        assert_eq!(pair, (score, 21));
    }

    #[test]
    fn string_clone_has_independent_contents() {
        // Given
        let original = String::from("rust");

        // When
        let mut cloned = original.clone();
        cloned.push('!');

        // Then
        assert_eq!(original, "rust");
        assert_eq!(cloned, "rust!");
    }

    #[test]
    fn clone_may_share_underlying_ownership() {
        // Given
        let original = Rc::new(String::from("shared"));

        // When
        let cloned = Rc::clone(&original);

        // Then
        assert!(Rc::ptr_eq(&original, &cloned));
        assert_eq!(Rc::strong_count(&original), 2);
    }
}
