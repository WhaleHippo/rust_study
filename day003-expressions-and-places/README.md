# Day 003: statement, expression, place, value

## 목표

statement와 expression을 구분하고, 값을 계산하는 식과 저장 위치를 가리키는 place expression을 코드에서 찾는다.

## 선수 지식과 돌아가기

- 선수 학습: [Day 002: 프로젝트 단위](../day002-project-units/README.md)
- 개념 지도: [식과 장소](../README.md#식과-장소), [문법, 바인딩, 타입, 함수, 제어 흐름](../README.md#2-문법-바인딩-타입-함수-제어-흐름)
- 진도표: [day003-expressions-and-places](../topics.md#1-도구-체인과-컴파일러-모델)

## 핵심 개념

- **statement**는 동작을 수행하지만 주변 식에 결과값을 제공하지 않는다. `let adjustment = 2;`는 local binding을 만드는 statement다.
- **expression**은 값을 계산한다. 리터럴, 연산, 함수 호출, 블록, `if`, `match`도 expression이다.
- 블록의 마지막 expression에 세미콜론이 없으면 그 값이 블록 전체의 값이 된다. 세미콜론을 붙이면 값을 버리고 unit 값 `()`가 된다.
- **place expression**은 값을 저장한 위치를 가리킨다. local 변수, tuple field, 배열 원소, 역참조가 대표적이다.
- **value expression**은 사용할 값을 계산한다. 대입문 `pair.1 = pair.0;`에서 왼쪽 `pair.1`은 쓸 place이고, 오른쪽 `pair.0`은 그 place에서 읽은 값으로 사용된다.

[`src/lib.rs`](src/lib.rs)의 `block_result`는 블록의 마지막 식을 반환하고, `update_second`는 `&mut (i32, i32)`를 받아 `pair.1` place를 변경한다.

다음은 값을 계산한 결과를 대입의 왼쪽 place처럼 쓸 수 없다는 **컴파일 실패 학습 조각**이다. 실제 소스에는 넣지 않는다.

```rust,compile_fail
(1 + 2) = 3; // `1 + 2`는 저장 위치가 아니라 계산된 값이다.
```

## 예제 관찰

```console
$ cargo run --manifest-path day003-expressions-and-places/Cargo.toml --example example
block value: 42
mutated tuple: (42, 42)
```

첫 줄의 `42`는 블록 마지막 expression의 값이다. 두 번째 줄은 가변 tuple 전체를 새로 만들지 않고 두 번째 field place에 값을 대입한 결과다.

## 흔한 오해

세미콜론은 단순한 줄 끝 기호가 아니다. 값이 필요한 위치에서 마지막 expression 뒤에 세미콜론을 붙이면 블록 값이 `()`가 되어 기대한 타입과 맞지 않을 수 있다.

## 퀴즈

1. `let answer = { let base = 40; base + 2 };`에서 statement와 최종 expression은 각각 무엇인가?
2. `pair.1 = pair.0;`의 왼쪽과 오른쪽은 각각 place와 value 관점에서 어떤 역할인가?
3. `{ 42; }`의 값과 타입은 무엇인가?
4. 함수 호출 결과를 대입 왼쪽에 둘 수 없는 이유는 무엇인가?

## 실습 방법과 완료 기준

1. `cargo test --manifest-path day003-expressions-and-places/Cargo.toml --all-features`를 실행해 ignored 테스트를 확인한다.
2. [`src/practice.rs`](src/practice.rs)의 `build_matching_pair`에서 블록의 마지막 expression으로 `base + 2`를 얻는다.
3. 가변 tuple의 두 번째 field place를 첫 번째 값과 같게 만든 뒤 tuple을 반환한다.
4. 아래 명령으로 학습 계약만 명시적으로 실행한다.

```console
$ cargo test --manifest-path day003-expressions-and-places/Cargo.toml --features learner-practice -- --ignored
```

완료 기준은 ignored 계약 테스트가 통과하고, 기본 테스트와 `--all-features` Clippy가 계속 통과하며, 작성한 코드의 statement, expression, place를 직접 표시해 설명하는 것이다. `PRACTICE_SENTINEL`은 유효한 tuple 값으로 미완료 상태를 안전하게 드러내며 정답 구현은 포함하지 않는다.

## 공식 참고 자료

- [The Rust Reference: Statements and expressions](https://doc.rust-lang.org/reference/statements-and-expressions.html)
- [The Rust Reference: Expressions](https://doc.rust-lang.org/reference/expressions.html)
- [The Rust Reference: Place expressions and value expressions](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions)
- [The Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
