# Day 007: 함수와 식

## 학습 목표

함수 본문의 꼬리 식이 반환값이 되는 원리, 세미콜론이 값을 `()`로 바꾸는 효과, 명시적 `return`, 함수 항목과 함수 포인터의 기초를 익힌다.

## 선수 지식과 돌아보기

- 선수 학습: [Day 006 핵심 타입](../day006-core-types/README.md)
- 전체 지도: [Rust 개념 지도](../README.md)
- 진도표: [`day007-function-expressions`](../topics.md#2-문법-바인딩-타입-함수-제어-흐름)

## 핵심 개념

- 블록의 마지막 expression에 세미콜론이 없으면 그 값이 블록의 값이다.
- expression 뒤의 세미콜론은 값을 버리고 unit 타입 `()`를 만든다.
- `return value;`는 함수 중간에서 즉시 반환해야 할 때 유용하다. 마지막 값에는 보통 꼬리 식이 더 간결하다.
- `double` 같은 함수 이름은 고유한 함수 항목 타입을 가진다. 필요한 자리에서 `fn(i32) -> i32` 함수 포인터로 강제 변환될 수 있다.
- 함수 포인터는 환경을 캡처하지 않는다. 캡처하는 클로저는 뒤의 별도 학습 주제다.

아래 함수는 세미콜론 때문에 본문 값이 `()`가 되어 컴파일되지 않는다.

```compile_fail
fn answer() -> i32 {
    42;
}
```

## 예제 관찰

```bash
cargo run --manifest-path day007-function-expressions/Cargo.toml --example example
```

꼬리 식은 `42`, 세미콜론으로 버린 결과는 `()`, 음수 입력의 명시적 조기 반환은 `0`, 함수 포인터 호출은 `42`로 출력된다.

## 흔한 오해

세미콜론은 단순한 줄 끝 기호가 아니다. 값이 필요한 위치에서 세미콜론을 추가하면 타입이 `()`로 바뀐다. 반대로 모든 함수 끝에 `return`을 적을 필요는 없다.

## 퀴즈

1. `{ 40 + 2 }`와 `{ 40 + 2; }`의 타입은 각각 무엇인가?
2. 명시적 `return`이 꼬리 식보다 자연스러운 경우는 언제인가?
3. 함수 항목이 `fn(i32) -> i32` 인수로 전달될 수 있는 이유는 무엇인가?

## 연습 명령과 완료 기준

`src/practice.rs`의 `answer_from_tail_expression`에서 sentinel을 제거하고 블록의 꼬리 식으로 `42`를 반환하라. 마지막 expression에 세미콜론을 붙이지 않는다.

```bash
cargo test --manifest-path day007-function-expressions/Cargo.toml --features learner-practice -- --ignored
```

연습 계약이 통과하고 함수의 반환 타입이 `i32`로 유지되면 완료다.

## 공식 링크

- [The Rust Programming Language: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Reference: Expressions](https://doc.rust-lang.org/reference/expressions.html)
- [Rust Reference: Function pointer types](https://doc.rust-lang.org/reference/types/function-pointer.html)
