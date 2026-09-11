# Day 006: 핵심 타입

## 학습 목표

정수, 부동소수점, `bool`, `char` 같은 스칼라 타입과 배열, 튜플, 배타 및 포괄 범위의 차이를 설명하고 배열 경계를 안전하게 다룬다.

## 선수 지식과 돌아보기

- 선수 학습: [Day 005 바인딩과 상수](../day005-bindings-and-constants/README.md)
- 전체 지도: [Rust 개념 지도](../README.md)
- 진도표: [`day006-core-types`](../topics.md#2-문법-바인딩-타입-함수-제어-흐름)

## 핵심 개념

- 스칼라는 하나의 값을 나타낸다. `char`는 4바이트 Unicode scalar value이며 화면의 글자 한 덩어리와 항상 같지는 않다.
- 배열 `[T; N]`은 같은 타입 `T`를 고정 길이 `N`만큼 연속으로 저장하며 길이도 타입의 일부다.
- 튜플은 서로 다른 타입을 정해진 순서로 묶고 `.0`, `.1`처럼 위치로 접근한다.
- `start..end`는 끝을 제외하고, `start..=end`는 끝을 포함한다.
- `values[index]`는 경계를 벗어나면 panic이지만 `values.get(index)`는 `Option`으로 안전하게 부재를 돌려준다.

아래 코드는 배열 길이가 타입의 일부라서 컴파일되지 않는다.

```compile_fail
let three: [u8; 3] = [1, 2, 3, 4];
```

## 예제 관찰

```bash
cargo run --manifest-path day006-core-types/Cargo.toml --example example
```

배열과 튜플의 모양, `1..4`의 합 `6`과 `1..=4`의 합 `10`, 배열 길이와 같은 인덱스에서 `None`이 반환되는 모습을 확인한다.

## 흔한 오해

Rust가 숫자 타입을 언제나 자동으로 넓혀 주지는 않는다. 또한 배열 인덱싱은 안전한 Rust이지만 경계를 벗어나면 panic한다. 메모리 안전과 panic 없는 동작은 서로 다른 보장이다.

## 퀴즈

1. `[u8; 3]`과 `[u8; 4]`는 같은 타입인가?
2. `0..3`과 `0..=3`은 각각 몇 값을 만드는가?
3. 알 수 없는 인덱스를 받을 때 `get`이 직접 인덱싱보다 나은 이유는 무엇인가?

## 연습 명령과 완료 기준

`src/practice.rs`의 `safe_last`를 구현한다. 빈 슬라이스에서도 panic하지 않고, 값이 있으면 마지막 원소를 `Some`으로 반환해야 한다.

```bash
cargo test --manifest-path day006-core-types/Cargo.toml --features learner-practice -- --ignored
```

연습 계약과 기본 테스트가 모두 통과하고 직접 인덱싱으로 경계 panic을 만들지 않으면 완료다.

## 공식 링크

- [The Rust Programming Language: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [표준 라이브러리의 primitive array](https://doc.rust-lang.org/std/primitive.array.html)
- [표준 라이브러리의 Range](https://doc.rust-lang.org/std/ops/struct.Range.html)
- [표준 라이브러리의 RangeInclusive](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html)
