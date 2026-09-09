# Rust Book 03장부터 19장까지 자습 과정

이 과정은 공식 Rust Book을 읽고, 실제 `chapter03`부터 `chapter19` 패키지를 실행하며, 경계 조건을 테스트로 먼저 표현하도록 구성했다. 설명을 외우기보다 코드가 보장하는 계약을 찾는 데 초점을 둔다. 모든 명령은 저장소 루트에서 실행한다.

## 준비와 워크스페이스 명령

Rust 1.90 이상과 edition 2024가 필요하다. 루트 `Cargo.toml`은 실행 대상이 없는 가상 워크스페이스다. 학습 코드는 각 자식 패키지에 있고, `Cargo.lock`과 `target/`은 워크스페이스 전체가 공유한다.

```console
rustc --version
cargo --version
cargo check --workspace
cargo test --workspace
```

장 하나를 다룰 때는 패키지 선택 옵션 `-p`를 쓴다.

```console
cargo run -p chapter03
cargo check -p chapter03
cargo test -p chapter03
```

전체 회귀 검사는 다음 순서로 실행한다.

```console
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets -- -D warnings
bash scripts/check-all-chapters.sh
```

## 한 장을 공부하는 반복 절차

1. 이 문서의 선수 지식과 목표를 읽고 모르는 말은 [용어집](GLOSSARY.md)에서 확인한다.
2. 연결된 공식 Rust Book 장을 읽는다.
3. 안내된 소스 진입점을 순서대로 읽고 `cargo run -p chapterNN`으로 관찰값을 확인한다.
4. 새 기능을 연습한다면 아직 구현하지 않은 경계 사례를 테스트로 먼저 작성한다. 제공된 완성 예제를 복습한다면 장별 실습에 적힌 한 줄 변형을 적용해 의도적으로 회귀를 만든다.
5. `cargo test -p chapterNN 테스트_이름`으로 RED를 확인하고, 실패 메시지가 새 계약 또는 의도한 회귀를 가리키는지 읽는다.
6. 가장 작은 구현 변경이나 변형 복구로 GREEN을 만든다. 이 저장소의 완성된 답과 비교한 뒤 `cargo test -p chapterNN`을 통과시킨다.
7. 실행 결과와 소유권, 타입, 공개 API의 관계를 말로 설명하고 성찰 질문에 답한다.

이미 완성된 저장소에서 RED를 재현하려면 해당 경계 테스트가 요구하는 구현 한 줄만 학습용으로 되돌린다. 실패를 확인한 뒤 즉시 원래 구현을 복구한다. 커밋 전에는 항상 전체 패키지 테스트가 GREEN이어야 한다.

## 목차

[03장](#chapter-03) · [04장](#chapter-04) · [05장](#chapter-05) · [06장](#chapter-06) · [07장](#chapter-07) · [08장](#chapter-08) · [09장](#chapter-09) · [10장](#chapter-10) · [11장](#chapter-11) · [12장](#chapter-12) · [13장](#chapter-13) · [14장](#chapter-14) · [15장](#chapter-15) · [16장](#chapter-16) · [17장](#chapter-17) · [18장](#chapter-18) · [19장](#chapter-19)

<a id="chapter-03"></a>
## 03장. 일반적인 프로그래밍 개념

공식 문서: [Common Programming Concepts](https://doc.rust-lang.org/stable/book/ch03-00-common-programming-concepts.html)

**선수 지식:** `cargo run` 사용법, 함수 호출, 기본 산술식.

**학습 목표**

1. 불변 바인딩, `mut`, 상수, 섀도잉의 차이를 코드에서 구분한다.
2. 스칼라, 튜플, 배열의 타입과 접근법을 설명한다.
3. 표현식인 `if`와 값을 반환하는 `loop`, `while`, `for`를 비교한다.
4. `const fn`으로 작성한 온도 변환의 경계값을 테스트한다.

**개념 지도:** 바인딩은 값을 이름에 연결하고, 타입은 허용되는 연산을 정한다. 블록의 마지막 표현식은 값을 만들 수 있으므로 제어 흐름도 계산의 일부가 된다. 상수 함수 `celsius_to_fahrenheit`는 이 규칙을 작은 순수 함수로 보여 준다. 관련 용어는 [바인딩](GLOSSARY.md#term-binding), [섀도잉](GLOSSARY.md#term-shadowing), [표현식](GLOSSARY.md#term-expression), [스칼라](GLOSSARY.md#term-scalar)를 참고한다.

**소스와 읽기 순서**

1. [`chapter03/src/main.rs`](chapter03/src/main.rs)의 상수와 `celsius_to_fahrenheit`, `plus_one`을 읽는다.
2. `main`의 바인딩과 타입을 본 뒤 `if`, `loop`, `while`, `for` 순서로 값을 추적한다.
3. 같은 파일의 `#[cfg(test)] mod tests`에서 정상값과 음수 경계를 비교한다.

**실행 관찰:** `cargo run -p chapter03`은 장 제목, 섀도잉 뒤 문자열 길이, 각 제어 흐름의 결과, 시간당 초, `0C`와 `-10C`의 화씨값을 출력한다. `-10C`는 `14F`이며 음수 분기 설명도 함께 나온다.

**테스트 우선 실습:** `converts_negative_temperature_when_celsius_is_below_zero`처럼 `-10.0`이 `14.0`이 되는 테스트를 먼저 작성한다. RED에서는 변환식을 임시로 `celsius * 9.0 / 5.0`으로 두고 실패값이 `-18.0`인지 확인한다. GREEN에서는 `+ 32.0`을 복구한다.

```console
cargo test -p chapter03 converts_negative_temperature_when_celsius_is_below_zero
cargo test -p chapter03
```

**성찰 질문:** 섀도잉은 왜 `mut`와 다른가? `loop`의 `break` 뒤 값은 어디에 저장되는가? `f64` 비교가 일반적으로 조심스러운데 이 예제의 값은 왜 정확히 비교 가능한가?

<a id="chapter-04"></a>
## 04장. 소유권 이해하기

공식 문서: [Understanding Ownership](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)

**선수 지식:** 03장의 변수, 함수, 복합 타입.

**학습 목표**

1. 힙 데이터를 가진 `String`의 이동과 명시적 `clone`을 구분한다.
2. 불변 빌림과 가변 빌림의 배타 규칙을 설명한다.
3. 소유하지 않는 `&str` 슬라이스를 반환하는 함수를 읽는다.
4. 바이트 인덱스와 문자열 경계의 관계를 테스트로 확인한다.

**개념 지도:** 값에는 한 소유자가 있고 소유자가 범위를 벗어나면 값이 해제된다. 참조는 값을 빌리며, 슬라이스는 컬렉션 일부를 가리키는 참조다. `String`은 소유한 UTF-8 버퍼이고 `&str`은 문자열 데이터를 빌린 뷰다. [소유권](GLOSSARY.md#term-ownership), [이동](GLOSSARY.md#term-move), [빌림](GLOSSARY.md#term-borrow), [`Copy`](GLOSSARY.md#term-copy), [`String`과 `&str`](GLOSSARY.md#term-string-str)을 함께 본다.

**소스와 읽기 순서**

1. [`chapter04/src/main.rs`](chapter04/src/main.rs)의 `first_word`가 공백 바이트 위치로 슬라이스를 만드는 과정을 읽는다.
2. `add_exclamation`의 `&mut String`과 `main`의 이동, 복제, 불변 빌림을 비교한다.
3. 테스트에서 한 단어, 선행 공백, 여러 공백 사례를 순서대로 읽는다.

**실행 관찰:** 이동된 문자열은 새 이름으로 출력되고 복제본도 독립적으로 남는다. 가변 빌림을 받은 문자열에는 `!`가 붙는다. `"borrowed   slice"`의 첫 단어는 `"borrowed"`다.

**테스트 우선 실습:** `returns_prefix_when_words_have_multiple_spaces_between_them`을 먼저 작성해 여러 공백에서도 첫 공백에서 멈추는 계약을 고정한다. RED를 보려면 공백을 찾았을 때 `text` 전체를 반환하도록 잠시 바꾼다. GREEN에서는 `&text[..index]`를 복구한다.

```console
cargo test -p chapter04 returns_prefix_when_words_have_multiple_spaces_between_them
cargo test -p chapter04
```

**성찰 질문:** `greeting`을 이동한 뒤 원래 이름을 쓸 수 없는 이유는 무엇인가? `first_word`의 반환값 수명은 어느 입력과 연결되는가? `&mut`를 동시에 둘 허용하지 않는 규칙이 어떤 오류를 막는가?

<a id="chapter-05"></a>
## 05장. 구조체로 연관된 데이터 구성하기

공식 문서: [Using Structs to Structure Related Data](https://doc.rust-lang.org/stable/book/ch05-00-structs.html)

**선수 지식:** 04장의 소유권, 이동, 참조.

**학습 목표**

1. 필드 구조체, 튜플 구조체, 유닛 구조체를 구분한다.
2. 필드 초기화 축약과 구조체 갱신 문법의 소유권 효과를 읽는다.
3. 메서드, 연관 함수, `Self`가 맡는 역할을 설명한다.
4. 사각형 포함 조건의 양쪽 차원을 경계 테스트로 고정한다.

**개념 지도:** 구조체는 관련 값을 이름 있는 타입으로 묶는다. `impl` 블록은 인스턴스를 받는 메서드와 생성자처럼 쓰는 연관 함수를 타입에 연결한다. `Rectangle::can_hold`는 두 차원이 모두 더 작아야 한다는 불변 조건을 표현한다. [구조체](GLOSSARY.md#term-struct), [메서드](GLOSSARY.md#term-method), [연관 함수](GLOSSARY.md#term-associated-function), [구조체 갱신](GLOSSARY.md#term-struct-update)을 참고한다.

**소스와 읽기 순서**

1. [`chapter05/src/main.rs`](chapter05/src/main.rs)의 네 구조체 형태를 비교한다.
2. `Rectangle`의 `area`, `can_hold`, `square`를 읽는다.
3. `build_user`와 `..user`를 확인한 뒤 테스트의 포함 및 거부 사례를 읽는다.

**실행 관찰:** 갱신된 사용자, 검정 RGB, 면적 `1500`, 작은 정사각형 포함 여부 `true`, 한 차원이 너무 큰 후보의 결과 `false`가 나온다.

**테스트 우선 실습:** `rejects_rectangle_when_one_dimension_is_too_large`를 작성해 너비가 맞아도 높이가 큰 후보는 거부되게 한다. RED에서는 `can_hold`의 `&& self.height > other.height`를 잠시 제거한다. GREEN에서는 두 비교를 `&&`로 다시 묶는다.

```console
cargo test -p chapter05 rejects_rectangle_when_one_dimension_is_too_large
cargo test -p chapter05
```

**성찰 질문:** `area`가 `self`를 소비하지 않고 `&self`를 받는 이유는 무엇인가? `square`가 메서드가 아닌 연관 함수인 까닭은 무엇인가? 한 차원만 검사하면 어떤 잘못된 상태가 통과하는가?

<a id="chapter-06"></a>
## 06장. 열거형과 패턴 매칭

공식 문서: [Enums and Pattern Matching](https://doc.rust-lang.org/stable/book/ch06-00-enums.html)

**선수 지식:** 구조체, 메서드, 소유권과 참조.

**학습 목표**

1. 서로 다른 데이터를 담는 열거형 배리언트를 설계한다.
2. 빠짐없는 `match`로 모든 배리언트를 처리한다.
3. 값의 부재를 `Option<T>`로 표현하고 `map`을 적용한다.
4. `if let`과 `let else`를 단일 패턴 처리에 사용한다.

**개념 지도:** 열거형은 가능한 상태의 합을 타입에 담는다. `Option`은 값 있음과 없음을, `match`는 각 상태의 처리를 명시한다. `if let`은 관심 있는 한 패턴을, `let else`는 성공 패턴을 먼저 꺼내고 실패 시 흐름을 끝낸다. [열거형](GLOSSARY.md#term-enum), [배리언트](GLOSSARY.md#term-variant), [`Option`](GLOSSARY.md#term-option-result), [`match`](GLOSSARY.md#term-match), [`let else`](GLOSSARY.md#term-let-else)를 참고한다.

**소스와 읽기 순서**

1. [`chapter06/src/main.rs`](chapter06/src/main.rs)의 `Route`, `Coin`, `UsState` 정의를 읽는다.
2. `coin_value`와 `route_destination`의 빠짐없는 매칭을 본다.
3. `selected_route_destination`의 `let else`, `plus_one`의 `Option::map`, `main`의 `if let`을 비교한다.

**실행 관찰:** IPv4와 IPv6 설명, 알래스카 주화, 각 동전 값, `Some(6)`, 선택 경로가 없다는 문장이 출력된다.

**테스트 우선 실습:** `describes_absence_when_no_route_is_selected`를 먼저 작성한다. RED에서는 `None` 분기의 문자열을 빈 문자열로 바꾸고 실패 차이를 읽는다. GREEN에서는 `"no route selected"`를 반환한다.

```console
cargo test -p chapter06 describes_absence_when_no_route_is_selected
cargo test -p chapter06
```

**성찰 질문:** `Option`이 널 값보다 안전한 이유는 무엇인가? `coin_value`에 와일드카드 분기를 두지 않으면 어떤 장점이 있는가? `let else`의 `else` 블록이 흐름을 끝내야 하는 이유는 무엇인가?

<a id="chapter-07"></a>
## 07장. 패키지, 크레이트, 모듈로 프로젝트 관리하기

공식 문서: [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

**선수 지식:** 가시성 전의 함수와 타입, 경로 개념, 열거형.

**학습 목표**

1. 패키지, 라이브러리 크레이트, 바이너리 크레이트, 모듈을 정확히 구분한다.
2. `mod` 선언이 물리 파일을 모듈 트리에 연결하는 방식을 추적한다.
3. `pub`, `use`, `pub use`, 별칭의 가시성 효과를 설명한다.
4. 외부 소비자 관점의 통합 테스트로 공개 API를 검증한다.

**개념 지도:** `chapter07` 패키지는 `src/lib.rs`의 라이브러리 크레이트와 `src/main.rs`의 바이너리 크레이트를 함께 만든다. `lib.rs`가 크레이트 루트이며 `mod back_of_house;`는 `back_of_house.rs`를, `pub mod front_of_house;`는 `front_of_house.rs`를 연결한다. 다시 `pub mod hosting;`이 `front_of_house/hosting.rs`를 연결한다. 폴더만 만든다고 모듈이 생기는 것이 아니다. [패키지](GLOSSARY.md#term-package), [크레이트](GLOSSARY.md#term-crate), [모듈](GLOSSARY.md#term-module), [가시성](GLOSSARY.md#term-visibility), [재노출](GLOSSARY.md#term-reexport)을 참고한다.

**소스와 읽기 순서**

1. [`chapter07/src/lib.rs`](chapter07/src/lib.rs)에서 모듈 선언과 `pub use`로 시작한다.
2. [`chapter07/src/front_of_house.rs`](chapter07/src/front_of_house.rs), [`chapter07/src/front_of_house/hosting.rs`](chapter07/src/front_of_house/hosting.rs)를 따라 공개 중첩 경로를 확인한다.
3. [`chapter07/src/back_of_house.rs`](chapter07/src/back_of_house.rs)에서 공개 타입, 공개 필드, 비공개 필드를 구분한다.
4. [`chapter07/src/main.rs`](chapter07/src/main.rs)가 라이브러리를 외부 크레이트처럼 쓰는 모습을 본다.
5. [`chapter07/tests/public_api.rs`](chapter07/tests/public_api.rs)에서 실제 외부 소비자 경로를 확인한다.

**실행 관찰:** `cargo run -p chapter07`은 `Welcome`, `Wheat toast with peaches`, `Rust Study serves soup with Welcome`을 차례로 출력한다. 바이너리는 `chapter07::hosting` 재노출 경로를 쓴다.

**테스트 우선 실습:** 통합 테스트 `greeting_when_hosting_is_reexported_from_the_library`를 먼저 작성한다. RED를 위해 `lib.rs`의 `pub use crate::front_of_house::hosting;`을 잠시 제거하면 `chapter07::hosting` 경로가 컴파일되지 않는다. GREEN에서는 재노출을 복구한다.

```console
cargo test -p chapter07 greeting_when_hosting_is_reexported_from_the_library
cargo test -p chapter07
```

**성찰 질문:** 패키지와 크레이트가 같은 말이 아닌 이유는 무엇인가? `Breakfast::seasonal_fruit`가 외부에서 직접 수정되지 않는 이유는 무엇인가? `pub use`는 단순 `use`와 공개 API 면에서 어떻게 다른가?

<a id="chapter-08"></a>
## 08장. 일반적인 컬렉션

공식 문서: [Common Collections](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html)

**선수 지식:** 소유권, 참조, 구조체와 열거형, 반복문.

**학습 목표**

1. `Vec<T>`에 같은 타입의 값을 순서대로 저장하고 추가한다.
2. UTF-8 바이트 수와 유니코드 스칼라 값 개수를 구분한다.
3. `HashMap`의 `entry`, `and_modify`, `or_insert`로 빈도를 누적한다.
4. 빌린 문자열을 키로 보관할 때 수명 관계를 읽는다.

**개념 지도:** 벡터는 연속된 같은 타입의 값, 문자열은 UTF-8 바이트, 해시맵은 키와 값의 대응을 관리한다. `chars()`는 바이트가 아니라 유니코드 스칼라 값을 순회한다. 빈도 함수는 입력 `&str`의 단어 슬라이스를 키로 빌린다. [`Vec`](GLOSSARY.md#term-vec), [UTF-8](GLOSSARY.md#term-utf8), [`HashMap`](GLOSSARY.md#term-hashmap), [`entry`](GLOSSARY.md#term-entry)을 참고한다.

**소스와 읽기 순서:** [`chapter08/src/main.rs`](chapter08/src/main.rs)에서 `vector_lesson`, `character_count`, `word_frequencies`, `main`, 테스트 순서로 읽는다.

**실행 관찰:** 벡터는 `[1, 2, 3, 4]`, `Rust` 문자 수는 `4`, 빈도 맵에서 `rust`는 `2`다. 해시맵의 출력 순서는 고정 계약이 아니므로 전체 문자열 순서에 기대지 않는다.

**테스트 우선 실습:** `word_frequencies_when_one_word_occurs_three_times`를 먼저 작성한다. RED에서는 `and_modify`를 제거해 기존 키가 증가하지 않게 한다. GREEN에서 `|count| *count += 1`을 복구한다.

```console
cargo test -p chapter08 word_frequencies_when_one_word_occurs_three_times
cargo test -p chapter08
```

**성찰 질문:** `"hi🌍"`가 3문자지만 더 많은 바이트인 이유는 무엇인가? 맵 키가 입력을 소유하지 않아도 되는 이유는 무엇인가? 삽입 순서가 출력 순서를 보장하지 않는 점은 테스트에 어떤 영향을 주는가?

<a id="chapter-09"></a>
## 09장. 오류 처리

공식 문서: [Error Handling](https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html)

**선수 지식:** 열거형, `Option`, `match`, 클로저의 기본 읽기.

**학습 목표**

1. 복구 가능한 오류를 `Result<T, E>`로 모델링한다.
2. 형식 오류, 범위 오류, 의미 오류를 타입 배리언트로 나눈다.
3. `?`가 오류를 조기에 반환하는 흐름을 추적한다.
4. `Result::ok`와 `find_map`으로 첫 성공값을 찾는다.

**개념 지도:** 패닉은 계속할 수 없는 상태를 위한 것이고 `Result`는 호출자가 처리할 수 있는 실패를 값으로 돌려준다. `parse_positive`는 구문, 표현 범위, 양수 조건을 구별한다. `first_positive`는 오류를 `Option`의 부재로 바꿔 탐색한다. [`Result`](GLOSSARY.md#term-option-result), [`?` 연산자](GLOSSARY.md#term-question-mark), [패닉](GLOSSARY.md#term-panic), [타입 오류](GLOSSARY.md#term-typed-error)을 참고한다.

**소스와 읽기 순서:** [`chapter09/src/main.rs`](chapter09/src/main.rs)의 `ParsePositiveError`, `parse_positive`, `first_positive`, `describe`, 테스트 순서로 읽는다.

**실행 관찰:** `0`, 잘못된 문자열, `12`를 차례로 검사한 뒤 `a positive value`를 출력한다. 예상 가능한 잘못된 입력은 패닉을 일으키지 않는다.

**테스트 우선 실습:** `parse_positive_when_input_is_zero`를 먼저 작성해 문법상 유효한 `0`을 `NotPositive`로 구분한다. RED에서는 마지막 `match`를 제거하고 `Ok(number)`를 반환한다. GREEN에서는 `0`과 양수를 다시 분기한다.

```console
cargo test -p chapter09 parse_positive_when_input_is_zero
cargo test -p chapter09
```

**성찰 질문:** `Malformed`와 `OutOfRange`를 나누면 호출자에게 어떤 선택권이 생기는가? `first_positive`에서 오류 상세가 사라지는 이유는 무엇인가? 어떤 실패라면 `panic!`이 더 적절한가?

<a id="chapter-10"></a>
## 10장. 제네릭 타입, 트레이트, 수명

공식 문서: [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)

**선수 지식:** 구조체와 `impl`, 참조, `Option`, 소유권.

**학습 목표**

1. 제네릭 함수와 여러 타입 매개변수를 가진 구조체를 읽는다.
2. 트레이트의 필수 메서드와 기본 메서드를 구현한다.
3. `impl Trait`와 `where` 트레이트 바운드가 허용하는 입력을 설명한다.
4. 반환 참조의 수명과 동률 선택 계약을 연결한다.

**개념 지도:** 제네릭은 타입별 중복을 줄이고, 트레이트는 공유 동작 계약을 정의한다. 바운드는 제네릭 코드가 사용할 수 있는 능력을 제한한다. 수명 표기는 참조가 얼마나 오래 사는지 늘리지 않고 입력과 출력 참조의 관계만 나타낸다. [제네릭](GLOSSARY.md#term-generic), [트레이트](GLOSSARY.md#term-trait), [트레이트 바운드](GLOSSARY.md#term-trait-bound), [수명](GLOSSARY.md#term-lifetime)을 참고한다.

**소스와 읽기 순서:** [`chapter10/src/main.rs`](chapter10/src/main.rs)에서 `Point`, `Summary`, `Article`, `largest`, `longest`, `notify`, `pair_summary`, 테스트 순서로 읽는다.

**실행 관찰:** 기본 트레이트 메서드의 미리보기, 두 기사 요약, 서로 다른 좌표 타입의 조합, 최댓값 `Some(9)`, 더 긴 문자열이 출력된다.

**테스트 우선 실습:** `longest_when_references_have_equal_lengths`를 작성해 동률이면 왼쪽의 바로 그 참조를 반환하게 한다. RED에서는 비교를 `left.len() > right.len()`으로 바꾼다. GREEN에서는 `>=`로 복구하고 `std::ptr::eq` 검사까지 통과시킨다.

```console
cargo test -p chapter10 longest_when_references_have_equal_lengths
cargo test -p chapter10
```

**성찰 질문:** `largest`가 값을 복사하지 않고 참조를 반환하는 장점은 무엇인가? `Summary`의 기본 메서드는 구현자에게 무엇을 제공하는가? `longest`의 수명 표기가 없다면 컴파일러가 어떤 관계를 알 수 없는가?

<a id="chapter-11"></a>
## 11장. 자동화 테스트 작성하기

공식 문서: [Writing Automated Tests](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)

**선수 지식:** 함수, 모듈, `Result`, 패닉.

**학습 목표**

1. `#[test]`와 `#[cfg(test)]`가 테스트 코드를 구성하는 방식을 설명한다.
2. `assert!`, `assert_eq!`, `assert_ne!`의 의도를 구분한다.
3. `#[should_panic(expected = ...)]`로 패닉 계약을 좁힌다.
4. `Result<(), E>`를 반환하는 테스트와 경계값 테스트를 작성한다.

**개념 지도:** 테스트는 준비, 실행, 검증의 작은 계약이다. 성공 경로는 값 비교로, 의도된 패닉은 메시지 일부로, 여러 실패 가능성은 `Result` 반환으로 표현할 수 있다. 경계 `100`은 허용되고 `101`은 패닉이라는 두 테스트가 범위를 닫는다. [단위 테스트](GLOSSARY.md#term-unit-test), [어설션](GLOSSARY.md#term-assertion), [`should_panic`](GLOSSARY.md#term-should-panic), [경계값](GLOSSARY.md#term-boundary-value)을 참고한다.

**소스와 읽기 순서:** [`chapter11/src/main.rs`](chapter11/src/main.rs)의 `add_two`, `Guess::new`, `main`, 일반 테스트, 패닉 테스트, `Result` 테스트, 상한 테스트 순서로 읽는다.

**실행 관찰:** 실행하면 `add_two(40) = 42`와 `Guess::new(42)`가 유효하다는 문장이 나온다. 테스트에서는 `101`의 패닉이 성공으로 집계되고 `100`은 정상 생성된다.

**테스트 우선 실습:** `guess_new_when_value_is_upper_boundary`를 먼저 작성한다. RED에서는 생성자 조건을 임시로 `value < 100`으로 바꿔 `100`에서 패닉을 확인한다. GREEN에서는 `value <= 100`으로 복구한다.

```console
cargo test -p chapter11 guess_new_when_value_is_upper_boundary
cargo test -p chapter11
```

**성찰 질문:** 패닉 여부만 검사하지 않고 메시지도 검사하는 이유는 무엇인가? 경계 바로 안쪽과 바깥쪽을 함께 테스트하면 무엇이 명확해지는가? `Result` 반환 테스트에서는 `?`를 어떻게 쓸 수 있는가?

<a id="chapter-12"></a>
## 12장. I/O 프로젝트, 미니그렙 만들기

공식 문서: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

**선수 지식:** 문자열 슬라이스, 수명, `Result`, 반복자와 클로저 기초.

**학습 목표**

1. 입력 파싱과 검색 로직을 작은 함수로 분리한다.
2. 패턴 매칭으로 명령 형태의 메모리 입력을 검증한다.
3. 대소문자 구분 검색과 무시 검색의 차이를 테스트한다.
4. 외부 I/O 없이 결정적으로 재현되는 미니그렙 경계를 이해한다.

**개념 지도:** 이 구현은 Book의 CLI 구조를 학습하되 프로세스 인수, 파일, 환경 변수, 네트워크를 읽지 않는다. `main`이 코드에 든 `query`, `contents`, 옵션을 `Config::parse`에 주입한다. 따라서 모든 실행과 테스트가 같은 입력, 같은 순서, 같은 결과를 낸다. 파싱과 검색은 `parse_and_search`에서 `?`로 조합된다. [결정성](GLOSSARY.md#term-determinism), [의존성 주입](GLOSSARY.md#term-dependency-injection), [구성](GLOSSARY.md#term-config), [순수 함수](GLOSSARY.md#term-pure-function)를 참고한다.

**소스와 읽기 순서:** [`chapter12/src/main.rs`](chapter12/src/main.rs)의 `ConfigError`, `Config::parse`, `search`, `search_case_insensitive`, `parse_and_search`, 주입된 `sample`, 테스트 순서로 읽는다.

**실행 관찰:** 첫 줄은 메모리 입력만 쓴다고 밝힌다. 쿼리 `duct`와 `--ignore-case`로 `safe, fast, productive.`와 `Duct tape.` 두 줄이 원문 순서대로 출력된다.

**테스트 우선 실습:** `search_when_query_has_no_match`를 먼저 작성해 일치가 없으면 빈 벡터를 반환하게 한다. RED에서는 `search` 뒤에 임시 기본 줄을 추가해 실패를 만든다. GREEN에서는 필터 결과만 수집하도록 복구한다.

```console
cargo test -p chapter12 search_when_query_has_no_match
cargo test -p chapter12
```

**성찰 질문:** 실제 파일을 읽지 않는 설계가 학습 테스트에 주는 이점은 무엇인가? `Config<'text>`의 두 참조는 어떤 입력에 묶이는가? 대소문자 무시 검색에서 새 `String` 할당은 어디서 생기는가?

<a id="chapter-13"></a>
## 13장. 함수형 기능, 반복자와 클로저

공식 문서: [Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html)

**선수 지식:** 소유권과 이동, 제네릭, 트레이트, `Option`.

**학습 목표**

1. 환경을 불변 빌림, 가변 빌림, 이동으로 캡처하는 클로저를 구분한다.
2. `Fn`, `FnMut`, `FnOnce`의 호출 가능성 차이를 설명한다.
3. `filter`, `map`, `collect`를 지연 반복자 파이프라인으로 읽는다.
4. `Iterator::next`의 종료 계약을 직접 구현하고 검증한다.

**개념 지도:** 클로저는 주변 환경을 캡처할 수 있는 호출 가능 값이다. 캡처 방식에 따라 구현되는 호출 트레이트가 달라진다. 반복자 어댑터는 소비되기 전까지 지연되며, `next`는 값이 끝나면 계속 `None`을 반환해야 한다. [클로저](GLOSSARY.md#term-closure), [`Fn` 계열](GLOSSARY.md#term-fn-traits), [반복자](GLOSSARY.md#term-iterator), [지연 평가](GLOSSARY.md#term-lazy)을 참고한다.

**소스와 읽기 순서:** [`chapter13/src/main.rs`](chapter13/src/main.rs)의 `doubled_evens`, 세 `call_fn` 함수, `closure_summary`, `Counter`의 `Iterator` 구현, 테스트 순서로 읽는다.

**실행 관찰:** 클로저 결과는 `42`, `[1, 2]`, 이동된 문자열이다. 짝수 필터와 두 배 매핑은 `[4, 8]`, 카운터는 `[1, 2, 3]`을 만든다.

**테스트 우선 실습:** `counter_when_end_is_one_stays_exhausted`를 먼저 작성해 호출 결과가 `[Some(1), None, None]`인지 검사한다. RED에서는 종료 분기에서 `Some(self.current)`를 반환하게 한다. GREEN에서는 종료 뒤 항상 `None`을 반환한다.

```console
cargo test -p chapter13 counter_when_end_is_one_stays_exhausted
cargo test -p chapter13
```

**성찰 질문:** `move` 클로저가 `FnOnce`가 되는 조건은 무엇인가? 어댑터만 만들고 소비하지 않으면 왜 계산이 일어나지 않는가? 소진된 반복자가 다시 값을 내놓으면 어떤 소비자 계약이 깨지는가?

<a id="chapter-14"></a>
## 14장. Cargo와 Crates.io 더 알아보기

공식 문서: [More About Cargo and Crates.io](https://doc.rust-lang.org/stable/book/ch14-00-more-about-cargo.html)

**선수 지식:** 07장의 패키지와 크레이트, 11장의 테스트.

**학습 목표**

1. 한 패키지의 라이브러리 대상과 바이너리 대상을 구분한다.
2. `///` 문서 주석의 예제가 문서 테스트로 컴파일되고 실행됨을 확인한다.
3. 공개 모듈의 항목을 크레이트 루트에 재노출한다.
4. 개발 및 릴리스 프로필, `cargo doc`, `cargo install`, 사용자 정의 명령의 용도를 설명한다.

**개념 지도:** `chapter14/src/lib.rs`는 `math::add_one`과 루트 재노출을 제공하는 라이브러리 크레이트다. `chapter14/src/main.rs`는 그 API를 소비하는 바이너리 크레이트다. `cargo test -p chapter14`는 라이브러리 단위 테스트, 바이너리 테스트 대상, 문서 주석의 doctest를 함께 다룬다. [라이브러리 크레이트](GLOSSARY.md#term-library-crate), [바이너리 크레이트](GLOSSARY.md#term-binary-crate), [문서 테스트](GLOSSARY.md#term-doctest), [프로필](GLOSSARY.md#term-profile)을 참고한다.

**소스와 읽기 순서**

1. [`chapter14/src/lib.rs`](chapter14/src/lib.rs)의 문서 주석, `math`, `pub use`, 단위 테스트를 읽는다.
2. [`chapter14/src/main.rs`](chapter14/src/main.rs)가 `use chapter14::add_one`으로 라이브러리를 소비하는 방식을 본다.
3. `cargo test -p chapter14 --doc`으로 문서 예제만 따로 확인한다.

**실행 관찰:** 실행 결과는 프로필, 문서, 설치, 워크스페이스, 사용자 정의 명령을 설명하고 `chapter14::add_one(41) = 42`를 보여 준다. 문서 테스트는 예제의 같은 결과를 검증한다.

**테스트 우선 실습:** `add_one_when_value_is_negative_two`를 라이브러리에 먼저 작성한다. RED에서는 `add_one`이 음수에 `0`을 반환하도록 임시 분기를 넣는다. GREEN에서는 모든 `i32`에 `value + 1`을 적용한다.

```console
cargo test -p chapter14 add_one_when_value_is_negative_two
cargo test -p chapter14
cargo test -p chapter14 --doc
```

**성찰 질문:** 같은 패키지의 `main.rs`가 왜 `chapter14::add_one` 경로를 쓰는가? 문서 테스트는 일반 단위 테스트와 어떤 독자를 위한 계약인가? 재노출이 사용자 경로를 어떻게 단순화하는가?

<a id="chapter-15"></a>
## 15장. 스마트 포인터

공식 문서: [Smart Pointers](https://doc.rust-lang.org/stable/book/ch15-00-smart-pointers.html)

**선수 지식:** 소유권, 재귀 열거형, 트레이트, 참조.

**학습 목표**

1. `Box<T>`가 재귀 타입에 필요한 고정 크기 간접 참조를 제공함을 설명한다.
2. `Deref`와 `Drop` 트레이트의 동작을 관찰한다.
3. `Rc<T>`의 공유 소유권과 `RefCell<T>`의 런타임 빌림 검사를 구분한다.
4. `Weak<T>`가 참조 순환 없이 비소유 관계를 나타내는 방법을 확인한다.

**개념 지도:** 스마트 포인터는 포인터처럼 참조하면서 추가 소유권 규칙과 동작을 제공한다. `Box`는 단일 소유, `Rc`는 단일 스레드 공유 소유, `RefCell`은 내부 가변성, `Weak`는 강한 소유권 없는 연결을 맡는다. [`Box`](GLOSSARY.md#term-box), [`Deref`](GLOSSARY.md#term-deref), [`Drop`](GLOSSARY.md#term-drop), [`Rc`](GLOSSARY.md#term-rc), [`RefCell`](GLOSSARY.md#term-refcell), [`Weak`](GLOSSARY.md#term-weak)을 참고한다.

**소스와 읽기 순서:** [`chapter15/src/main.rs`](chapter15/src/main.rs)의 `List`, `SmallBox`, `DropRecorder`, `rc_counts`, `refcell_value`, `Node`, `weak_parent_lifecycle`, 테스트 순서로 읽는다.

**실행 관찰:** 재귀 목록 합 `3`, 역참조 값 `15`, 역순 drop 기록, 강한 참조 수 `[1, 2, 3, 2]`, 내부 변경값 `15`, 부모 생존 상태 `[true, false]`가 나온다.

**테스트 우선 실습:** `recursive_list_sums_three_nodes`를 먼저 작성해 세 노드의 합 `6`을 요구한다. RED에서는 `sum`이 현재 노드만 반환하도록 바꾼다. GREEN에서는 `value + next.sum()` 재귀를 복구한다.

```console
cargo test -p chapter15 recursive_list_sums_three_nodes
cargo test -p chapter15
```

**성찰 질문:** `Box`가 없으면 `List`의 크기를 계산할 수 없는 이유는 무엇인가? `RefCell`은 빌림 오류를 언제 찾는가? 부모 링크에 `Rc` 대신 `Weak`를 쓰는 이유는 무엇인가?

<a id="chapter-16"></a>
## 16장. 겁 없는 동시성

공식 문서: [Fearless Concurrency](https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html)

**선수 지식:** 이동 클로저, `Result`, 스마트 포인터, 트레이트.

**학습 목표**

1. `move` 클로저로 스레드에 소유권을 넘기고 `join`으로 결과를 회수한다.
2. 다중 생산자 단일 소비자 채널의 종료 조건을 설명한다.
3. `Arc<Mutex<T>>`로 공유 상태를 안전하게 갱신한다.
4. `Send`와 `Sync`를 컴파일 시간 속성으로 확인한다.

**개념 지도:** 스레드는 독립 실행 흐름이다. 채널은 메시지 전달, `Arc<Mutex<_>>`는 동기화된 공유 상태를 나타낸다. 예제는 스케줄 순서가 달라도 메시지를 정렬해 관찰 결과를 결정적으로 만든다. [`thread`](GLOSSARY.md#term-thread), [채널](GLOSSARY.md#term-channel), [`Arc`](GLOSSARY.md#term-arc), [`Mutex`](GLOSSARY.md#term-mutex), [`Send`와 `Sync`](GLOSSARY.md#term-send-sync)을 참고한다.

**소스와 읽기 순서:** [`chapter16/src/main.rs`](chapter16/src/main.rs)의 `LessonError`, `moved_thread_sum`, `join_workers`, `collect_messages`, `increment_shared`, `prove_send_sync`, `run_lesson`, 테스트 순서로 읽는다.

**실행 관찰:** 이동된 값의 합 `10`, 정렬된 메시지 `[1, 2, 3, 4]`, 작업자 네 명의 카운터 `4`, 작업자 없는 카운터 `0`이 출력된다. 성공 시 프로세스 종료 코드는 성공이다.

**테스트 우선 실습:** `one_worker_increments_shared_counter_once`를 먼저 작성한다. RED에서는 작업자 루프를 `0..worker_count.saturating_sub(1)`로 잠시 줄인다. GREEN에서는 정확히 `0..worker_count`만큼 생성한다.

```console
cargo test -p chapter16 one_worker_increments_shared_counter_once
cargo test -p chapter16
```

**성찰 질문:** 마지막 원본 송신자를 drop해야 수신 반복이 끝나는 이유는 무엇인가? `Rc<RefCell<_>>` 대신 `Arc<Mutex<_>>`가 필요한 까닭은 무엇인가? 정렬은 동시성 테스트의 어떤 비결정성을 제거하는가?

<a id="chapter-17"></a>
## 17장. 비동기 프로그래밍의 기초

공식 문서: [Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/stable/book/ch17-00-async-await.html)

**선수 지식:** 클로저, 반복자, `Option`, 동시성의 실행 흐름.

**학습 목표**

1. `async fn`이 즉시 결과가 아니라 future를 만든다는 점을 설명한다.
2. `.await`로 future와 stream의 진행 지점을 읽는다.
3. `join`으로 준비된 future 둘을 결합하고 `block_on`으로 실행한다.
4. 비동기 stream을 순서대로 소비해 조건에 맞는 값만 모은다.

**개념 지도:** future는 나중에 준비될 값을 나타내고 executor가 poll해야 진행된다. stream은 여러 비동기 값을 낸다. 이 장만 [`chapter17/Cargo.toml`](chapter17/Cargo.toml)에 외부 `futures` 의존성을 직접 선언한다. 다른 패키지는 이를 상속하지 않으며, 의존성은 패키지 로컬이다. [future](GLOSSARY.md#term-future), [`async`와 `await`](GLOSSARY.md#term-async-await), [executor](GLOSSARY.md#term-executor), [stream](GLOSSARY.md#term-stream), [패키지 로컬 의존성](GLOSSARY.md#term-package-local-dependency)을 참고한다.

**소스와 읽기 순서**

1. [`chapter17/Cargo.toml`](chapter17/Cargo.toml)의 `futures` 선언을 확인한다.
2. [`chapter17/src/main.rs`](chapter17/src/main.rs)의 import, `joined_total`, `collect_even`, `block_on` 블록, 테스트 순서로 읽는다.

**실행 관찰:** 두 입력을 각각 두 배로 만든 뒤 보너스 `1`을 더한 `joined total: 11`, stream에서 모은 `streamed even values: [2, 4]`가 출력된다.

**테스트 우선 실습:** `all_odd_stream_collects_no_even_values`를 먼저 작성한다. RED에서는 조건 없이 모든 값을 `collected`에 넣는다. GREEN에서는 `value.is_multiple_of(2)`일 때만 넣는다.

```console
cargo test -p chapter17 all_odd_stream_collects_no_even_values
cargo test -p chapter17
```

**성찰 질문:** future를 만들기만 하고 poll하지 않으면 왜 본문이 실행되지 않는가? `join`과 스레드 생성은 같은 개념인가? `futures`를 루트가 아닌 `chapter17`에만 둔 설계가 의존성 경계를 어떻게 드러내는가?

<a id="chapter-18"></a>
## 18장. Rust의 객체 지향 기능

공식 문서: [Object-Oriented Programming Features of Rust](https://doc.rust-lang.org/stable/book/ch18-00-oop.html)

**선수 지식:** 구조체와 메서드, `Option`, 트레이트, 스마트 포인터.

**학습 목표**

1. 비공개 상태와 공개 메서드로 불변 조건을 캡슐화한다.
2. 캐시된 평균이 추가와 제거 뒤 함께 갱신되는지 검증한다.
3. `dyn Trait` 트레이트 객체와 제네릭 정적 디스패치를 구분한다.
4. 서로 다른 구체 타입을 `Vec<Box<dyn Draw>>`에 저장한다.

**개념 지도:** 캡슐화는 상태 변경 통로를 메서드로 제한해 `values`와 `average`를 동기화한다. 트레이트 객체는 공통 트레이트를 구현한 서로 다른 크기의 값을 포인터 뒤에 두고 런타임에 메서드를 고른다. [캡슐화](GLOSSARY.md#term-encapsulation), [트레이트 객체](GLOSSARY.md#term-trait-object), [동적 디스패치](GLOSSARY.md#term-dynamic-dispatch), [객체 안전성](GLOSSARY.md#term-object-safety)을 참고한다.

**소스와 읽기 순서:** [`chapter18/src/main.rs`](chapter18/src/main.rs)의 `AveragedCollection`, 비공개 `update_average`, `Draw`, 두 구현 타입, `Screen`, 테스트 순서로 읽는다.

**실행 관찰:** 값 `10`, `20`의 평균은 `Some(15.0)`이고 `20`을 제거한 뒤 `Some(10.0)`이다. 화면은 서로 다른 타입을 동적 디스패치해 `["Save", "Rust"]`를 렌더링한다.

**테스트 우선 실습:** `updates_remaining_average_when_one_of_multiple_values_is_removed`를 먼저 작성해 `30` 제거 후 평균 `15.0`을 요구한다. RED에서는 `remove` 안의 `update_average()` 호출을 잠시 제거한다. GREEN에서는 제거 직후 캐시를 다시 계산한다.

```console
cargo test -p chapter18 updates_remaining_average_when_one_of_multiple_values_is_removed
cargo test -p chapter18
```

**성찰 질문:** `values`와 `average`가 공개 필드라면 어떤 불일치가 생길 수 있는가? `Vec<Button>`으로는 이 예제의 무엇을 표현할 수 없는가? 트레이트 객체가 정적 디스패치보다 치르는 비용은 무엇인가?

<a id="chapter-19"></a>
## 19장. 패턴과 매칭

공식 문서: [Patterns and Matching](https://doc.rust-lang.org/stable/book/ch19-00-patterns.html)

**선수 지식:** 열거형과 `match`, 구조체, 반복자, `Option`.

**학습 목표**

1. 구조체, 튜플, 열거형을 패턴으로 분해한다.
2. 범위, OR 패턴, 매치 가드, `@` 바인딩의 평가 순서를 설명한다.
3. `if let`, `while let`, `for`의 패턴 위치를 찾는다.
4. 구체적인 패턴을 일반 패턴보다 앞에 두고 빠짐없이 분류한다.

**개념 지도:** 패턴은 값의 구조와 조건을 동시에 검사하고 내부 값을 이름에 묶는다. `classify_point`는 원점을 먼저 잡고, 범위와 `@` 및 가드로 작은 대각선을 찾고, OR 패턴으로 두 축을 합친다. 마지막 패턴이 나머지를 처리한다. [패턴](GLOSSARY.md#term-pattern), [구조 분해](GLOSSARY.md#term-destructuring), [OR 패턴](GLOSSARY.md#term-or-pattern), [매치 가드](GLOSSARY.md#term-match-guard), [`@` 바인딩](GLOSSARY.md#term-at-binding)을 참고한다.

**소스와 읽기 순서:** [`chapter19/src/main.rs`](chapter19/src/main.rs)의 `Point`, `Message`, `classify_point`, `pair_total`, `sum_pairs`, `drain_stack`, `describe_message`, 테스트 순서로 읽는다.

**실행 관찰:** 작은 대각선, 원점, 축, 아래쪽 분류가 각각 출력된다. 튜플 합은 `10`, 스택은 역순으로 비워져 마지막 관찰값이 `1`, 모든 메시지 배리언트는 담긴 데이터와 함께 설명된다.

**테스트 우선 실습:** `classifies_origin_when_both_coordinates_are_zero`와 `classifies_axis_when_exactly_one_coordinate_is_zero`를 먼저 작성한다. RED에서는 `(0, 0)` 분기를 축 분기 아래로 옮겨 원점이 `axis`가 되는 것을 본다. GREEN에서는 더 구체적인 원점 패턴을 먼저 두고 두 축은 `(0, _) | (_, 0)`으로 처리한다.

```console
cargo test -p chapter19 classifies_origin_when_both_coordinates_are_zero
cargo test -p chapter19 classifies_axis_when_exactly_one_coordinate_is_zero
cargo test -p chapter19
```

**성찰 질문:** 패턴 순서가 원점 결과를 바꾸는 이유는 무엇인가? 매치 가드는 빠짐없음 검사에 어떤 제한을 주는가? `while let Some(value)`가 스택이 빌 때 자연스럽게 끝나는 원리는 무엇인가?

## 완주 점검

각 장의 성찰 질문에 자신의 말로 답하고 다음 명령을 모두 통과시키면 한 차례 학습을 마친 것이다.

```console
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets -- -D warnings
bash scripts/check-all-chapters.sh
```

다음 회차에는 테스트 이름만 읽고 구현 계약을 예측해 본다. 그 뒤 경계값 하나를 스스로 추가하고 RED, GREEN, 리팩터 순서를 기록한다.
