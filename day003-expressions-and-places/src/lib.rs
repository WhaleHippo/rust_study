/// 블록의 마지막 식을 함수의 값으로 사용한다.
#[must_use]
pub const fn block_result(base: i32) -> i32 {
    let adjustment = 2;
    base + adjustment
}

/// 튜플의 두 번째 필드라는 place에 첫 번째 필드의 값을 대입한다.
pub const fn update_second(pair: &mut (i32, i32)) {
    pair.1 = pair.0;
}

#[cfg(feature = "learner-practice")]
pub mod practice;

#[cfg(test)]
mod tests {
    #[test]
    fn block_result_uses_its_final_expression_when_given_a_base() {
        // Given
        let base = 40;

        // When
        let result = super::block_result(base);

        // Then
        assert_eq!(result, 42);
    }

    #[test]
    fn update_second_mutates_the_tuple_field_place_when_given_a_pair() {
        // Given
        let mut pair = (42, 0);

        // When
        super::update_second(&mut pair);

        // Then
        assert_eq!(pair, (42, 42));
    }
}
