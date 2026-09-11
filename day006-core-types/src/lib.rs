#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

pub const CORE_SCALARS: (i32, f64, bool, char) = (-7, 2.5, true, '러');

pub const fn array_and_tuple() -> ([u8; 3], (&'static str, u16, bool)) {
    ([10, 20, 30], ("Rust", 2024, true))
}

pub fn range_totals() -> (u32, u32) {
    ((1..4).sum(), (1..=4).sum())
}

pub fn safe_array_value(values: &[u8], index: usize) -> Option<u8> {
    values.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::{CORE_SCALARS, array_and_tuple, range_totals, safe_array_value};

    #[test]
    fn scalar_values_keep_their_distinct_types() {
        // Given
        let expected = (-7, 2.5, true, '러');

        // When
        let actual = CORE_SCALARS;

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn compound_types_preserve_length_and_positions() {
        // Given
        let expected = ([10, 20, 30], ("Rust", 2024, true));

        // When
        let actual = array_and_tuple();

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn ranges_differ_when_the_end_is_inclusive() {
        // Given
        let expected = (6, 10);

        // When
        let actual = range_totals();

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn array_access_returns_none_when_index_is_out_of_bounds() {
        // Given
        let values = [10, 20, 30];

        // When
        let actual = safe_array_value(&values, 3);

        // Then
        assert_eq!(actual, None);
    }
}
