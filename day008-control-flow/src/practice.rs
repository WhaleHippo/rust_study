pub static PRACTICE_SENTINEL: &str = "연습 필요";

pub const fn traffic_action(_score: u8) -> &'static str {
    PRACTICE_SENTINEL
}

#[cfg(test)]
mod tests {
    use super::traffic_action;

    #[test]
    #[ignore = "학습자가 traffic_action의 분기를 완성한 뒤 실행하세요"]
    fn practice_contract_selects_go_when_score_is_high() {
        // Given
        let score = 80;

        // When
        let action = traffic_action(score);

        // Then
        assert_eq!(action, "진행");
    }

    #[test]
    #[ignore = "학습자가 traffic_action의 분기를 완성한 뒤 실행하세요"]
    fn practice_contract_selects_go_when_score_is_above_high_threshold() {
        // Given
        let score = 100;

        // When
        let action = traffic_action(score);

        // Then
        assert_eq!(action, "진행");
    }

    #[test]
    #[ignore = "학습자가 traffic_action의 분기를 완성한 뒤 실행하세요"]
    fn practice_contract_selects_caution_at_score_50() {
        // Given
        let score = 50;

        // When
        let action = traffic_action(score);

        // Then
        assert_eq!(action, "주의");
    }

    #[test]
    #[ignore = "학습자가 traffic_action의 분기를 완성한 뒤 실행하세요"]
    fn practice_contract_selects_caution_at_score_79() {
        // Given
        let score = 79;

        // When
        let action = traffic_action(score);

        // Then
        assert_eq!(action, "주의");
    }

    #[test]
    #[ignore = "학습자가 traffic_action의 분기를 완성한 뒤 실행하세요"]
    fn practice_contract_selects_stop_at_score_49() {
        // Given
        let score = 49;

        // When
        let action = traffic_action(score);

        // Then
        assert_eq!(action, "멈춤");
    }

    #[test]
    #[ignore = "학습자가 traffic_action의 분기를 완성한 뒤 실행하세요"]
    fn practice_contract_selects_stop_for_low_score() {
        // Given
        let score = 0;

        // When
        let action = traffic_action(score);

        // Then
        assert_eq!(action, "멈춤");
    }
}
