/// 아직 식과 place 조작을 구현하지 않았음을 나타내는 안전한 시작값이다.
pub const PRACTICE_SENTINEL: (i32, i32) = (0, 0);

/// 블록 식으로 첫 값을 만들고 두 번째 필드 place를 같은 값으로 바꾼다.
#[must_use]
pub const fn build_matching_pair(_base: i32) -> (i32, i32) {
    PRACTICE_SENTINEL
}

#[cfg(test)]
mod tests {
    use super::build_matching_pair;

    #[test]
    #[ignore = "learning exercise"]
    fn builds_matching_pair_when_given_base_value() {
        // Given
        let base = 40;

        // When
        let pair = build_matching_pair(base);

        // Then
        assert_eq!(pair, (42, 42));
    }
}
