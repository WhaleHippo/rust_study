/// Arithmetic helpers demonstrated by this crate.
/// 이 공개 모듈은 라이브러리 사용자가 `chapter14::math` 경로로 발견할 수 있는 API의 묶음이다.
pub mod math {
    /// Adds one to an integer.
    /// 이 함수는 공개 API 예제이며, `const fn`이므로 인수가 상수 평가 문맥에 있을 때도 컴파일 시 계산할 수 있다.
    ///
    /// # Examples
    ///
    /// 아래 코드는 문서에 보이는 동시에 `cargo test --doc`가 컴파일하고 실행하는 사용 예제다.
    /// `chapter14::add_one`은 모듈 내부 경로가 아닌 재공개된 안정적인 공개 경로를 보여 준다.
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
/// 재공개는 새 함수를 만들지 않고 같은 항목을 `chapter14::add_one`이라는 짧은 공개 경로로 노출한다.
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
