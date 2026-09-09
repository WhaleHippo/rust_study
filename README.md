# Rust Book 03장부터 19장까지

[공식 안정판 Rust Book](https://doc.rust-lang.org/stable/book/)의 03장부터 19장까지를 실제 코드와 테스트로 공부하는 자습용 워크스페이스다. 각 장은 `cargo new`의 표준 디렉터리 형태를 따르는 독립 자식 패키지다. 한 장의 코드와 테스트를 함께 실행할 수 있고, 루트에서는 전체 패키지를 한 번에 검사할 수 있다.

## 문서 안내

- [CURRICULUM.md](CURRICULUM.md): 장별 선수 지식, 목표, 개념 지도, 읽기 순서, 실행 관찰, RED와 GREEN 실습
- [GLOSSARY.md](GLOSSARY.md): 처음 등장하는 장을 표시한 Rust 용어 정의와 과정 복귀 링크

처음 공부한다면 [학습 반복 절차](CURRICULUM.md#한-장을-공부하는-반복-절차)를 읽고 03장부터 숫자 순서대로 진행한다.

## 요구 환경

- Rust 1.90 이상
- Rust edition 2024

```console
rustup update stable
rustc --version
cargo --version
```

## 워크스페이스 구조

루트 `Cargo.toml`에는 `[workspace]`만 있고 루트 패키지는 없다. 즉, 실행할 루트 크레이트가 없는 가상 워크스페이스다. 루트 `src`는 실행 진입점으로 쓰지 않도록 의도적으로 비워 두며, 학습 대상은 `chapter03/`부터 `chapter19/`까지다.

각 자식 패키지는 자체 `Cargo.toml`과 소스를 가진다. 대부분의 장은 `chapterNN/src/main.rs` 하나로 구성된다. 07장은 물리 파일로 나눈 라이브러리와 바이너리 및 통합 테스트를, 14장은 라이브러리와 바이너리 및 문서 테스트를 함께 제공한다.

워크스페이스는 루트 `Cargo.lock` 하나로 의존성 버전을 고정하고 루트 `target/` 하나에 빌드 결과를 모은다. 외부 의존성을 직접 선언한 패키지는 17장뿐이며, [`chapter17/Cargo.toml`](chapter17/Cargo.toml)의 `futures`는 다른 장에 전파되지 않는다.

## 장별 실행

패키지는 `-p` 또는 `--package`로 선택한다. 아래 명령의 `chapter03`을 원하는 장으로 바꾸면 된다.

```console
cargo run -p chapter03
cargo check -p chapter03
cargo test -p chapter03
```

## 장 목록

| 장 | 공식 주제 | 패키지 진입점 | 과정 |
|---|---|---|---|
| 03 | Common Programming Concepts | [`chapter03/src/main.rs`](chapter03/src/main.rs) | [03장](CURRICULUM.md#chapter-03) |
| 04 | Understanding Ownership | [`chapter04/src/main.rs`](chapter04/src/main.rs) | [04장](CURRICULUM.md#chapter-04) |
| 05 | Using Structs to Structure Related Data | [`chapter05/src/main.rs`](chapter05/src/main.rs) | [05장](CURRICULUM.md#chapter-05) |
| 06 | Enums and Pattern Matching | [`chapter06/src/main.rs`](chapter06/src/main.rs) | [06장](CURRICULUM.md#chapter-06) |
| 07 | Managing Growing Projects with Packages, Crates, and Modules | [`chapter07/src/lib.rs`](chapter07/src/lib.rs), [`chapter07/src/main.rs`](chapter07/src/main.rs) | [07장](CURRICULUM.md#chapter-07) |
| 08 | Common Collections | [`chapter08/src/main.rs`](chapter08/src/main.rs) | [08장](CURRICULUM.md#chapter-08) |
| 09 | Error Handling | [`chapter09/src/main.rs`](chapter09/src/main.rs) | [09장](CURRICULUM.md#chapter-09) |
| 10 | Generic Types, Traits, and Lifetimes | [`chapter10/src/main.rs`](chapter10/src/main.rs) | [10장](CURRICULUM.md#chapter-10) |
| 11 | Writing Automated Tests | [`chapter11/src/main.rs`](chapter11/src/main.rs) | [11장](CURRICULUM.md#chapter-11) |
| 12 | An I/O Project: Building a Command Line Program | [`chapter12/src/main.rs`](chapter12/src/main.rs) | [12장](CURRICULUM.md#chapter-12) |
| 13 | Functional Language Features: Iterators and Closures | [`chapter13/src/main.rs`](chapter13/src/main.rs) | [13장](CURRICULUM.md#chapter-13) |
| 14 | More About Cargo and Crates.io | [`chapter14/src/lib.rs`](chapter14/src/lib.rs), [`chapter14/src/main.rs`](chapter14/src/main.rs) | [14장](CURRICULUM.md#chapter-14) |
| 15 | Smart Pointers | [`chapter15/src/main.rs`](chapter15/src/main.rs) | [15장](CURRICULUM.md#chapter-15) |
| 16 | Fearless Concurrency | [`chapter16/src/main.rs`](chapter16/src/main.rs) | [16장](CURRICULUM.md#chapter-16) |
| 17 | Fundamentals of Asynchronous Programming | [`chapter17/src/main.rs`](chapter17/src/main.rs) | [17장](CURRICULUM.md#chapter-17) |
| 18 | Object-Oriented Programming Features of Rust | [`chapter18/src/main.rs`](chapter18/src/main.rs) | [18장](CURRICULUM.md#chapter-18) |
| 19 | Patterns and Matching | [`chapter19/src/main.rs`](chapter19/src/main.rs) | [19장](CURRICULUM.md#chapter-19) |

12장은 실제 프로세스 인수, 파일, 환경 변수, 네트워크를 읽지 않는다. 코드에 주입한 메모리 입력으로 미니그렙의 파싱과 검색을 매번 같은 결과로 실행한다.

## 전체 품질 검사

```console
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets -- -D warnings
bash scripts/check-all-chapters.sh
```

개별 테스트 이름과 경계 조건은 [CURRICULUM.md](CURRICULUM.md)에 적혀 있다. 개념을 확인하고 싶을 때는 [GLOSSARY.md](GLOSSARY.md)에서 해당 장이나 용어를 찾는다.
