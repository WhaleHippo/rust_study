# Day 011: slice와 ownership

## 목표

`&str`과 `&[T]`가 데이터를 소유하지 않고 원본의 연속 구간을 빌리는 view임을 설명하고, 반환된 슬라이스가 소유자보다 오래 살 수 없다는 관계를 코드에서 찾는다.

## 선수 지식과 돌아가기

- 선수 지식: [Day 010: borrowing과 reborrowing](../day010-borrowing-and-reborrowing/README.md)
- [전체 개념 지도](../README.md)
- [학습 주제 목록의 day011](../topics.md#3-소유권-빌림-슬라이스-수명-소멸)

## 핵심 설명

슬라이스 `&[T]`와 문자열 슬라이스 `&str`은 원본 데이터의 시작 위치와 길이를 가진 빌린 view다. 요소를 복제하거나 소유권을 가져오지 않는다. `tail_from(&numbers, 1)`의 결과는 `numbers`의 일부를 가리키므로 `numbers`가 살아 있는 동안에만 사용할 수 있다. 함수 시그니처의 lifetime elision은 `first_word`의 출력이 입력 `text`에서 빌렸다는 관계를 표현한다.

Rust의 `str`은 유효한 UTF-8이다. 바이트 위치로 문자열을 자를 때는 문자 경계여야 한다. `first_word`는 `char_indices()`가 돌려준 경계 위치에서만 슬라이스하므로 한글처럼 여러 바이트인 문자를 안전하게 처리한다. 첫 Unicode whitespace 전까지를 반환하며, 빈 문자열에는 빈 슬라이스를, 공백이 없으면 전체 입력을 반환한다.

소유자보다 오래 슬라이스를 쓰는 코드는 target에 넣지 않고 Markdown으로 확인한다.

```rust,compile_fail
let word: &str;
{
    let owner = String::from("rust ownership");
    word = &owner[..4];
} // owner가 여기서 drop됨
println!("{word}"); // error: 빌린 데이터가 이미 소멸함
```

문자 경계가 아닌 바이트 위치를 직접 선택하면 컴파일 오류가 아니라 실행 중 panic이 될 수 있으므로 그런 코드를 예제 target에 넣지 않는다.

## 예제 실행과 관찰

```bash
cargo run --manifest-path day011-slice-ownership/Cargo.toml --example example
```

출력은 `String` 소유자와 그 안을 가리키는 첫 단어 `&str`을 함께 보여 준다. 배열도 소유권을 넘기지 않고 `&[T]` 꼬리 슬라이스를 만든다.

## 흔한 오해

슬라이스는 잘라 낸 데이터의 새 소유자가 아니다. 또한 `&str`이 문자 배열이라는 뜻도 아니다. 문자열 슬라이스의 길이는 UTF-8 바이트 수이며 정수 인덱싱은 지원하지 않는다.

## 확인 퀴즈

1. 함수가 `String` 대신 `&str`을 받으면 호출자의 어떤 선택을 보존하는가?
2. 반환된 슬라이스를 원본 소유자가 drop된 뒤 사용할 수 없는 이유는 무엇인가?
3. `char_indices()`의 인덱스로 자르면 UTF-8 경계를 지킬 수 있는 이유는 무엇인가?

## 연습 문제와 채점 기준

`src/practice.rs`는 `learner-practice` feature를 켤 때만 컴파일된다. `learner_first_word`가 할당 없이 입력에서 빌린 첫 단어를 반환하도록 수정한다. 한글 입력을 문자 경계에서 안전하게 자르고 아래 ignored 테스트가 `"소유권"`을 얻으면 완료다.

```bash
cargo test --manifest-path day011-slice-ownership/Cargo.toml \
  --features learner-practice -- --ignored
```

starter sentinel은 전체 입력을 반환하므로 선택된 연습 테스트가 실패한다. 일반 `cargo test --all-features`에서는 ignored 테스트를 실행하지 않아 기준선이 통과한다.

## 공식 자료

- [The Rust Programming Language: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [The Rust Reference: Slice types](https://doc.rust-lang.org/reference/types/slice.html)
- [`str::char_indices`](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices)
- [`slice::get`](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
