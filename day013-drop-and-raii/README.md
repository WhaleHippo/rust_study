# Day 013: Drop과 RAII

## 학습 목표

- 구조체의 일부 필드를 이동한 뒤 사용할 수 있는 부분과 사용할 수 없는 전체 값을 구분한다.
- 지역 변수와 구조체 필드의 결정적인 소멸 순서를 설명한다.
- RAII가 자원 획득과 해제를 값의 수명에 묶는 방식을 이해한다.
- `std::mem::drop`으로 소유 값을 블록 끝보다 일찍 소멸시킨다.

## 부분 이동

`partial_move`는 `Profile.name`을 꺼내 소유권을 이동한 뒤 `Profile.skills`를 빌려 길이를 읽는다. 이 시점부터 `profile` 전체는 사용할 수 없지만 이동되지 않은 필드는 각각 사용할 수 있다.

부분 이동을 보여 주는 `Profile`은 의도적으로 `Drop`을 구현하지 않는다. `Drop`을 구현한 타입에서 필드 하나만 밖으로 이동하면 destructor가 완전한 `self`를 받아야 한다는 계약을 깰 수 있어 컴파일러가 거부한다. 필드별 소유권 이전이 필요하다면 타입 설계를 나누거나 `Option::take` 같은 명시적 상태 전이를 고려한다.

## drop scope와 순서

값마다 정해진 drop scope가 있다. 이 패키지의 기록 함수는 실행 순서를 문자열 벡터에 남겨 규칙을 결정적으로 확인한다.

- 같은 블록의 지역 변수는 선언의 **역순**으로 소멸한다: `second`, `first`.
- 구조체의 필드는 구조체 선언에 적힌 **순서**로 소멸한다: `first`, `second`.
- `std::mem::drop(value)`은 값을 소비해 해당 위치에서 일찍 소멸시킨다.

`Drop::drop` 메서드는 직접 호출하지 않는다. 조기 정리가 필요하면 표준 함수 `std::mem::drop`을 사용한다.

Rust 2024에서는 일부 임시 값의 범위가 더 좁아졌다. 특히 `if let` 조건의 임시 값은 `else`에 들어가기 전에 소멸할 수 있고, 블록의 tail expression에서 만들어진 임시 값은 블록의 지역 변수보다 먼저 소멸할 수 있다. 지역 변수 규칙만 외워 모든 임시 값의 시점을 추측하지 말고, expression 종류와 edition의 temporary scope 규칙을 함께 확인해야 한다.

## RAII와 Drop

RAII(Resource Acquisition Is Initialization)는 자원 획득을 값 생성과 묶고, 값이 scope를 벗어날 때 `Drop`으로 정리한다. `DropRecorder`는 `Rc<RefCell<Vec<&'static str>>>`를 안전한 단일 스레드 기록 장치로 공유한다. 각 recorder가 소멸되면 이름을 기록하므로 파일이나 잠금 guard가 scope 종료 시 정리되는 원리를 외부 자원 없이 관찰할 수 있다.

`Drop`은 정상적인 scope 종료와 조기 반환에서 결정적 정리를 제공한다. panic이 unwind되는 구성에서는 이미 생성된 값도 정리되지만, 프로세스 종료나 `panic = "abort"`에서는 destructor 실행을 보장할 수 없다. 반드시 수행되어야 하는 외부 영속 작업을 `Drop` 하나에만 의존해서는 안 된다.

## 실행 방법

```bash
cargo test
cargo run --example example
```

예제 출력은 다음과 같다.

```text
부분 이동: Ferris, 기술 2개
지역 변수: ["second", "first"]
구조체 필드: ["first", "second"]
조기 해제: ["early", "later"]
```

## 연습

`src/practice.rs`의 연습 함수와 테스트는 기본 빌드에서 제외된다. 아래 명령으로 기능을 켜고 무시된 연습 테스트를 실행한다. 시작 상태에서는 안전한 TODO sentinel이 `Err`를 반환하므로 의도적으로 실패한다.

```bash
cargo test --features learner-practice -- --ignored
```

지역 값 두 개가 역순으로 소멸한 기록을 반환하도록 `observe_drop_order`를 완성해 보자.

## 공식 참고 자료

- [The Rust Programming Language: Running Code on Cleanup with the Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [The Rust Reference: Destructors](https://doc.rust-lang.org/reference/destructors.html)
- [The Rust Reference: Destructuring](https://doc.rust-lang.org/reference/patterns.html#destructuring)
- [Edition Guide: `if let` temporary scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html)
- [Edition Guide: Tail expression temporary scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-tail-expr-scope.html)
- [`std::mem::drop`](https://doc.rust-lang.org/std/mem/fn.drop.html)
