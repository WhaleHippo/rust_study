// `const`는 프로그램 전체에서 같은 값이며, 컴파일 시 계산되어 시간당 초 수를 분명한 이름으로 남긴다.
const HOURS_IN_SECONDS: u32 = 60 * 60;

// 입력과 반환값이 모두 수치이므로, 호출 위치에서 계산 결과를 예측할 수 있는 순수한 변환이다.
// 영하 입력도 `+ 32.0` 오프셋을 거쳐야 `-10.0`이 화씨 `14.0`이 된다.
const fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// 표현식의 값을 반환하는 함수: `if`와 `loop`에서도 같은 방식으로 값을 만들 수 있다.
const fn plus_one(value: i32) -> i32 {
    value + 1
}

fn main() {
    println!("Chapter 03: Common Programming Concepts");

    // 불변 바인딩 `language`는 재대입할 수 없고, 이후 같은 이름의 `let`은 별도 바인딩을 만든다.
    let language = "Rust";
    // `mut`는 이 바인딩에만 변경 권한을 부여하므로 `+= 1` 뒤 출력값은 4가 된다.
    let mut lessons = 3;
    lessons += 1;
    // 섀도잉은 문자열을 지우는 재대입이 아니라 이름을 `usize` 길이 값에 다시 연결한다.
    let language = language.len();
    // 스칼라는 각각 부호 있는 정수, 접미사로 명시한 부동소수점, 유니코드 문자, 불리언을 나타낸다.
    let signed: i8 = -8;
    let decimal = 2.5_f32;
    let initial = 'R';
    let truth = true;
    // 튜플은 서로 다른 타입을 위치로 묶고, 배열은 같은 타입의 고정 길이 요소를 보관한다.
    let pair = (language, signed);
    let chapters = [3, 4, 5, 6];

    // `if`의 두 분기는 모두 `i32`를 만들어야 하며, `truth`가 참이므로 `plus_one(4)`의 5가 선택된다.
    let conditional = if truth { plus_one(4) } else { 0 };
    // `loop`는 `break 값`으로 반복문 자체의 값을 반환한다. 두 번째 반복에서 20을 반환하며 끝난다.
    let mut loop_count = 0;
    let loop_result = loop {
        loop_count += 1;
        if loop_count == 2 {
            break loop_count * 10;
        }
    };
    // `while`은 조건이 참인 동안만 실행한다. 인덱스가 배열 길이에 닿으면 멈춰 4가 된다.
    let mut while_index = 0;
    while while_index < chapters.len() {
        while_index += 1;
    }
    // 배열을 값으로 순회해 각 장 번호를 더하므로, 네 요소의 합인 18이 출력된다.
    let mut for_total = 0;
    for chapter in chapters {
        for_total += chapter;
    }

    // 비교 결과에 따라 같은 타입인 문자열 리터럴 중 하나를 고르는 조건식이다.
    let cold_celsius = -10.0;
    let temperature_description = if cold_celsius < 0.0 {
        "below freezing"
    } else {
        "at or above freezing"
    };

    println!("bindings: lessons={lessons}, shadowed_language_length={language}");
    println!("scalars: {signed}, {decimal}, {initial}, {truth}");
    println!("tuple_first={}, array_count={while_index}", pair.0);
    println!("control_flow: if={conditional}, loop={loop_result}, for={for_total}");
    println!("constant_seconds_per_hour={HOURS_IN_SECONDS}");
    println!("0C is {}F", celsius_to_fahrenheit(0.0));
    println!(
        "negative temperature: {cold_celsius}C is {}F ({temperature_description})",
        celsius_to_fahrenheit(cold_celsius)
    );
}

#[cfg(test)]
mod tests {
    use super::celsius_to_fahrenheit;

    #[test]
    fn converts_freezing_point_when_celsius_is_zero() {
        // 준비: 섭씨 물의 어는점은 변환식의 기준값이다.
        let celsius = 0.0;
        // 실행: 섭씨 값을 화씨로 변환한다.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // 검증: 0도에는 상수항 32만 남아 화씨 32도가 된다.
        assert_eq!(fahrenheit, 32.0);
    }

    #[test]
    fn converts_boiling_point_when_celsius_is_one_hundred() {
        // 준비: 섭씨 물의 끓는점은 비율 계산을 확인하는 대표값이다.
        let celsius = 100.0;
        // 실행: 같은 변환식을 적용한다.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // 검증: 100 * 9 / 5 + 32가 화씨 212가 됨을 보장한다.
        assert_eq!(fahrenheit, 212.0);
    }

    #[test]
    fn preserves_shared_scale_point_when_celsius_is_negative_forty() {
        // 준비: -40은 섭씨와 화씨 눈금이 만나는 경계 사례다.
        let celsius = -40.0;
        // 실행: 음수에도 동일한 산술식을 적용한다.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // 검증: 두 눈금의 공통값인 -40을 그대로 반환한다.
        assert_eq!(fahrenheit, -40.0);
    }

    // [학습 실습: C03-01]
    // 목표: 음수 섭씨 변환에서 `-10.0`이 `14.0`이 되는 경계 계약을 고정한다.
    // 학습자 행동: 로컬에서 `+ 32.0`을 임시 제거한 뒤 화씨 오프셋을 직접 다시 구현한다.
    // RED: `cargo test -p chapter03-common-programming-concepts converts_negative_temperature_when_celsius_is_below_zero`가 `14.0` 대신 `-18.0`을 관찰해 실패한다.
    // GREEN: `+ 32.0`을 복구하면 이 테스트와 장 전체 테스트가 통과한다.
    // 힌트: `celsius_to_fahrenheit`에서 곱셈과 나눗셈 뒤의 화씨 오프셋을 확인한다.
    #[test]
    fn converts_negative_temperature_when_celsius_is_below_zero() {
        // 준비: -10은 공통점이 아닌 영하 입력 경계다.
        let celsius = -10.0;
        // 실행: 부호가 있는 부동소수점 계산을 수행한다.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // 검증: 영하에서도 분기 없이 같은 식으로 화씨 14가 나온다.
        assert_eq!(fahrenheit, 14.0);
    }
}
