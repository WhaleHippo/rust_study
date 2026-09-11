# Day 010: borrowing과 reborrowing

## 목표

공유 빌림 `&T`, 가변 빌림 `&mut T`, 가변 참조에서 만드는 짧은 재빌림의 유효 구간을 그리고, non-lexical lifetimes(NLL)가 마지막 사용을 기준으로 빌림을 끝내는 사례를 읽는다.

## 선수 지식과 돌아가기

- 선수 지식: [Day 009: move, Copy, Clone](../day009-move-copy-clone/README.md)
- [전체 개념 지도](../README.md)
- [학습 주제 목록의 day010](../topics.md#3-소유권-빌림-슬라이스-수명-소멸)

## 핵심 설명

공유 참조 `&T`는 값을 읽는 권한을 빌린다. 활성 구간이 겹치는 공유 참조는 여러 개 만들 수 있다. 가변 참조 `&mut T`는 해당 구간의 배타적 접근 권한이므로 충돌하는 다른 참조와 동시에 활성일 수 없다.

`&mut T`를 함수에 넘길 때 `&mut *reference`처럼 더 짧은 가변 참조를 만들 수 있다. 이것이 **재빌림(reborrow)** 이다. 짧은 재빌림의 마지막 사용이 끝나면 원래 가변 참조를 다시 사용할 수 있다. `increment_twice`가 이 순서를 실행한다.

NLL은 참조의 유효 구간을 무조건 중괄호 끝까지 잡지 않고 마지막 사용까지로 줄인다. `read_then_append`에서는 공유 재빌림으로 길이를 읽은 뒤 그 참조를 다시 쓰지 않으므로 같은 `String`을 이어서 변경할 수 있다.

아래 충돌 사례는 학습용 Markdown이며 빌드 target에는 포함되지 않는다.

```rust,compile_fail
let mut text = String::from("rust");
let shared = &text;
let exclusive = &mut text; // error: 활성 공유 빌림과 가변 빌림이 겹침
println!("{shared} {exclusive}");
```

```rust,compile_fail
let mut number = 1;
let first = &mut number;
let second = &mut number; // error: 활성 가변 빌림은 하나만 가능
*first += 1;
*second += 1;
```

## 예제 실행과 관찰

```bash
cargo run --manifest-path day010-borrowing-and-reborrowing/Cargo.toml --example example
```

두 공유 빌림은 소유권을 가져가지 않는다. 가변 참조의 짧은 재빌림 두 번은 순서대로 끝나 40을 42로 바꾼다. NLL 예제는 공유 참조의 마지막 사용 뒤 `!`를 붙인다.

## 흔한 오해

빌림이 항상 lexical scope의 닫는 중괄호까지 유지되는 것은 아니다. 반대로 NLL이 참조의 실제 수명을 늘리거나 충돌하는 접근을 허용하는 것도 아니다. 마지막 사용을 더 정확히 찾아 허용 구간을 좁힐 뿐이다.

## 확인 퀴즈

1. 공유 참조 두 개가 동시에 활성일 수 있는 이유는 무엇인가?
2. `&mut *reference`로 만든 재빌림이 끝난 뒤 무엇을 다시 사용할 수 있는가?
3. NLL에서 빌림의 끝을 판단하는 중요한 지점은 어디인가?

## 연습 문제와 채점 기준

`src/practice.rs`는 `learner-practice` feature 전용이다. `learner_increment_twice`에서 같은 `&mut i32`로부터 겹치지 않는 짧은 재빌림을 두 번 만들어 값을 각각 1씩 증가시킨다. 아래 ignored 테스트의 결과가 42이면 완료다.

```bash
cargo test --manifest-path day010-borrowing-and-reborrowing/Cargo.toml \
  --features learner-practice -- --ignored
```

starter sentinel은 아무것도 변경하지 않으므로 명시적으로 선택한 연습 테스트만 실패한다. 일반 `cargo test --all-features` 기준선은 통과한다.

## 공식 자료

- [The Rust Programming Language: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [The Rust Reference: Borrow expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators)
- [The Rust Reference: Place expressions and value contexts](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-contexts)
