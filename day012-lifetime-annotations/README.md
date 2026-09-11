# Day 012: 수명 표기

## 학습 목표

- 수명 표기가 참조의 실제 생존 시간을 늘리는 장치가 아니라 참조 사이의 **관계**를 설명하는 타입 정보임을 이해한다.
- 여러 입력 중 하나를 빌려 반환할 때 명시적 수명 표기가 왜 필요한지 설명한다.
- 수명 생략 규칙과 `'static`의 의미를 구분한다.

## 핵심 개념

`longer`는 새 문자열을 만들지 않고 두 입력 중 하나를 그대로 빌려 반환한다.

```rust
pub const fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

여기서 `'a`는 "두 입력이 정확히 같은 시간 동안 산다"는 뜻이 아니다. 호출 시점에 두 입력 수명이 겹치는 공통 구간을 나타내며, 반환 참조도 그 구간을 넘어 사용할 수 없다는 관계를 표현한다. 함수 본문이 어느 입력을 반환할지 런타임에 결정하므로 출력이 두 입력 모두보다 오래 유효하다고 약속할 수 없다.

수명 표기는 데이터를 소유하지도 않고 소멸 시점을 바꾸지도 않는다. 컴파일러가 검사할 관계만 드러낸다.

## 수명 생략

입력 참조가 하나인 `fn first_word(text: &str) -> &str` 같은 시그니처는 생략 규칙으로 출력 수명을 그 입력에 연결할 수 있다. 하지만 `longer`처럼 입력 참조가 둘이고 출력이 어느 입력에서 왔는지 시그니처만으로 정할 수 없으면 관계를 직접 표기해야 한다.

## 공통 수명의 한계

다음 코드는 반환 참조를 짧게 사는 `inner`보다 오래 사용하려 하므로 컴파일되지 않는다.

```compile_fail
use day012_lifetime_annotations::longer;

let outer = String::from("오래 사는 값");
let selected;
{
    let inner = String::from("짧게 사는 값이지만 더 긴 문자열");
    selected = longer(&outer, &inner);
}
println!("{selected}");
```

함수가 실제 실행에서 `outer`를 고를 가능성이 있어도 타입 계약은 두 입력 중 어느 쪽이 반환되어도 안전해야 한다. 따라서 반환 참조는 두 입력이 함께 유효한 공통 구간으로 제한된다.

지역 값을 가리키는 참조를 반환하는 코드도 수명 이름을 붙인다고 고쳐지지 않는다.

```compile_fail
fn dangling<'a>() -> &'a str {
    let local = String::from("곧 소멸함");
    &local
}
```

`local`은 함수가 끝날 때 소멸한다. `'a`는 그 값을 연장하지 못한다.

## 왜 `'static`이 해결책이 아닌가

`&'static str`은 프로그램 전체 동안 유효한 데이터를 가리켜야 한다는 강한 요구다. 문자열 리터럴은 이를 만족하지만 지역 `String`의 참조는 만족하지 않는다. 반환 타입을 `&'static str`로 바꾸면 지역 데이터가 오래 사는 것이 아니라, 지킬 수 없는 계약을 작성해 컴파일 오류를 더 분명하게 만들 뿐이다. 소유권을 호출자에게 넘겨야 한다면 참조 대신 `String` 같은 소유 값을 반환해야 한다.

## 실행 방법

```bash
cargo test
cargo run --example example
```

예제 출력은 다음과 같다.

```text
선택: lifetime
```

## 연습

`src/practice.rs`의 연습 함수와 테스트는 기본 빌드에서 제외된다. 아래 명령으로 연습 기능을 켜고 무시된 테스트를 실행한다. 시작 상태에서는 TODO sentinel 때문에 의도적으로 실패한다.

```bash
cargo test --features learner-practice -- --ignored
```

`choose_longer`가 더 긴 입력을 빌려 반환하도록 구현한 뒤 테스트를 통과시켜 보자.

## 공식 참고 자료

- [The Rust Programming Language: Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [The Rust Reference: Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [표준 라이브러리의 primitive reference 문서](https://doc.rust-lang.org/std/primitive.reference.html)
