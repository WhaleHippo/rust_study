# Day 008: 제어 흐름

## 학습 목표

조건과 반복의 의도에 따라 `if`, 완전한 `match`, 값이 있는 `loop`와 `break`, `while`, `for`, loop label을 알맞게 고른다.

## 선수 지식과 돌아보기

- 선수 학습: [Day 007 함수와 식](../day007-function-expressions/README.md)
- 전체 지도: [Rust 개념 지도](../README.md)
- 진도표: [`day008-control-flow`](../topics.md#2-문법-바인딩-타입-함수-제어-흐름)

## 핵심 개념

- `if`는 `bool` 조건으로 가지를 선택하며 expression이므로 값을 만들 수 있다.
- `match`는 가능한 모든 경우를 처리해야 한다. 직접 정의한 enum에는 넓은 wildcard 대신 variant를 명시하면 새 variant가 생길 때 컴파일러가 알려 준다.
- `loop`는 조건 없는 반복이며 `break value`로 전체 loop expression의 값을 만들 수 있다.
- `while`은 조건이 참인 동안 반복하고, `for`는 범위나 배열처럼 반복 가능한 값을 순회한다.
- 중첩 반복의 label은 어느 loop를 `break`하거나 `continue`하는지 분명하게 한다.

아래 코드는 enum의 모든 variant를 다루지 않아 컴파일되지 않는다.

```compile_fail
enum Signal { Red, Yellow, Green }

fn action(signal: Signal) -> &'static str {
    match signal {
        Signal::Red => "멈춤",
        Signal::Green => "진행",
    }
}
```

## 예제 관찰

```bash
cargo run --manifest-path day008-control-flow/Cargo.toml --example example
```

`if`와 `while`의 단계 수, exhaustive `match` 결과, `break`가 만든 값, `for`의 합계, label로 중첩 반복을 빠져나온 첫 좌표를 차례로 확인한다.

## 흔한 오해

`if`, `match`, `loop`는 제어만 옮기는 문장이 아니라 값을 만들 수 있는 expression이다. 반면 `while`과 `for`는 반복 자체에서 `break value`를 반환하지 않는다. 여러 상태를 빠짐없이 처리해야 할 때 `if` 연쇄보다 exhaustive `match`가 안전하다.

## 퀴즈

1. enum variant가 늘었을 때 exhaustive `match`는 어떤 도움을 주는가?
2. 횟수 없는 반복에서 계산 결과를 꺼내려면 어떤 문법을 쓰는가?
3. 배열 전체를 순회할 때 `while` 인덱스보다 `for`가 자연스러운 이유는 무엇인가?
4. 중첩 loop에서 label이 없으면 어느 반복을 빠져나오는가?

## 연습 명령과 완료 기준

`src/practice.rs`의 `traffic_action`을 구현한다. `score >= 80`이면 `"진행"`, `score >= 50`이면 `"주의"`, 그 외에는 `"멈춤"`을 반환하도록 `if` expression을 사용한다.

```bash
cargo test --manifest-path day008-control-flow/Cargo.toml --features learner-practice -- --ignored
```

연습 계약과 기본 테스트가 통과하고 모든 분기가 같은 `&'static str` 타입을 반환하면 완료다.

## 공식 링크

- [The Rust Programming Language: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [The Rust Programming Language: match](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Rust Reference: Loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)
- [Rust Reference: If expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
