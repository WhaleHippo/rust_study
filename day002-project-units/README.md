# Day 002: package, crate, module, edition, toolchain

## 목표

package, crate, module의 경계를 실제 파일과 코드에서 찾고, edition과 toolchain이 서로 다른 선택임을 설명한다.

## 선수 지식과 돌아가기

- 선수 학습: [Day 001: 도구 체인의 역할](../day001-toolchain-roles/README.md)
- 개념 지도: [도구 체인과 컴파일러 모델](../README.md#1-도구-체인과-컴파일러-모델), [모듈, 크레이트, 가시성, Cargo](../README.md#6-모듈-크레이트-가시성-cargo)
- 진도표: [day002-project-units](../topics.md#1-도구-체인과-컴파일러-모델)

## 핵심 개념

- **package**: 하나의 `Cargo.toml`이 설명하는 빌드와 배포 단위다. 이 폴더 전체가 `day002-project-units` package다.
- **crate**: `rustc`가 한 번에 컴파일하는 단위다. `src/lib.rs`는 이 package의 library crate root다. `examples/example.rs`는 별도 example binary crate의 root다.
- **module**: crate 안에서 이름과 가시성을 조직하는 단위다. `lib.rs`의 `project_units`와 feature가 켜질 때의 `practice`가 module이다.
- **edition**: 호환성을 유지하며 문법과 이름 해석 규칙을 발전시키는 package 설정이다. 이 package는 `edition = "2024"`다.
- **toolchain**: `rustc`, Cargo, 표준 라이브러리, component의 설치 묶음이다. edition은 toolchain 버전이나 release channel을 고정하지 않는다.

`pub use project_units::{ProjectUnit, unit_label};`는 내부 module의 항목을 crate root에서 다시 공개한다. 따라서 example crate는 `day002_project_units::ProjectUnit` 경로를 쓸 수 있다. package 이름의 `-`는 Rust crate 경로에서 `_`가 된다.

다음 코드는 module 선언이 런타임 파일 로딩이 아님을 보여 주는 **컴파일 실패 학습 조각**이다. 실제 소스에는 넣지 않는다.

```rust,compile_fail
mod lessons; // src/lessons.rs 또는 src/lessons/mod.rs가 없으므로 컴파일되지 않는다.
```

## 예제 관찰

```console
$ cargo run --manifest-path day002-project-units/Cargo.toml --example example
package: package
crate: crate
module: module
```

한 package 안에서 library crate의 공개 API를 example binary crate가 사용한다. `project_units` module은 세 단위를 enum으로 구분하며 `unit_label`은 모든 variant를 빠짐없이 처리한다.

## 흔한 오해

파일 하나가 항상 module 하나이고 package 하나가 항상 crate 하나인 것은 아니다. module은 module tree의 논리 경계이며, 한 package는 library와 여러 binary/example crate를 함께 가질 수 있다.

## 퀴즈

1. 이 폴더에서 package 경계를 선언하는 파일은 무엇인가?
2. `src/lib.rs`와 `examples/example.rs`는 같은 crate인가?
3. `project_units`는 package, crate, module 중 무엇인가?
4. edition 2024를 선택하면 nightly compiler가 자동 선택되는가?

## 실습 방법과 완료 기준

1. `cargo test --manifest-path day002-project-units/Cargo.toml --all-features`를 실행해 ignored 테스트를 확인한다.
2. [`src/practice.rs`](src/practice.rs)의 `classify_project_file`이 `Cargo.toml`을 `Some(ProjectUnit::Package)`로 분류하도록 sentinel 반환을 교체한다.
3. 아래 명령으로 학습 계약만 명시적으로 실행한다.

```console
$ cargo test --manifest-path day002-project-units/Cargo.toml --features learner-practice -- --ignored
```

완료 기준은 ignored 계약 테스트가 통과하고, 기본 테스트와 `--all-features` Clippy가 계속 통과하며, package/crate/module의 차이를 이 폴더의 파일로 설명하는 것이다. `PRACTICE_SENTINEL`은 panic이나 unsafe 없이 미완료 상태를 드러내는 시작값이며 정답 구현은 포함하지 않는다.

## 공식 참고 자료

- [The Cargo Book: Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [The Rust Reference: Crates and source files](https://doc.rust-lang.org/reference/crates-and-source-files.html)
- [The Rust Reference: Modules](https://doc.rust-lang.org/reference/items/modules.html)
- [The Edition Guide](https://doc.rust-lang.org/edition-guide/)
- [rustup: Toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html)
