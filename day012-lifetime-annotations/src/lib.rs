#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

/// 두 빌린 문자열 중 더 긴 쪽을 반환한다.
///
/// 반환 참조는 두 입력이 함께 유효한 수명 `'a` 안에서만 사용할 수 있다.
#[must_use]
pub const fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn longer_returns_left_when_left_is_longer() {
        // Given
        let left = String::from("lifetime");
        let right = String::from("Rust");

        // When
        let selected = super::longer(&left, &right);

        // Then
        assert_eq!(selected, "lifetime");
    }

    #[test]
    fn longer_returns_right_when_right_is_longer() {
        // Given
        let left = String::from("Rust");
        let right = String::from("borrow checker");

        // When
        let selected = super::longer(&left, &right);

        // Then
        assert_eq!(selected, "borrow checker");
    }
}
