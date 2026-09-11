pub const PRACTICE_SENTINEL: u32 = 0;

pub const fn minutes_in_days(_days: u32) -> u32 {
    PRACTICE_SENTINEL
}

#[cfg(test)]
mod tests {
    use super::minutes_in_days;

    #[test]
    #[ignore = "학습자가 minutes_in_days를 구현한 뒤 실행하세요"]
    fn practice_contract_returns_minutes_when_days_are_given() {
        // Given
        let days = 2;

        // When
        let minutes = minutes_in_days(days);

        // Then
        assert_eq!(minutes, 2_880);
    }
}
