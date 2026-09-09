fn add_two(value: i32) -> i32 {
    // 단위 테스트가 입력과 기대값을 간결하게 비교할 수 있도록 만든 순수 함수 예제다.
    value + 2
}

// 생성 시점에 값의 범위를 강제하는 타입이다. 유효하지 않은 상태를 만들지 않는 것이 핵심이다.
struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Self {
        // `assert!`는 조건이 거짓이면 즉시 패닉한다. 뒤의 형식 문자열은 실패 원인을 알려 준다.
        assert!(
            value <= 100,
            "Guess value must be less than or equal to 100, got {value}."
        );
        Self { value }
    }

    fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Chapter 11: Writing Automated Tests");
    let guess = Guess::new(42);
    println!("assert_eq!: add_two(40) = {}", add_two(40));
    println!(
        "boundary invariant: Guess::new({}) accepts values through 100.",
        guess.value()
    );
}

// `cargo test`일 때만 이 모듈을 컴파일한다. 일반 `cargo run` 바이너리에는 테스트 코드가 포함되지 않는다.
#[cfg(test)]
mod tests {
    // `super`로 상위 모듈의 비공개 항목도 시험할 수 있어 구현 단위의 동작을 직접 검증한다.
    use super::{Guess, add_two};

    // `#[test]`가 붙은 인자 없는 함수는 테스트 러너가 발견하여 실행한다.
    #[test]
    fn add_two_when_given_two() {
        // Given: the number two.
        // When: two is added.
        let result = add_two(2);
        // Then: the result is four.
        // `assert_eq!`는 두 값이 같아야 통과하며, 세 번째 인자는 실패 시에만 표시할 설명이다.
        assert_eq!(result, 4, "add_two should increase the input by two");
        // `assert_ne!`는 반대로 두 값이 달라야 통과한다. 두 매크로 모두 실패하면 현재 값을 보여 준다.
        assert_ne!(result, 2, "the result must differ from the original input");
    }

    #[test]
    fn add_two_when_given_a_negative_value() {
        // Given: a negative integer.
        // When: two is added.
        let result = add_two(-4);
        // Then: the arithmetic remains correct across zero.
        assert_eq!(result, -2);
    }

    #[test]
    // 이 테스트는 패닉이 발생해야 통과한다. `expected`는 패닉 메시지 전체가 아니라 포함 문자열을 검사한다.
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn guess_new_when_value_is_above_the_allowed_range() {
        // Given: a value outside the lesson's allowed range.
        // When: a Guess is constructed.
        let _guess = Guess::new(101);
        // Then: the constructor panics with its documented message.
    }

    #[test]
    // 테스트 함수도 `Result`를 반환할 수 있다. `Ok(())`는 통과, `Err`는 그 문자열을 실패 이유로 보고한다.
    fn result_returning_test_when_guess_and_add_two_are_valid() -> Result<(), String> {
        // Given: a valid guess and an input near the expected sum.
        let guess = Guess::new(42);
        // When: the lesson values are read and calculated.
        let total = add_two(40);
        // Then: both outcomes can be checked while returning Result.
        if guess.value() != 42 {
            return Err("Guess constructor did not preserve its valid value".to_owned());
        }
        if total != 42 {
            return Err("add_two did not produce the expected total".to_owned());
        }
        Ok(())
    }

    #[test]
    fn guess_new_when_value_is_upper_boundary() {
        // Given: the largest permitted guess.
        // 100은 `<= 100` 조건을 만족하는 포함 경계이고, 위의 101은 바로 다음 값이라 패닉을 검증한다.
        // When: the boundary value is constructed.
        let guess = Guess::new(100);
        // Then: the constructor preserves the valid boundary.
        assert_eq!(guess.value(), 100);
    }
}
