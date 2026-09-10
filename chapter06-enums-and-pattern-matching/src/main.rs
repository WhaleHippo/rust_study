// 각 열거형 변형은 같은 `Route` 타입 안에서 서로 다른 종류의 주소 데이터를 안전하게 표현한다.
#[derive(Debug)]
enum Route {
    V4(String),
    V6(String),
}

// 이 예제는 쿼터가 어느 주에서 발행되었는지 변형 데이터로 보관할 수 있음을 보여 준다.
#[derive(Debug)]
enum UsState {
    Alaska,
}

// 데이터 없는 변형과 `UsState`를 담는 변형을 한 타입으로 묶어 동전마다 필요한 정보만 표현한다.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// `match`는 모든 `Coin` 변형을 명시하므로, 새 변형을 추가하면 컴파일러가 이 계산의 갱신을 요구한다.
const fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

// 각 분기에서 변형에 담긴 `String`의 소유권을 꺼내 형식화된 목적지 문자열을 만든다.
fn route_destination(route: Route) -> String {
    match route {
        Route::V4(address) => format!("IPv4 {address}"),
        Route::V6(address) => format!("IPv6 {address}"),
    }
}

// `let-else`는 `Some`만 다음 코드로 통과시키고, `None`은 조기에 반환해 부재 처리를 분리한다.
// `None`의 빈 문자열은 부재를 숨기므로, 학습 테스트는 명시적인 설명을 요구한다.
fn selected_route_destination(route: Option<Route>) -> String {
    let Some(route) = route else {
        return String::from("no route selected");
    };
    route_destination(route)
}

// `Option::map`은 `Some`의 값에만 함수를 적용하고, `None`은 계산 없이 그대로 전파한다.
fn plus_one(value: Option<i32>) -> Option<i32> {
    value.map(|number| number + 1)
}

fn main() {
    println!("Chapter 06: Enums and Pattern Matching");

    // 같은 열거형 타입이지만 변형마다 주소 문자열의 의미와 출력 접두사가 달라진다.
    let home = Route::V4(String::from("127.0.0.1"));
    let loopback = Route::V6(String::from("::1"));
    println!(
        "routes: {}, {}",
        route_destination(home),
        route_destination(loopback)
    );

    // 쿼터 변형은 주 데이터를 함께 저장한다. `if let`은 관심 있는 한 변형만 간결하게 분해한다.
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("if let found a state quarter: {state:?}");
    }
    println!(
        "coin values: penny={}, nickel={}, dime={}",
        coin_value(Coin::Penny),
        coin_value(Coin::Nickel),
        coin_value(Coin::Dime)
    );

    // `Some(5)`는 6으로 변환되고, 아래 `match`는 존재와 부재 모두를 처리한다.
    let next = plus_one(Some(5));
    match next {
        Some(value) => println!("Option matched: {value}"),
        None => println!("Option matched: no value"),
    }
    println!("let else: {}", selected_route_destination(None));
}

#[cfg(test)]
mod tests {
    use super::{Coin, Route, UsState, coin_value, plus_one, selected_route_destination};

    #[test]
    fn values_state_quarter_when_coin_carries_state_data() {
        // 준비: 발행 주를 담은 쿼터 변형을 만든다.
        let coin = Coin::Quarter(UsState::Alaska);
        // 실행: 소유한 열거형을 완전한 `match`로 평가한다.
        let value = coin_value(coin);
        // 검증: 변형의 부가 데이터와 무관하게 쿼터 값은 25다.
        assert_eq!(value, 25);
    }

    #[test]
    fn returns_none_when_option_has_no_number() {
        // 준비: 값이 없는 `None` 입력이다.
        let value = None;
        // 실행: `map`은 `Some`에만 덧셈 클로저를 적용한다.
        let next = plus_one(value);
        // 검증: 클로저가 실행되지 않아 부재 상태가 그대로 유지된다.
        assert_eq!(next, None);
    }

    #[test]
    fn describes_v6_when_let_else_receives_selected_route() {
        // 준비: 선택된 IPv6 경로를 `Some`으로 감싼다.
        let route = Some(Route::V6(String::from("::1")));
        // 실행: `let-else`가 `Some`을 분해해 다음 계산으로 넘긴다.
        let destination = selected_route_destination(route);
        // 검증: 변형 데이터가 IPv6 접두사가 있는 출력으로 이어진다.
        assert_eq!(destination, "IPv6 ::1");
    }

    // [학습 실습: C06-01]
    // 목표: 선택한 경로가 없을 때 `None`을 명시적인 설명으로 바꾸는 계약을 고정한다.
    // 학습자 행동: 로컬에서 `None` 분기를 임시 빈 문자열로 바꾼 뒤 부재 설명을 직접 다시 구현한다.
    // RED: `cargo test -p chapter06-enums-and-pattern-matching describes_absence_when_no_route_is_selected`가 `"no route selected"` 대신 빈 문자열을 받아 실패한다.
    // GREEN: `None`에서 `"no route selected"`를 복구하면 이 테스트와 장 전체 테스트가 통과한다.
    // 힌트: `selected_route_destination`의 `let else`가 실패 흐름에서 반환하는 값을 확인한다.
    #[test]
    fn describes_absence_when_no_route_is_selected() {
        // 준비: 선택된 경로가 없는 `None`이다.
        let route = None;
        // 실행: 패턴 불일치가 `else`의 조기 반환을 선택한다.
        let destination = selected_route_destination(route);
        // 검증: 경로를 분해하지 않고 부재를 설명하는 문자열을 반환한다.
        assert_eq!(destination, "no route selected");
    }
}
