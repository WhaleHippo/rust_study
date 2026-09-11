pub const PRACTICE_SENTINEL: i32 = 0;

pub const fn answer_from_tail_expression() -> i32 {
    PRACTICE_SENTINEL
}

#[cfg(test)]
mod tests {
    use super::answer_from_tail_expression;

    #[test]
    #[ignore = "학습자가 꼬리 식을 완성한 뒤 실행하세요"]
    fn practice_contract_returns_answer_from_tail_expression() {
        // Given
        let expected = 42;

        // When
        let answer = answer_from_tail_expression();

        // Then
        assert_eq!(answer, expected);
    }
}
