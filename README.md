# Rust Book 3장부터 19장까지

이 저장소는 [공식 안정판 Rust Book](https://doc.rust-lang.org/stable/book/)의 3장부터 19장까지를 문법 중심으로 익히는 자습용 예제 모음이다. 각 장은 독립 실행 파일 하나와 그 파일 안의 단위 테스트로 구성되어 있다.

## 준비

Rust 1.90 이상과 edition 2024를 사용한다. 안정판 도구 체인을 갱신하고 버전을 확인한다.

```console
rustup update stable
rustc --version
cargo --version
```

저장소 루트에서 전체 대상을 먼저 빌드한다.

```console
cargo build --all-targets
```

`src/bin/chapter03.rs` 같은 파일은 Cargo가 바이너리 대상으로 자동 발견한다. 실행 명령은 원래의 `cargo --bin` 축약이 아니라 다음과 같이 `run`을 포함해야 한다.

```console
cargo run --bin chapter03
```

실행하지 않고 컴파일만 확인하려면 다음 명령을 쓴다.

```console
cargo check --bin chapter03
```

다른 장에서는 `chapter03`을 해당 대상 이름으로 바꾸면 된다. 모든 바이너리는 인수 없이 실행되며 `Chapter NN:`으로 시작하는 결과를 출력한다.

장별 테스트 명령의 공통 형식은 다음과 같다. `chapterNN`을 표의 대상 이름으로 바꾼다.

```console
cargo test --bin chapterNN
```

## 장별 안내

| 장 | 공식 장 제목 | 이 저장소에서 다루는 내용 | 실행 | 짧은 실습 |
|---|---|---|---|---|
| 03 | Common Programming Concepts | 변수와 가변성, 상수, 섀도잉, 스칼라와 복합 타입, 함수, 제어 흐름, 섭씨 변환 | `cargo run --bin chapter03` | `chapter03.rs`에 음수 온도 테스트를 먼저 추가한 뒤 `celsius_to_fahrenheit`를 확인한다. |
| 04 | Understanding Ownership | 이동, 복제, 불변 및 가변 빌림, 문자열 슬라이스, `first_word` | `cargo run --bin chapter04` | `chapter04.rs`에 여러 공백이 있는 입력의 `first_word` 테스트를 추가하고 결과를 맞춘다. |
| 05 | Using Structs to Structure Related Data | 일반, 튜플, 유닛 구조체, 구조체 갱신, 메서드, 연관 함수, 사각형 넓이와 포함 관계 | `cargo run --bin chapter05` | `chapter05.rs`에 한 변만 더 큰 사각형은 포함하지 않는다는 테스트를 추가한다. |
| 06 | Enums and Pattern Matching | 데이터를 담는 열거형, `Option`, 빠짐없는 `match`, `if let`, `let else` | `cargo run --bin chapter06` | `chapter06.rs`에 선택되지 않은 경로의 설명을 검증하는 테스트를 추가한다. |
| 07 | Managing Growing Projects with Packages, Crates, and Modules | 중첩 모듈, 공개 범위, 절대 및 상대 경로, `use` 별칭, `pub use` | `cargo run --bin chapter07` | `chapter07.rs`의 메뉴 요약 테스트를 먼저 바꾸고 공개 경로를 통해 다른 인사말을 조합한다. |
| 08 | Common Collections | `Vec`, UTF-8 문자열의 문자 수, `HashMap`과 `entry`를 이용한 단어 빈도 | `cargo run --bin chapter08` | `chapter08.rs`에 세 번 반복되는 단어의 빈도 테스트를 추가한다. |
| 09 | Error Handling | `Result`와 `Option`, `?`, 패닉 없는 파싱, 형식 오류와 범위 오류를 구분하는 타입 오류 | `cargo run --bin chapter09` | `chapter09.rs`에 `0`이 `NotPositive`를 반환하는 테스트를 추가한다. |
| 10 | Generic Types, Traits, and Lifetimes | 제네릭 구조체와 함수, 트레이트와 기본 메서드, 트레이트 바운드, 수명, 참조 반환 | `cargo run --bin chapter10` | `chapter10.rs`에 길이가 같은 두 문자열에서 왼쪽 참조를 고르는 테스트를 추가한다. |
| 11 | Writing Automated Tests | `#[test]`, `assert_eq!`, `assert_ne!`, 오류 메시지, `should_panic`, `Result`를 반환하는 테스트 | `cargo run --bin chapter11` | `chapter11.rs`에 경계값 `100`이 허용된다는 테스트를 먼저 작성한다. |
| 12 | An I/O Project: Building a Command Line Program | 순수한 미니그렙 구성 파싱, 대소문자 검색, 수명, `Result`와 `?`로 조합 | `cargo run --bin chapter12` | `chapter12.rs`에 검색 결과가 없는 내장 문자열 테스트를 추가한다. |
| 13 | Functional Language Features: Iterators and Closures | 클로저 캡처, `Fn`, `FnMut`, `FnOnce`, 반복자 어댑터, 사용자 정의 `Iterator` | `cargo run --bin chapter13` | `chapter13.rs`에 `Counter::new(1)`의 종료 동작을 검증하는 테스트를 추가한다. |
| 14 | More About Cargo and Crates.io | 개발 및 릴리스 프로필 개념, 문서 주석 예제, 공개 모듈과 재노출, Cargo 명령 관례 | `cargo run --bin chapter14` | `chapter14.rs`에 다른 음수 입력의 `add_one` 테스트를 추가하고 문서 예제와 동작을 함께 확인한다. |
| 15 | Smart Pointers | `Box`, `Deref`, `Drop`, `Rc`, `RefCell`, `Weak`, 재귀 자료형과 내부 가변성 | `cargo run --bin chapter15` | `chapter15.rs`에 세 노드 재귀 목록의 합 테스트를 추가한다. |
| 16 | Fearless Concurrency | 소유권을 옮기는 스레드, 조인, 다중 생산자 채널, `Arc<Mutex<_>>`, `Send`와 `Sync` | `cargo run --bin chapter16` | `chapter16.rs`에 한 작업자만 있는 공유 카운터 테스트를 추가한다. |
| 17 | Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams | `async`와 `await`, future 결합, `block_on`, 비동기 스트림 소비 | `cargo run --bin chapter17` | `chapter17.rs`에 홀수만 있는 스트림이 빈 벡터를 만드는 테스트를 추가한다. |
| 18 | Object-Oriented Programming Features of Rust | 상태와 평균값 캡슐화, 트레이트 객체, 동적 디스패치, 서로 다른 구성 요소 렌더링 | `cargo run --bin chapter18` | `chapter18.rs`에 값을 제거한 뒤 남은 평균을 검증하는 테스트를 추가한다. |
| 19 | Patterns and Matching | 구조체와 튜플 분해, 범위, OR 패턴, 매치 가드, `@`, `if let`, `while let`, `for` 패턴 | `cargo run --bin chapter19` | `chapter19.rs`에 원점과 축 위 점을 각각 분류하는 테스트를 추가한다. |

12장의 실행은 실제 파일, 환경 변수, 네트워크, 프로세스 명령줄을 읽지 않는다. 코드에 들어 있는 쿼리와 본문을 사용하므로 언제 실행해도 같은 결과를 확인할 수 있다. 17장은 이 저장소에서 유일한 외부 의존성인 `futures` 하나만 사용한다.

## 테스트부터 공부하는 순서

한 장을 다음 순서로 반복한다. 아래 예시는 3장이며, 다른 장은 대상 이름만 바꾼다.

1. 공식 Rust Book의 해당 장과 `src/bin/chapter03.rs`를 읽는다.
2. `cargo run --bin chapter03`으로 현재 동작을 본다.
3. 바꾸려는 동작을 나타내는 테스트를 파일 안에 먼저 추가한다.
4. `cargo test --bin chapter03`을 실행해 새 테스트가 의도한 이유로 실패하는지 본다.
5. 최소한의 구현으로 테스트를 통과시킨다.
6. `cargo test --bin chapter03`을 다시 실행해 통과를 확인한다.

## 전체 확인

개별 장을 마친 뒤 저장소 전체를 확인한다. 전체 테스트에는 17개 바이너리를 실제로 실행하는 스모크 테스트도 포함된다.

```console
cargo build --all-targets
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```
