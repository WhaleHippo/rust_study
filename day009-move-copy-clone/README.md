# Day 009: move, Copy, Clone

## 목표

대입과 함수 호출에서 값이 **이동(move)** 하는 경우, 암시적으로 `Copy` 되는 경우, 명시적으로 `Clone` 하는 경우를 구분하고 이후 사용할 수 있는 바인딩을 예측한다.

## 선수 지식과 돌아가기

- 선수 지식: 바인딩, 타입, 함수 호출 (`day005`~`day008` 범위)
- [전체 개념 지도](../README.md)
- [학습 주제 목록의 day009](../topics.md#3-소유권-빌림-슬라이스-수명-소멸)

## 핵심 설명

`String`처럼 `Copy`가 아닌 값을 다른 바인딩에 대입하거나 값 인수로 넘기면 소유권이 이동한다. 이전 바인딩은 더 이상 사용할 수 없고 새 소유자가 값을 정리한다. `u32`처럼 `Copy`인 타입은 대입 시 값이 암시적으로 복사되므로 양쪽 바인딩을 계속 쓸 수 있다.

`Clone::clone`은 명시적인 복제 연산이다. 복제 비용과 의미는 타입의 구현이 정한다. `String::clone`은 별도의 문자열 버퍼를 만들지만, **Clone이 언제나 깊은 복사를 뜻하지는 않는다.** 예를 들어 `Rc::clone`은 같은 할당을 가리키는 핸들을 만들고 참조 횟수를 늘린다. 모든 `Copy` 타입은 `Clone`도 구현하지만 모든 `Clone` 타입이 `Copy`인 것은 아니다.

컴파일되지 않는 소유권 사례는 실행 target에 넣지 않고 여기에서 관찰한다.

```rust,compile_fail
let first = String::from("rust");
let second = first;
println!("{first}"); // error: moved value를 다시 사용
println!("{second}");
```

## 예제 실행과 관찰

```bash
cargo run --manifest-path day009-move-copy-clone/Cargo.toml --example example
```

출력에서 이동 후에는 새 소유자가 문자열을 갖고, `u32` 원본은 복사 뒤에도 사용된다. 독립적인 `String` 복제본에 `!`를 붙여도 원본은 `clone`으로 남는다. 반대로 두 `Rc`는 같은 할당을 가리키며 강한 참조 수는 2가 된다.

## 흔한 오해

`clone()`은 borrow checker 오류를 없애는 만능 수단이 아니다. 단순히 읽기만 한다면 소유 값을 복제하기보다 `&str`이나 `&T`를 빌리는 API가 보통 의도를 더 정확히 나타낸다.

## 확인 퀴즈

1. `let b = a;` 뒤에도 `a`를 쓸 수 있는지는 무엇이 결정하는가?
2. `String::clone`과 `Rc::clone`의 메모리 공유 방식은 어떻게 다른가?
3. 함수가 문자열을 읽기만 할 때 `String` 대신 `&str`을 받는 이유는 무엇인가?

## 연습 문제와 채점 기준

`src/practice.rs`는 `learner-practice` feature를 켤 때만 컴파일된다. `learner_append_owned`가 입력 `String`의 소유권을 받아 `"acean"`을 붙인 값을 반환하도록 수정한다. 새 문자열을 불필요하게 복제하지 않고 아래 테스트 하나가 통과하면 완료다.

```bash
cargo test --manifest-path day009-move-copy-clone/Cargo.toml \
  --features learner-practice -- --ignored
```

starter sentinel은 입력을 그대로 반환하므로 명시적으로 ignored 테스트를 선택하면 실패한다. 일반 `cargo test --all-features`에서는 학습 중인 테스트가 무시되어 기준선이 유지된다.

## 공식 자료

- [The Rust Programming Language: Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Reference: Copy types](https://doc.rust-lang.org/reference/special-types-and-traits.html#copy)
- [`std::clone::Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [`std::marker::Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
