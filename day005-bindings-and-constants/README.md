# Day 005: 바인딩과 상수

## 학습 목표

기본 불변 바인딩, `mut`로 바꾸는 값, shadowing으로 새로 만드는 바인딩, `const`, 불변 `static`을 구분한다.

## 선수 지식과 돌아보기

- 선수 지식: crate와 expression의 기초
- 전체 지도: [Rust 개념 지도](../README.md)
- 진도표: [`day005-bindings-and-constants`](../topics.md#2-문법-바인딩-타입-함수-제어-흐름)

## 핵심 개념

- `let value = 3;`은 기본적으로 불변이다.
- `let mut value = 3;`은 같은 바인딩의 값을 바꿀 수 있게 한다.
- `let value = value.to_string();`은 값을 변경하는 것이 아니라 같은 이름의 새 바인딩을 만든다. 그래서 타입도 달라질 수 있다.
- `const`는 타입을 명시하며 상수 문맥에서 계산된다.
- 불변 `static`은 프로그램 전체 수명의 저장 위치를 제공한다. 이 학습에서는 `static mut`을 사용하지 않는다.

아래 코드는 컴파일되지 않는다. 불변 바인딩에는 다시 대입할 수 없다.

```compile_fail
let count = 1;
count = 2;
```

## 예제 관찰

```bash
cargo run --manifest-path day005-bindings-and-constants/Cargo.toml --example example
```

첫 줄에서 불변 값은 `3`으로 유지되고, `mut` 값은 `5`가 되며, shadowing 뒤에는 문자열 대신 길이 `4`가 남는다. 둘째 줄에서는 `const`와 불변 `static`을 관찰한다.

## 흔한 오해

`mut`와 shadowing은 같지 않다. `mut`는 같은 바인딩의 값을 바꾸고, shadowing은 새 바인딩을 만들므로 타입을 바꿀 수 있다. `const`와 `static`도 모두 오래 쓰는 값처럼 보이지만, `static`은 고정된 저장 위치와 `'static` 수명을 가진다.

## 퀴즈

1. `mut` 없이 값을 두 번 바인딩할 수 있는 이유는 무엇인가?
2. `const` 선언에서 타입 표기를 생략할 수 있는가?
3. 읽기 전용 전역 문자열에 `static mut`이 필요하지 않은 이유는 무엇인가?

## 연습 명령과 완료 기준

`src/practice.rs`의 `minutes_in_days`를 구현한다. `const`를 재사용하고 shadowing 또는 불변 지역 바인딩으로 계산 단계를 표현하라. sentinel `0`을 실제 계산으로 바꾼 뒤 다음 명령을 실행한다.

```bash
cargo test --manifest-path day005-bindings-and-constants/Cargo.toml --features learner-practice -- --ignored
```

테스트가 `2일 == 2,880분`으로 통과하고, 기본 `cargo test`도 계속 통과하면 완료다.

## 공식 링크

- [The Rust Programming Language: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Reference: Static items](https://doc.rust-lang.org/reference/items/static-items.html)
- [Rust Reference: Constant items](https://doc.rust-lang.org/reference/items/constant-items.html)
