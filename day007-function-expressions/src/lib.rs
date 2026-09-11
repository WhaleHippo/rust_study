#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

pub const fn tail_expression(base: i32) -> i32 {
    let offset = 2;
    base + offset
}

const fn record_observed_value(value: i32, observed: &mut i32) -> i32 {
    *observed = value;
    value
}

pub const fn discarded_expression(base: i32, observed: &mut i32) {
    record_observed_value(tail_expression(base), observed);
}

pub const fn explicit_return(value: i32) -> i32 {
    if value < 0 {
        return 0;
    }
    value
}

pub const fn double(value: i32) -> i32 {
    value.saturating_mul(2)
}

pub fn apply(function: fn(i32) -> i32, value: i32) -> i32 {
    function(value)
}

#[cfg(test)]
mod tests {
    use super::{apply, discarded_expression, double, explicit_return, tail_expression};

    #[test]
    fn tail_expression_becomes_the_return_value() {
        // Given
        let base = 40;

        // When
        let answer = tail_expression(base);

        // Then
        assert_eq!(answer, 42);
    }

    #[test]
    fn semicolon_discards_an_expression_value_as_unit() {
        // Given
        let function: fn(i32, &mut i32) = discarded_expression;
        let mut observed = 0;

        // When
        function(40, &mut observed);

        // Then
        assert_eq!(observed, 42);
    }

    #[test]
    fn explicit_return_exits_early_for_negative_input() {
        // Given
        let input = -3;

        // When
        let actual = explicit_return(input);

        // Then
        assert_eq!(actual, 0);
    }

    #[test]
    fn function_item_coerces_to_a_function_pointer() {
        // Given
        let function: fn(i32) -> i32 = double;

        // When
        let actual = apply(function, 21);

        // Then
        assert_eq!(actual, 42);
    }
}
