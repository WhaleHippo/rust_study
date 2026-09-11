//! `learner-practice` 기능을 켰을 때만 포함되는 연습 시작점이다.

/// TODO: 이미 표현된 공유 수명 관계를 유지하며, 길이에 따라 두 입력 중 하나를 빌려 반환한다.
#[cfg(feature = "learner-practice")]
pub fn choose_longer<'a>(_left: &'a str, _right: &'a str) -> Result<&'a str, &'static str> {
    Err("TODO: 수명 관계를 표현하고 문자열을 선택하세요")
}

#[cfg(all(test, feature = "learner-practice"))]
mod tests {
    #[test]
    #[ignore = "학습자가 choose_longer를 구현한 뒤 실행하세요"]
    fn choose_longer_returns_left_when_left_is_longer() {
        // Given
        let left = String::from("considerably longer");
        let right = String::from("short");

        // When
        let selected = super::choose_longer(&left, &right);

        // Then
        assert_eq!(selected, Ok(left.as_str()));
    }

    #[test]
    #[ignore = "학습자가 choose_longer를 구현한 뒤 실행하세요"]
    fn choose_longer_returns_right_when_right_is_longer() {
        // Given
        let left = String::from("short");
        let right = String::from("considerably longer");

        // When
        let selected = super::choose_longer(&left, &right);

        // Then
        assert_eq!(selected, Ok(right.as_str()));
    }

    #[test]
    #[ignore = "학습자가 choose_longer를 구현한 뒤 실행하세요"]
    fn choose_longer_returns_left_when_lengths_are_equal() {
        // Given
        let left = String::from("left");
        let right = String::from("same");

        // When
        let selected = super::choose_longer(&left, &right);

        // Then
        assert_eq!(selected, Ok(left.as_str()));
    }
}
