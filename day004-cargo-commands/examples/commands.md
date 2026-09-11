# day002/day003에 Cargo 명령 실행하기

저장소 루트에서 실행한다. `<day>`를 `day002-project-units` 또는 `day003-expressions-and-places`로 바꾼다.

## 빠른 컴파일 검사

```console
$ cargo check --manifest-path <day>/Cargo.toml
    Checking <package> v0.1.0 (...)
    Finished `dev` profile ...
```

최종 링크보다 빠른 피드백이 목적이다.

## 개발용 산출물 빌드

```console
$ cargo build --manifest-path <day>/Cargo.toml
   Compiling <package> v0.1.0 (...)
    Finished `dev` profile ...
```

workspace 안의 라이브러리 산출물은 루트 `target/debug` 아래에 생긴다.

## 테스트 실행

```console
$ cargo test --manifest-path <day>/Cargo.toml
running 1 test
test result: ok. 1 passed; 0 failed; ...
```

실제 테스트 수는 package마다 다르다. 기능을 포함한 정상 검사에는 다음 명령을 쓴다.

```console
$ cargo test --manifest-path <day>/Cargo.toml --all-features
test result: ok. ...; 1 ignored; ...
```

## API 문서 생성

```console
$ cargo doc --manifest-path <day>/Cargo.toml --no-deps
 Documenting <package> v0.1.0 (...)
    Finished `dev` profile ...
   Generated <workspace-root>/target/doc/<crate_name>/index.html
```

package 이름의 `-`는 Rust crate 경로와 문서 디렉터리에서 `_`로 바뀐다.
