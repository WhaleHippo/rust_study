# Day 004: Cargo 명령 선택

## 목표

`cargo check`, `cargo build`, `cargo test`, `cargo doc`의 결과물과 사용 시점을 구분하고 day002/day003 package에 올바른 명령을 적용한다.

## 선수 지식과 돌아가기

- 선수 학습: [Day 002: 프로젝트 단위](../day002-project-units/README.md), [Day 003: 식과 place](../day003-expressions-and-places/README.md)
- 개념 지도: [도구 체인과 컴파일러 모델](../README.md#1-도구-체인과-컴파일러-모델)
- 진도표: [day004-cargo-commands](../topics.md#1-도구-체인과-컴파일러-모델)

## 핵심 개념

| 명령 | 하는 일 | 알맞은 질문 |
| --- | --- | --- |
| `cargo check` | 코드를 검사하고 crate metadata까지 만들지만 최종 실행 산출물 링크는 생략한다. | 지금 코드가 빠르게 컴파일되는가? |
| `cargo build` | 현재 profile의 라이브러리와 실행 target을 완성한다. | 실행하거나 배포할 산출물이 필요한가? |
| `cargo test` | test profile로 테스트 하네스와 관련 target을 만들고 테스트를 실행한다. | 동작 계약이 지켜지는가? |
| `cargo doc` | 의존 package의 문서를 포함한 API 문서를 생성한다. | 공개 API를 탐색할 문서가 필요한가? |

모두 `Cargo.toml`에서 package 정보를 읽고 필요하면 `rustc` 또는 `rustdoc`을 호출한다. `--manifest-path`를 사용하면 현재 디렉터리와 관계없이 검사할 package를 명시할 수 있다.

## 예제 관찰

[`examples/commands.md`](examples/commands.md)의 명령은 day002와 day003의 manifest를 직접 지정한다.

- `check`와 `build`는 성공 시 보통 `Finished`를 출력하지만 목적과 산출물이 다르다.
- `test` 출력에는 통과, 실패, 무시된 테스트 수가 나타난다. `learner-practice` 기능을 켜도 연습 계약은 `ignored`라 기본 실행에서 실패하지 않는다.
- `doc --no-deps`는 이 학습 package의 문서만 생성하고 `target/doc/.../index.html` 경로를 안내한다.

Cargo는 변경되지 않은 target을 재사용하므로 두 번째 실행이 더 짧거나 `Fresh`로 표시될 수 있다.

## 흔한 오해

`cargo check`가 통과했다고 테스트 동작까지 맞는 것은 아니다. 반대로 `cargo test`만 반복하면 단순 타입 오류를 빠르게 확인하려는 단계에서도 불필요한 test target 빌드 비용을 낼 수 있다.

## 퀴즈

1. 편집 중 가장 빠르게 타입 오류를 확인하려면 어떤 명령을 먼저 고르는가?
2. `target/debug`에 실행 가능한 산출물이 필요할 때 `check`로 충분한가?
3. `#[ignore]` 테스트를 기본 `cargo test`가 실행하지 않는 이유는 무엇인가?
4. 의존성 문서를 제외하고 현재 package 문서만 만들려면 어떤 옵션을 붙이는가?

## 실습 방법과 완료 기준

1. [`examples/commands.md`](examples/commands.md)의 day002 명령 네 개를 순서대로 실행한다.
2. manifest 경로를 day003으로 바꾸어 같은 명령을 반복한다.
3. 각 출력에서 `Checking`/`Compiling`/`Finished`, 테스트 요약, 생성된 문서 경로를 찾는다.

완료 기준은 네 명령을 결과물과 목적에 따라 선택하고, day002/day003 모두에서 명령이 성공하는 것을 직접 확인하는 것이다. 이 날은 새 Cargo package나 연습 소스 파일을 만들지 않는다.

## 공식 참고 자료

- [`cargo check`](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
- [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html)
- [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [`cargo doc`](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
