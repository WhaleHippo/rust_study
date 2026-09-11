//! Feature-gated learner exercise.

/// Starter sentinel: increment through two sequential short reborrows.
pub fn learner_increment_twice(value: &mut i32) {
    let _ = value;
}

#[cfg(test)]
mod tests {
    use super::learner_increment_twice;

    #[test]
    #[ignore = "learning exercise"]
    fn learner_reborrows_mutable_reference_twice() {
        // Given
        let mut value = 40;

        // When
        learner_increment_twice(&mut value);

        // Then
        assert_eq!(value, 42);
    }
}
