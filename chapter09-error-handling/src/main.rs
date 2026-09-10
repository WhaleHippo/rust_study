#[derive(Debug, PartialEq, Eq)]
// 문자열 파싱 실패를 그대로 노출하지 않고, 호출자가 match할 수 있는 도메인 오류를 타입으로 표현한다.
enum ParsePositiveError {
    Malformed,
    OutOfRange,
    NotPositive,
}

fn parse_positive(input: &str) -> Result<u32, ParsePositiveError> {
    let number = match input {
        // 빈 입력은 숫자 파서를 호출할 이유가 없으므로, 이 지점에서 즉시 의미 있는 typed error를 반환한다.
        "" => return Err(ParsePositiveError::Malformed),
        // map_err는 std의 ParseIntError를 이 함수의 ParsePositiveError로 번역한다. 숫자만으로
        // 이루어진 실패는 u32 범위 초과이고, 그 밖의 실패는 형식 오류로 구분한다.
        text => text.parse::<u32>().map_err(|_| {
            if text.bytes().all(|byte| byte.is_ascii_digit()) {
                ParsePositiveError::OutOfRange
            } else {
                ParsePositiveError::Malformed
            }
            // ?는 변환된 Err를 호출자에게 조기 반환하고, Ok(number)일 때만 다음 식에 number를 바인딩한다.
        })?,
    };
    match number {
        // 숫자 파싱 성공과 'positive' 의미 계약은 다르다. 0은 u32로 유효해도 typed error로 거절한다.
        // C09-01은 이 도메인 오류를 호출자가 match할 수 있는 `NotPositive`로 보존하는 연습이다.
        0 => Err(ParsePositiveError::NotPositive),
        positive => Ok(positive),
    }
}

fn first_positive(values: &[&str]) -> Option<u32> {
    // Result::ok은 Err의 구체적 원인(Malformed, OutOfRange, NotPositive)을 버리고 Option으로 바꾼다.
    // find_map은 왼쪽부터 이 변환을 적용해 첫 Some만 반환하므로, 실패한 항목을 건너뛰어 첫 양수를 찾는다.
    values.iter().find_map(|value| parse_positive(value).ok())
}

fn describe(value: Option<u32>) -> &'static str {
    match value {
        Some(_) => "a positive value",
        None => "no positive value",
    }
}

fn main() {
    println!("Chapter 09: Error Handling");
    println!("{}", describe(first_positive(&["0", "bad", "12"])));
}

#[cfg(test)]
mod tests {
    use super::parse_positive;

    #[test]
    fn parse_positive_when_input_is_a_positive_integer() {
        // Given: a positive integer written as text.
        // When: it is parsed.
        let value = parse_positive("7");
        // Then: parsing returns that integer.
        assert_eq!(value, Ok(7));
    }

    #[test]
    fn parse_positive_when_input_is_malformed() {
        // Given: text that is not an unsigned integer.
        // When: it is parsed.
        let value = parse_positive("seven");
        // Then: the typed malformed-input error is returned.
        assert_eq!(value, Err(super::ParsePositiveError::Malformed));
    }

    #[test]
    fn parse_positive_when_input_exceeds_u32() {
        // Given: decimal text larger than u32::MAX.
        // When: it is parsed.
        let value = parse_positive("4294967296");
        // Then: the typed out-of-range error is returned.
        assert_eq!(value, Err(super::ParsePositiveError::OutOfRange));
    }

    #[test]
    fn first_positive_when_prior_values_fail_to_parse() {
        // Given: zero and malformed values before a valid positive value.
        // When: the first positive value is selected through Option.
        let value = super::first_positive(&["0", "bad", "12"]);
        // Then: parsing continues without panicking on expected bad input.
        assert_eq!(value, Some(12));
    }

    #[test]
    fn describe_when_no_positive_value_exists() {
        // Given: the absence of a parsed positive value.
        // When: the Option is matched for display.
        let description = super::describe(None);
        // Then: the empty case has an explicit description.
        assert_eq!(description, "no positive value");
    }

    // [학습 실습: C09-01]
    // 목표: 파싱 성공과 양수라는 도메인 계약을 typed error로 구분한다.
    // 학습자 행동: `0 => Ok(0)`으로 잠시 바꾸고 아래 테스트만 실행한다.
    // RED: `cargo test -p chapter09-error-handling parse_positive_when_input_is_zero`는 `NotPositive` 기대와 `Ok(0)` 결과 차이로 실패한다.
    // GREEN: `0 => Err(ParsePositiveError::NotPositive)`로 복원해 테스트를 통과시킨다.
    // 힌트: `u32`의 유효 범위와 이 함수의 양수 계약은 서로 다르다.
    #[test]
    fn parse_positive_when_input_is_zero() {
        // 준비: 유효한 부호 없는 정수 텍스트인 0을 준비한다.
        // 실행: 이를 양수로 파싱한다.
        let value = parse_positive("0");
        // 검증: typed non-positive 오류가 반환된다.
        assert_eq!(value, Err(super::ParsePositiveError::NotPositive));
    }
}
