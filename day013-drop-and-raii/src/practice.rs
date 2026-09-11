//! `learner-practice` 기능을 켰을 때만 포함되는 연습 시작점이다.

/// TODO: RAII guard의 소멸 순서를 관찰하도록 완성한다.
#[cfg(feature = "learner-practice")]
pub fn observe_drop_order() -> Result<Vec<&'static str>, &'static str> {
    Err("TODO: Drop으로 정리 순서를 기록하세요")
}

#[cfg(all(test, feature = "learner-practice"))]
mod tests {
    #[test]
    #[ignore = "학습자가 observe_drop_order를 구현한 뒤 실행하세요"]
    fn observe_drop_order_reports_reverse_local_order_when_implemented() {
        // Given
        let expected = vec!["second", "first"];

        // When
        let observed = super::observe_drop_order();

        // Then
        assert_eq!(observed, Ok(expected));
    }
}
