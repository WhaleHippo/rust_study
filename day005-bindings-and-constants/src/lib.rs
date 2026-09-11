#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

pub const MINUTES_PER_HOUR: u32 = 60;
pub static COURSE_NAME: &str = "Rust 기초";

pub const fn binding_snapshot(start: u32) -> (u32, u32, usize) {
    let immutable = start;

    let mut mutable = immutable;
    mutable += 2;

    let shadowed = "Rust";
    let shadowed = shadowed.len();

    (immutable, mutable, shadowed)
}

pub const fn hours_to_minutes(hours: u32) -> u32 {
    hours.saturating_mul(MINUTES_PER_HOUR)
}

#[cfg(test)]
mod tests {
    use super::{COURSE_NAME, MINUTES_PER_HOUR, binding_snapshot, hours_to_minutes};

    #[test]
    fn bindings_reveal_mutation_and_shadowing_when_snapshot_is_built() {
        // Given
        let start = 3;

        // When
        let snapshot = binding_snapshot(start);

        // Then
        assert_eq!(snapshot, (3, 5, 4));
    }

    #[test]
    fn constants_are_available_when_the_crate_is_used() {
        // Given
        let hours = 2;

        // When
        let minutes = hours_to_minutes(hours);

        // Then
        assert_eq!(
            (minutes, MINUTES_PER_HOUR, COURSE_NAME),
            (120, 60, "Rust 기초")
        );
    }
}
