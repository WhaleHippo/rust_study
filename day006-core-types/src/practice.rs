pub const PRACTICE_SENTINEL: Option<i32> = None;

pub fn safe_last(_values: &[i32]) -> Option<i32> {
    PRACTICE_SENTINEL
}

#[cfg(test)]
mod tests {
    use super::safe_last;

    #[test]
    #[ignore = "학습자가 safe_last를 구현한 뒤 실행하세요"]
    fn practice_contract_returns_last_value_when_array_is_not_empty() {
        // Given
        let values = [10, 20, 30];

        // When
        let last = safe_last(&values);

        // Then
        assert_eq!(last, Some(30));
    }

    #[test]
    #[ignore = "학습자가 safe_last를 구현한 뒤 실행하세요"]
    fn practice_contract_returns_the_only_value_when_slice_has_one_element() {
        // Given
        let values = [42];

        // When
        let last = safe_last(&values);

        // Then
        assert_eq!(last, Some(42));
    }

    #[test]
    #[ignore = "학습자가 safe_last를 구현한 뒤 실행하세요"]
    fn practice_contract_returns_none_when_slice_is_empty() {
        // Given
        let values: [i32; 0] = [];

        // When
        let last = safe_last(&values);

        // Then
        assert_eq!(last, None);
    }
}
