#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

/// Reads through a shared borrow without taking ownership.
pub const fn shared_length(text: &str) -> usize {
    text.len()
}

fn increment(value: &mut i32) {
    *value += 1;
}

/// Mutates twice by creating a short reborrow for the first call.
pub fn increment_twice(value: &mut i32) {
    increment(&mut *value);
    increment(value);
}

/// Reads through a shared reborrow, then mutates after its last use.
pub fn read_then_append(text: &mut String) -> usize {
    let shared = text.as_str();
    let previous_length = shared.len();
    text.push('!');
    previous_length
}

#[cfg(test)]
mod tests {
    use super::{increment_twice, read_then_append, shared_length};

    #[test]
    fn shared_borrow_reads_without_taking_ownership() {
        // Given
        let text = String::from("rust");

        // When
        let length = shared_length(&text);

        // Then
        assert_eq!(length, 4);
        assert_eq!(text, "rust");
    }

    #[test]
    fn short_reborrow_allows_original_reference_to_be_reused() {
        // Given
        let mut value = 40;

        // When
        increment_twice(&mut value);

        // Then
        assert_eq!(value, 42);
    }

    #[test]
    fn mutable_borrow_starts_after_shared_borrows_last_use() {
        // Given
        let mut text = String::from("rust");

        // When
        let previous_length = read_then_append(&mut text);

        // Then
        assert_eq!(previous_length, 4);
        assert_eq!(text, "rust!");
    }
}
