/// Arithmetic helpers demonstrated by this crate.
pub mod math {
    /// Adds one to an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use chapter14::add_one;
    ///
    /// assert_eq!(add_one(41), 42);
    /// ```
    pub const fn add_one(value: i32) -> i32 {
        value + 1
    }
}

/// Re-exports the documented arithmetic helper at the crate root.
pub use math::add_one;

#[cfg(test)]
mod tests {
    use super::add_one;

    #[test]
    fn add_one_when_value_is_forty_one() {
        // Given: a value one below the answer.
        // When: the documented helper is called.
        let result = add_one(41);
        // Then: it returns forty two.
        assert_eq!(result, 42);
    }

    #[test]
    fn add_one_when_value_is_negative_one() {
        // Given: the integer directly below zero.
        // When: the documented helper is called.
        let result = add_one(-1);
        // Then: arithmetic crosses zero correctly.
        assert_eq!(result, 0);
    }

    #[test]
    fn add_one_reexport_matches_its_public_module() {
        // Given: the module path and root re-export.
        // When: both public entry points are used.
        let results = (add_one(4), super::math::add_one(4));
        // Then: the re-export remains connected to the documented function.
        assert_eq!(results, (5, 5));
    }

    #[test]
    fn add_one_when_value_is_negative_two() {
        // Given: a negative integer below the existing edge case.
        // When: the public helper is called.
        let result = add_one(-2);
        // Then: adding one keeps the correct negative result.
        assert_eq!(result, -1);
    }
}
