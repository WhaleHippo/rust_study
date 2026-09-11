# 명령과 출력 읽기

명령은 어느 디렉터리에서 실행해도 된다. 아래 출력은 한 환경의 관찰값이며 버전, commit, 날짜, host는 설치에 따라 달라진다.

## 활성 toolchain

```console
$ rustup show active-toolchain
stable-x86_64-unknown-linux-gnu (default)
```

- `stable`: release channel
- `x86_64-unknown-linux-gnu`: host target triple
- `(default)`: 디렉터리별 override가 없을 때 선택되는 toolchain

## 컴파일러 정보

```console
$ rustc --version --verbose
rustc 1.95.0 (59807616e 2026-04-14)
binary: rustc
commit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860
commit-date: 2026-04-14
host: x86_64-unknown-linux-gnu
release: 1.95.0
LLVM version: 22.1.2
```

`release`는 컴파일러 버전, `host`는 이 컴파일러가 실행되는 기본 target, `LLVM version`은 코드 생성 backend 버전이다.

## Cargo 정보

```console
$ cargo --version
cargo 1.95.0 (f2d3ce0bd 2026-03-21)
```

Cargo 버전과 source commit을 보여 준다. 이 명령은 프로젝트를 빌드하지 않는다.
