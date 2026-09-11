# Day 001: rustup, rustc, cargo의 역할

## 목표

`rustup`, `rustc`, `cargo`가 각각 무엇을 관리하는지 구분하고, 버전 확인 명령의 출력을 읽는다.

## 선수 지식과 돌아가기

- 선수 지식: 터미널에서 명령을 실행하고 현재 디렉터리를 확인할 수 있어야 한다.
- 개념 지도: [도구 체인과 컴파일러 모델](../README.md#1-도구-체인과-컴파일러-모델)
- 진도표: [day001-toolchain-roles](../topics.md#1-도구-체인과-컴파일러-모델)

## 핵심 개념

### `rustup`: 도구 체인 관리자

`rustup`은 stable, beta, nightly 같은 **toolchain**과 target, component를 설치하고 선택한다. 소스 코드를 직접 빌드하는 도구가 아니다. 일반 학습과 프로젝트에는 stable을 기본으로 쓴다.

### `rustc`: Rust 컴파일러

`rustc`는 Rust 소스를 **crate** 단위로 컴파일한다. 이름과 타입을 확인하고 빌림 규칙을 검사한 뒤 코드 생성과 링크를 수행한다. 단일 파일을 직접 실험할 수 있지만, 프로젝트의 반복 작업은 보통 Cargo에 맡긴다.

### `cargo`: 프로젝트 작업 관리자

`cargo`는 `Cargo.toml`을 읽어 package, target, feature를 파악하고 `rustc`를 적절한 옵션으로 호출한다. 생성, 검사, 빌드, 실행, 테스트, 문서 생성과 의존성 해석을 한 인터페이스에서 제공한다.

```text
rustup -> 사용할 toolchain을 선택
cargo  -> 프로젝트 작업을 계획하고 rustc를 호출
rustc  -> crate를 컴파일
```

edition은 소스 문법과 관용구의 호환 경계이고 toolchain은 설치된 컴파일러와 도구 묶음이다. `edition = "2024"`라고 썼다고 nightly toolchain을 사용한다는 뜻은 아니다.

## 예제 관찰

[`examples/commands.md`](examples/commands.md)의 세 명령을 실행한다. 이 환경에서는 다음을 확인했다.

```text
stable-x86_64-unknown-linux-gnu (default)
rustc 1.95.0 (59807616e 2026-04-14)
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

첫 줄은 선택된 채널, host target, 기본 여부를 보여 준다. `rustc`와 `cargo`의 버전은 서로 관련되지만 각자 별도 프로그램이므로 commit과 날짜가 다를 수 있다. 버전과 hash는 설치 시점에 따라 달라지는 것이 정상이다.

## 흔한 오해

`cargo`가 별도 Rust 컴파일러인 것은 아니다. Cargo는 빌드 과정을 조정하고 실제 Rust crate 컴파일은 선택된 toolchain의 `rustc`에 맡긴다.

## 퀴즈

1. 다른 target용 표준 라이브러리를 설치하는 책임은 어느 도구에 있는가?
2. 단일 crate의 타입과 빌림을 검사하는 주체는 무엇인가?
3. `Cargo.toml`을 읽고 테스트 target을 실행하는 주체는 무엇인가?
4. edition과 stable/nightly 채널은 왜 같은 축이 아닌가?

## 실습 방법과 완료 기준

1. [`examples/commands.md`](examples/commands.md)의 명령을 직접 실행한다.
2. 출력에서 활성 toolchain, host target, `rustc` release, Cargo 버전을 표시한다.
3. 각 명령을 `설치/선택`, `컴파일`, `프로젝트 작업` 중 하나로 분류한다.

완료 기준은 세 도구의 책임을 한 문장씩 설명하고, 자신의 출력이 예시와 달라도 어떤 필드가 무엇을 뜻하는지 설명하는 것이다. 이 날은 Cargo package나 별도 연습 소스 파일을 만들지 않는다.

## 공식 참고 자료

- [rustup 문서](https://rust-lang.github.io/rustup/)
- [`rustc` 문서](https://doc.rust-lang.org/rustc/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)
- [The Edition Guide](https://doc.rust-lang.org/edition-guide/)
