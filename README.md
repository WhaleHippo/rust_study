# Rust 개념 지도

## 이 문서의 목적

이 문서는 Rust를 익힐 때 필요한 개념을 **서로의 선수 관계**에 따라 정리한 지도다. 문법 목록만 외우는 대신, 소유권이 왜 빌림과 수명으로 이어지는지, trait가 왜 제네릭과 동시성의 토대가 되는지, 안전한 추상화가 어떤 메모리 불변 조건 위에 서는지를 연결해서 보는 데 목적이 있다.

처음부터 모든 세부 사항을 완벽히 이해할 필요는 없다. 각 단계에서 다음 세 가지를 구분하며 읽으면 좋다.

1. **설명할 수 있는가:** 개념의 뜻과 필요한 이유를 자기 말로 표현한다.
2. **코드에서 알아보는가:** 컴파일러 오류나 라이브러리 API에서 개념을 식별한다.
3. **설계에 적용하는가:** 여러 선택지 중 상황에 맞는 타입과 경계를 고른다.

앞부분은 뒷부분의 전제다. 소유권을 모른 채 `Arc<Mutex<T>>`를 외우거나, `Future`를 모른 채 비동기 런타임 사용법만 익히면 문제가 생겼을 때 원인을 찾기 어렵다. 이미 아는 주제는 [학습 주제 및 완료 추적표](topics.md)에서 확인하고, 막히는 용어가 나오면 연결된 앞 단계로 돌아가면 된다.

> **표기 원칙:** 별도 표시가 없는 내용은 stable Rust에서 배울 수 있는 개념이다. **고급**은 사용 빈도보다 이해 난도가 높은 주제다. **nightly 또는 불안정** 표시는 현재 안정 채널에서 일반적으로 쓸 수 없거나 세부 상태를 공식 문서에서 다시 확인해야 하는 기능을 뜻한다. Rust는 계속 발전하므로 실제 사용 전에는 해당 API와 기능의 안정화 상태를 확인한다.

## 목차

1. [전체 선수 관계](#전체-선수-관계)
2. [도구 체인과 컴파일러 모델](#1-도구-체인과-컴파일러-모델)
3. [문법, 바인딩, 타입, 함수, 제어 흐름](#2-문법-바인딩-타입-함수-제어-흐름)
4. [소유권, 빌림, 슬라이스, 수명, 소멸](#3-소유권-빌림-슬라이스-수명-소멸)
5. [문자열, UTF-8, 컬렉션](#4-문자열-utf-8-컬렉션)
6. [구조체, 열거형, Option, Result, 패턴](#5-구조체-열거형-option-result-패턴)
7. [모듈, 크레이트, 가시성, Cargo](#6-모듈-크레이트-가시성-cargo)
8. [제네릭, trait, 연관 항목, 다형성](#7-제네릭-trait-연관-항목-다형성)
9. [클로저, 반복자, 변환](#8-클로저-반복자-변환)
10. [스마트 포인터, 내부 가변성, Pin](#9-스마트-포인터-내부-가변성-pin)
11. [오류 설계](#10-오류-설계)
12. [테스트, 문서, 품질 도구](#11-테스트-문서-품질-도구)
13. [스레드와 동시성](#12-스레드와-동시성)
14. [비동기 프로그래밍](#13-비동기-프로그래밍)
15. [매크로와 조건부 컴파일](#14-매크로와-조건부-컴파일)
16. [unsafe, 메모리 불변 조건, FFI](#15-unsafe-메모리-불변-조건-ffi)
17. [고급 타입 시스템](#16-고급-타입-시스템)
18. [성능과 프로덕션 운영](#17-성능과-프로덕션-운영)
19. [프로젝트 진행 방향](#18-프로젝트-진행-방향)
20. [공식 참고 자료](#공식-참고-자료)

## 전체 선수 관계

개념의 큰 흐름은 다음과 같다.

```text
도구 체인과 컴파일 모델
  -> 기본 문법과 타입
  -> 소유권, 빌림, 수명, Drop
  -> 문자열, 컬렉션, 데이터 모델링
  -> 모듈과 Cargo
  -> 제네릭과 trait
  -> 반복자, 스마트 포인터, 오류 설계
  -> 테스트와 품질
  -> 동시성 기초 -> 비동기
  -> 매크로, unsafe, 고급 타입 시스템
  -> 성능, 배포, 호환성, 보안
```

이 흐름은 한 방향으로만 진행되지는 않는다. 예를 들어 컬렉션을 쓰며 소유권을 다시 배우고, 동시성을 공부하며 `Send`, `Sync`, 내부 가변성의 의미를 새로 이해하게 된다. 중요한 것은 현재 문제를 어느 층의 개념으로 설명해야 하는지 아는 것이다.

## 1. 도구 체인과 컴파일러 모델

**선수 지식:** 명령줄에서 프로그램을 실행하고 파일 경로를 다루는 기초

### 도구의 역할

* `rustup`: toolchain, target, component를 설치하고 채널을 관리한다.
* `rustc`: Rust 소스를 크레이트 단위로 컴파일한다.
* `cargo`: 패키지 생성, 의존성 해석, 빌드, 실행, 테스트, 문서 생성을 맡는다.
* stable, beta, nightly는 릴리스 채널이다. 일반 프로젝트의 기본 선택은 stable이며, nightly는 실험 기능이나 특정 도구가 꼭 필요할 때 범위를 제한해 쓴다.
* edition은 언어가 호환되는 범위 안에서 문법과 관용구를 발전시키는 장치다. edition과 컴파일러 버전은 같은 개념이 아니다.

### 컴파일을 보는 관점

Rust의 기본 컴파일 단위는 **crate**다. 패키지는 하나 이상의 크레이트를 담을 수 있고, 크레이트는 라이브러리 또는 실행 파일이 된다. 컴파일 과정에서는 이름과 타입을 확인하고, borrow checker가 참조의 유효성을 검증하며, 제네릭 코드는 흔히 구체 타입별로 단형화된다. 이후 최적화와 코드 생성, 링크가 이어진다.

`cargo check`는 실행 파일을 완성하는 비용을 줄이면서 많은 오류를 빠르게 찾는다. `cargo build`는 산출물을 만들고, `cargo test`는 테스트 하네스를 컴파일해 실행한다. debug와 release profile은 최적화 수준뿐 아니라 오버플로 검사 같은 동작 차이도 만들 수 있으므로 둘 다 관찰할 가치가 있다.

컴파일러 오류는 단순한 거부가 아니라 타입, 이동, 빌림이 어디에서 충돌했는지 보여 주는 설계 피드백이다. 오류 메시지의 첫 줄만 고치기보다 원래 값의 소유자와 필요한 사용 기간을 먼저 그려 보는 습관이 중요하다.

### 식과 장소

Rust에서 대부분의 구문은 값을 만드는 **expression**이다. `if`, `match`, 블록도 값을 낼 수 있다. 반면 **place expression**은 메모리상의 위치를 가리킨다. 변수, `value.field`, `items[index]`, `*pointer`가 대표적이다. 대입, 이동, 빌림은 어떤 place에서 값을 읽거나 그 place를 참조하는 동작으로 이해하면 소유권 오류가 선명해진다.

```rust
let answer = {
    let base = 40;
    base + 2 // 세미콜론이 없으므로 블록의 값이다.
};

let mut pair = (answer, 0);
pair.1 = pair.0; // pair.0과 pair.1은 place expression이다.
```

**흔한 오해:** Rust는 모든 것을 컴파일 시점에 결정하지 않는다. 경계 검사, 동적 디스패치, 할당, 잠금처럼 실행 시점 비용과 검사가 필요한 작업도 있다.

## 2. 문법, 바인딩, 타입, 함수, 제어 흐름

**선수 지식:** 1단계

### 바인딩과 가변성

`let`은 이름을 값에 바인딩한다. 바인딩은 기본적으로 불변이며 `mut`를 붙여야 같은 place의 값을 바꿀 수 있다. **shadowing**은 같은 이름으로 새 바인딩을 만드는 것이므로 `mut`와 다르다. 타입도 바꿀 수 있고 이전 빌림을 끝내는 데 영향을 줄 수도 있다.

```rust
let input = " 42 ";
let input = input.trim();
let input: u32 = input.parse().expect("숫자여야 합니다");

let mut count = 0;
count += 1;
```

상수 `const`는 타입 표기가 필요하고 상수 문맥에서 계산된다. `static`은 프로그램 전체 수명의 저장 위치를 가지며, 변경 가능한 `static mut`는 안전성 문제가 크므로 피해야 한다.

### 기본 타입과 조합 타입

* 정수는 `i8`부터 `i128`, `u8`부터 `u128`, 그리고 포인터 크기를 따르는 `isize`, `usize`가 있다.
* 부동소수점은 `f32`, `f64`이며 IEEE 754의 반올림과 `NaN` 특성을 가진다.
* `bool`, Unicode scalar value를 담는 `char`, 값이 하나뿐인 unit 타입 `()`가 있다.
* 배열 `[T; N]`은 길이가 타입에 포함된 연속 저장소다. 튜플 `(A, B, C)`은 서로 다른 타입을 한 값으로 묶는다.
* 범위 `start..end`, `start..=end`는 반복과 슬라이싱에 쓰이며 끝 포함 여부가 다르다.

산술, 비교, 논리, 비트, 대입 연산자를 익히되 타입 변환이 자동으로 넓어질 것이라 기대하면 안 된다. `as`는 저수준 숫자 캐스트에 가깝고 값 손실 가능성이 있다. 의미 있는 변환은 뒤에서 다룰 `From`과 `TryFrom`이 더 잘 표현한다. 연산자 대부분은 trait로 연결되므로 사용자 타입도 제한된 방식으로 동작을 정의할 수 있다.

### 함수와 제어 흐름

함수는 `fn name(parameter: Type) -> ReturnType` 형태로 선언한다. 마지막 expression이 반환값이며 `return`은 중간 종료에 쓴다. 함수 항목은 캡처가 없는 함수 포인터 `fn(A) -> B`로 강제 변환될 수 있다.

`if`의 모든 가지는 호환되는 타입의 값을 내야 한다. `loop`는 `break value`로 값을 반환할 수 있고, `while`은 조건 반복, `for`는 `IntoIterator` 기반 반복이다. 중첩 반복에는 loop label을 붙여 바깥 반복을 명시적으로 `break`하거나 `continue`할 수 있다.

```rust
fn classify(number: i32) -> &'static str {
    if number < 0 { "negative" } else { "non-negative" }
}

'outer: for row in 0..3 {
    for column in 0..3 {
        if row == column {
            continue 'outer;
        }
        println!("{row}, {column}");
    }
}
```

**흔한 오해:** 세미콜론은 단순한 줄 끝 표시가 아니다. expression 뒤에 붙으면 그 값을 버리고 `()`로 만든다.

## 3. 소유권, 빌림, 슬라이스, 수명, 소멸

**선수 지식:** 2단계의 바인딩, 타입, place expression

### 소유권과 이동

각 값에는 소유자가 있고, 소유자가 scope를 벗어나면 값이 소멸한다. 대입이나 함수 인수 전달은 타입에 따라 값을 복사하거나 이동한다. 이동된 원래 바인딩은 더 이상 사용할 수 없다. 이 규칙은 garbage collector 없이 이중 해제와 use after free를 막는 기반이다.

`Copy` 타입은 단순한 비트 복사가 유효하고 소멸 로직이 필요 없는 작은 값에 적합하다. `Clone`은 명시적 복제 연산을 제공하며 힙 할당 같은 비용이 들 수 있다. 모든 `Copy`는 `Clone`이지만 모든 `Clone`이 `Copy`는 아니다. `Drop`을 구현한 타입은 `Copy`가 될 수 없다.

```rust
let first = String::from("rust");
let second = first;       // 소유권 이동
let third = second.clone(); // 명시적 깊은 복제가 가능하다.

let x = 10;
let y = x; // i32는 Copy이므로 x도 계속 쓸 수 있다.
```

### 빌림과 재빌림

참조 `&T`는 공유 빌림, `&mut T`는 배타 빌림이다. 같은 데이터의 겹치는 부분에는 한 시점에 여러 활성 공유 참조가 있거나 하나의 활성 가변 참조가 있을 수 있지만, 충돌하는 공유 접근과 가변 접근은 겹칠 수 없다. 서로 겹치지 않음이 증명된 place는 각각 따로 가변 빌림할 수 있다. 이 규칙은 별칭과 변경의 충돌을 막는다.

가변 참조를 다른 함수에 넘길 때는 원래 참조 자체를 영구히 이동하는 대신 더 짧은 **reborrow**가 만들어지는 경우가 많다.

```rust
fn increment(value: &mut i32) {
    *value += 1;
}

let mut number = 0;
let reference = &mut number;
increment(&mut *reference); // 짧은 재빌림
increment(reference);       // 앞 재빌림이 끝났으므로 다시 쓸 수 있다.
```

non lexical lifetimes 덕분에 빌림은 보통 lexical scope 끝이 아니라 마지막 사용 뒤에 끝난다. 그러나 참조가 가리키는 값보다 오래 살아서는 안 된다는 핵심 규칙은 그대로다.

### 슬라이스와 수명

슬라이스 `&[T]`와 `&str`은 소유하지 않은 연속 구간의 view다. 포인터와 길이로 이루어진 fat pointer이며, 원본 일부를 복사 없이 빌린다.

수명 표기 `'a`는 여러 참조의 유효 기간 관계를 타입에 드러낸다. **수명 표기는 값이나 참조의 실제 수명을 늘리지 않는다.** 컴파일러가 관계를 추론하지 못할 때 이미 존재하는 관계를 설명할 뿐이다. lifetime elision 규칙이 단순한 함수 시그니처의 표기를 생략해 준다. `'static` 참조는 프로그램 전체 동안 유효하다. 반면 타입에 `'static` bound가 붙었다고 해서 그 값이 프로그램 끝까지 살아 있다는 뜻은 아니다. 그 타입이 `'static`보다 짧은 수명의 참조를 포함하지 않는다는 뜻이며, 소유 데이터나 `'static` 참조는 포함할 수 있다.

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

### 부분 이동, binding mode, 소멸 범위

구조체의 일부 필드를 이동하면 나머지 필드는 계속 쓸 수 있지만 구조체 전체는 쓸 수 없는 **partial move**가 생길 수 있다. 패턴의 기본 binding mode는 대상이 값인지 참조인지에 따라 이동, 복사, 빌림을 결정한다. `ref`, `ref mut` 또는 패턴 앞의 참조 형태를 이해해야 예상치 못한 이동을 피할 수 있다.

값이 소멸되는 시점은 단순히 중괄호 끝만으로 결정되지 않는다. 지역 변수, 임시 값, 함수 인수마다 **drop scope**가 있으며, `if let`, `match`, 문장 안의 임시 값도 정해진 범위에서 소멸한다. 필요하면 `std::mem::drop`으로 소유 값을 일찍 소비해 자원을 해제할 수 있다.

### RAII와 Drop

RAII는 값의 생성과 자원 획득을, 값의 소멸과 자원 해제를 묶는다. 파일, 잠금 guard, 힙 메모리가 scope를 벗어날 때 자동 정리되는 이유다. `Drop::drop`은 직접 호출하지 않으며, 필드는 타입이 정한 순서에 따라 자동 소멸한다. panic 중에도 unwind가 선택된 환경에서는 이미 생성된 값의 `Drop`이 실행되지만, 프로세스 중단이나 abort에서는 그렇지 않을 수 있다.

**흔한 오해:** borrow checker는 메모리의 물리적 위치만 추적하는 도구가 아니다. 값의 이동, 별칭, 사용 기간을 타입과 제어 흐름 수준에서 검사한다.

## 4. 문자열, UTF-8, 컬렉션

**선수 지식:** 소유권, 빌림, 슬라이스

### `String`, `str`, `&str`

`str`은 유효한 UTF-8 바이트 시퀀스를 나타내는 동적 크기 타입이다. 보통 단독 값이 아니라 `&str`, `Box<str>`처럼 간접 참조로 쓴다. `String`은 소유권을 가진 확장 가능한 UTF-8 버퍼이고, `&str`은 문자열 데이터의 빌린 view다. 문자열 리터럴의 타입은 `&'static str`이다.

```rust
fn greet(name: &str) -> String {
    format!("안녕하세요, {name}!")
}

let owned = String::from("러스트");
let message = greet(&owned); // &String이 &str로 deref coercion된다.
```

Rust 문자열은 정수 인덱싱을 허용하지 않는다. UTF-8에서 한 글자의 바이트 수가 일정하지 않아 `text[0]`의 의미와 비용이 불명확하기 때문이다.

* `as_bytes()`와 `bytes()`는 원시 UTF-8 바이트를 본다.
* `chars()`는 Unicode scalar value를 순회한다. 화면에 보이는 글자 단위와 같지 않을 수 있다.
* 사용자에게 보이는 grapheme cluster는 표준 라이브러리가 직접 분할하지 않는다. 필요하다면 Unicode 분할을 다루는 외부 생태계 크레이트를 선택적으로 검토한다.
* 바이트 인덱스로 슬라이스할 때는 UTF-8 문자 경계여야 하며, 아니면 panic이 발생한다. `get(range)`는 실패를 `Option`으로 받는다.

파일 시스템 문자열은 항상 UTF-8이라고 가정할 수 없다. 경로는 `Path`와 `PathBuf`, 운영체제 문자열은 `OsStr`과 `OsString`으로 다루고, 표시나 직렬화가 필요한 경계에서 변환 정책을 정한다.

### 컬렉션 선택

* `Vec<T>`: 연속 메모리, 빠른 인덱싱과 끝 삽입이 필요한 기본 시퀀스다. capacity와 length는 다르다.
* `HashMap<K, V>`와 `BTreeMap<K, V>`: 각각 hash 기반 조회와 정렬된 키 순회를 제공한다.
* `HashSet<T>`와 `BTreeSet<T>`: 고유 원소와 집합 연산을 표현한다.
* `VecDeque<T>`: 앞뒤 삽입과 제거가 잦은 queue에 적합하다.
* `BinaryHeap<T>`: 우선순위가 가장 높은 값을 꺼내는 priority queue다.

인덱싱 `collection[index]`는 범위를 벗어나면 panic이 날 수 있다. `get`은 부재를 `Option`으로 표현한다. map의 `entry` API는 조회 후 삽입을 한 동작으로 표현해 중복 검색과 빌림 충돌을 줄인다.

컬렉션의 반복 방식은 소유권과 연결된다. `for value in collection`은 흔히 컬렉션을 소비하고, `for value in &collection`은 공유 빌림, `for value in &mut collection`은 가변 빌림을 사용한다.

**흔한 오해:** `String`은 문자 배열이 아니며 `char`도 사용자가 보는 글자 하나를 항상 뜻하지 않는다.

## 5. 구조체, 열거형, Option, Result, 패턴

**선수 지식:** 타입, 소유권, 문자열과 컬렉션

### 데이터 모델링

구조체는 관련 필드를 이름 있는 한 타입으로 묶는다. tuple struct는 구분되는 타입을 간결하게 만들고, unit struct는 데이터 없이 표지 역할을 한다. `impl` 블록 안의 **method**는 첫 인수로 `self`, `&self`, `&mut self` 등을 받고, **associated function**은 `self`를 받지 않아 `Type::new()`처럼 호출한다.

열거형 `enum`은 가능한 상태의 집합을 나타내며 variant마다 서로 다른 데이터를 담을 수 있다. bool과 서로 연관 없는 optional 필드로 상태를 표현하기보다, 유효한 조합만 variant로 만드는 편이 잘못된 상태를 줄인다.

```rust
enum Connection {
    Disconnected,
    Connecting { attempts: u8 },
    Connected(String),
}

impl Connection {
    fn is_ready(&self) -> bool {
        matches!(self, Self::Connected(_))
    }
}
```

`Option<T>`는 값의 존재와 부재를 `Some(T)`와 `None`으로 표현한다. `Result<T, E>`는 성공과 예상 가능한 실패를 `Ok(T)`와 `Err(E)`로 표현한다. null이나 마법 값보다 상태를 타입에 드러내며, 호출자가 빠뜨리지 않고 처리하게 한다.

### 패턴과 완전성

`match`는 모든 가능한 경우를 처리해야 한다. 이 **exhaustiveness**가 enum에 variant를 추가했을 때 수정할 지점을 찾아 준다. wildcard `_`는 값을 무시하지만, 지나치게 넓게 쓰면 새 상태를 조용히 삼킬 수 있다.

패턴은 `let`, 함수 매개변수, `match`, `if let`, `while let`, `for`에서도 쓰인다.

* destructuring은 구조체, 튜플, enum을 구성 요소로 나눈다.
* irrefutable pattern은 입력 가능한 모든 값과 맞아서 `let`과 함수 인수에 쓸 수 있다.
* refutable pattern은 실패할 수 있어 `match`, `if let`, `let else` 같은 문맥이 필요하다.
* match guard `if condition`은 패턴 뒤에 추가 조건을 붙인다.
* `binding @ pattern`은 범위를 검사하면서 전체 값을 이름에 묶는다.
* or pattern `A | B`는 여러 패턴이 같은 처리로 이어짐을 표현한다.

```rust
fn describe(value: Option<i32>) -> &'static str {
    match value {
        Some(number @ 1..=9) if number % 2 == 0 => "한 자리 짝수",
        Some(1..=9) => "한 자리 홀수",
        Some(10 | 20 | 30) => "선택된 십의 배수",
        Some(_) => "다른 숫자",
        None => "값 없음",
    }
}
```

패턴에 따라 필드가 이동되거나 빌려질 수 있다. `match &value`와 `match value`는 소유권 결과가 다르며, match ergonomics가 참조 패턴을 간결하게 해도 binding mode를 이해해야 한다.

**흔한 오해:** `if let`이 `match`보다 항상 더 간단한 것은 아니다. 여러 상태를 빠짐없이 처리해야 한다면 `match`의 완전성 검사가 더 안전하다.

## 6. 모듈, 크레이트, 가시성, Cargo

**선수 지식:** 함수와 사용자 정의 타입

### 코드 경계

package는 `Cargo.toml`이 설명하는 배포와 빌드 단위다. crate는 컴파일 단위이며 crate root에서 module tree가 시작된다. module은 이름 공간과 가시성 경계를 만든다. 파일과 모듈은 관련 있지만 같은 개념은 아니다.

경로는 `crate`, `self`, `super` 또는 외부 크레이트 이름에서 시작할 수 있다. 항목은 기본적으로 비공개다. `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`로 필요한 범위만 연다. 공개 함수가 비공개 타입을 노출하지 않도록 public API 전체의 가시성을 함께 생각해야 한다.

`use`는 경로를 scope로 가져오고, `pub use`는 다른 경로로 다시 공개한다. re-export는 내부 배치를 감추고 사용자가 이해하기 쉬운 API 표면을 만드는 데 유용하다.

### Cargo가 관리하는 것

Cargo는 의존성 버전 요구, feature, target별 의존성, build script, profile, workspace를 관리한다. `Cargo.lock`은 해석된 의존성 그래프를 기록한다. 애플리케이션은 재현 가능한 빌드를 위해 보통 lockfile을 함께 관리하고, 라이브러리는 소비자의 전체 해석과 공개 호환성까지 고려한다.

feature는 조건부 기능을 합성하는 수단이며 보통 additive해야 한다. 서로 배타적인 feature 조합은 의존성 통합 과정에서 깨지기 쉽다. build script는 컴파일 전에 실행되는 별도 Rust 프로그램이므로 외부 도구, 환경, 재실행 조건을 명확히 해야 한다.

### 속성과 조건부 컴파일

attribute `#[...]`와 `#![...]`는 item 또는 crate에 메타데이터를 붙인다. `#[derive(...)]`, `#[test]`, `#[must_use]`, lint 설정이 대표적이다. `#[cfg(...)]`는 target이나 feature에 따라 코드를 포함하고, `cfg!(...)`는 조건의 bool 값을 만든다. 조건별 코드가 실제 target에서 계속 컴파일되는지 CI로 확인해야 한다.

**흔한 오해:** `mod name;`은 파일을 런타임에 불러오는 명령이 아니다. 컴파일할 module tree를 선언한다.

## 7. 제네릭, trait, 연관 항목, 다형성

**선수 지식:** 사용자 정의 타입, 모듈과 공개 API, 수명 기초

### 제네릭과 trait bound

제네릭은 타입, 수명, 상수의 차이를 매개변수화한다. trait는 여러 타입이 공유하는 능력과 계약을 정의한다. `T: Display + Clone` 같은 bound는 함수가 실제로 필요한 연산을 선언하며, `where` 절은 복잡한 제약을 읽기 쉽게 만든다.

연관 항목에는 associated type, associated constant, associated function이 있다. associated type은 trait 구현마다 하나의 관련 타입을 정한다. 반면 generic parameter는 같은 타입이 여러 구현을 가질 수 있게 한다. 어느 쪽이 맞는지는 호출자가 타입을 고르는지, 구현자가 고르는지에 달려 있다.

```rust
trait Summary {
    type Context;
    const PREFIX: &'static str;

    fn summarize(&self, context: &Self::Context) -> String;
}
```

### 정적 디스패치와 동적 디스패치

제네릭과 `impl Trait`는 보통 정적 디스패치를 사용해 컴파일 시점에 구체 구현을 정하고 단형화한다.

* 인수 위치의 `impl Trait`는 익명 generic parameter와 비슷하게 호출자에게 여러 구체 타입을 허용한다.
* 반환 위치의 `impl Trait`는 함수가 하나의 숨겨진 구체 타입을 반환하게 한다. 서로 다른 구체 타입을 분기마다 반환하는 기능은 아니다.
* `dyn Trait`는 vtable을 통한 동적 디스패치다. 서로 다른 구체 타입을 하나의 포인터 뒤에 담을 수 있지만, dyn compatibility 조건을 만족해야 한다.

trait object는 `dyn Trait` 자체가 동적 크기 타입이므로 `&dyn Trait`, `Box<dyn Trait>`처럼 포인터 뒤에서 쓴다. 정적 디스패치는 인라이닝 기회와 타입별 코드 생성을 얻고, 동적 디스패치는 이질적인 값을 담는 유연성과 작은 API 경계를 얻는다. 둘 중 하나가 항상 빠르거나 우월하지는 않다.

### coherence와 orphan rule

coherence는 적용 가능한 구현의 충돌을 막는다. local trait는 외부 타입에도 구현할 수 있다. 외부 trait를 구현하려면 trait 구현에 등장하는 타입 중 orphan rule이 인정하는 local type이 있어야 하며, 그 local type보다 앞선 uncovered type parameter에도 제한이 적용된다. 따라서 단순히 trait 또는 self type 중 하나가 local인지만으로 모든 경우를 판정할 수는 없다. 이 규칙 덕분에 별도 크레이트가 충돌하는 구현을 추가해 프로그램 의미를 바꾸기 어렵다.

외부 trait를 외부 타입에 구현할 수 없을 때는 **newtype**으로 지역 타입을 감싸는 방법이 흔하다. blanket implementation은 넓은 타입 집합에 구현을 제공하므로 이후 구현 가능성을 제한할 수 있다. 공개 trait를 설계할 때는 이 영향을 고려한다.

**흔한 오해:** trait는 클래스 상속의 다른 이름이 아니다. 데이터 상속 없이 동작 계약, 제약, 정적 및 동적 다형성을 표현한다.

## 8. 클로저, 반복자, 변환

**선수 지식:** 소유권, 제네릭, trait

### 함수 포인터와 클로저

함수 포인터 `fn(A) -> B`는 캡처가 없는 함수나 클로저를 가리킨다. 클로저는 주변 환경을 빌리거나, 가변으로 빌리거나, 소유할 수 있는 익명 값이다. `move`는 캡처한 값을 클로저 환경으로 이동하도록 요구하며, 캡처 값이 `Copy`라면 복사가 일어날 수도 있다.

클로저가 구현하는 호출 trait는 본문이 캡처를 쓰는 방식에 따라 정해진다.

* `FnOnce`: `self`를 값으로 받아 호출하며 모든 클로저가 구현한다. 캡처 값을 밖으로 이동하는 클로저는 보통 이것만 구현한다.
* `FnMut`: `&mut self`로 반복 호출할 수 있다. 캡처를 변경할 수 있지만 반드시 변경하는 것은 아니며, 모든 `Fn` 클로저는 `FnMut`이기도 하다.
* `Fn`: `&self`로 반복 호출할 수 있다. 캡처를 이동하거나 변경하지 않는 클로저가 구현하며, `Fn`은 `FnMut`과 `FnOnce` 요구에도 사용할 수 있다.

API는 필요한 것보다 강한 bound를 요구하지 않는 편이 재사용성이 좋다. 함수 포인터와 클로저 타입은 같지 않지만, 캡처 없는 클로저는 함수 포인터로 바뀔 수 있다.

### 반복자

`Iterator`는 지연 계산되는 값의 흐름이며 핵심은 `next(&mut self) -> Option<Self::Item>`이다. `map`, `filter`, `flat_map`, `take` 같은 adapter는 새 반복자를 만들고, `collect`, `sum`, `fold`, `for_each` 같은 consumer가 실제 순회를 진행한다.

```rust
let total: i32 = [1, 2, 3, 4]
    .into_iter()
    .filter(|number| number % 2 == 0)
    .map(|number| number * number)
    .sum();

assert_eq!(total, 20);
```

`iter`, `iter_mut`, `into_iter`는 각각 공유 참조, 가변 참조, 소유 값의 흐름을 만드는 것이 핵심이다. 반복자 체인은 보통 zero cost abstraction을 목표로 최적화되지만, 실제 성능은 측정해야 한다.

### 변환 trait

* `From<T>`는 실패하지 않는 의미 있는 변환을 정의하고 `Into<U>`는 이에 따라 자동 제공되는 경우가 많다. 구현할 때는 보통 `From`을 우선한다.
* `TryFrom<T>`와 `TryInto<U>`는 범위 초과나 형식 오류가 가능한 변환을 `Result`로 표현한다.
* `AsRef<T>`와 `AsMut<T>`는 값싼 참조 변환에 쓰인다. 소유권 변환이 아니다.
* `Borrow<T>`와 `BorrowMut<T>`는 빌린 형태가 소유 형태와 같은 동등성, 순서, hash 의미를 가져야 하는 더 강한 계약이다. map 조회 같은 곳에서 중요하다.
* `Deref` coercion은 스마트 포인터의 참조 접근을 편하게 하지만, 일반 변환 수단으로 남용하면 API가 읽기 어려워진다.

**흔한 오해:** `into_iter()`의 결과를 이름만으로 단정하면 안 된다. 수신 타입과 edition, 구현된 trait를 보고 item이 값인지 참조인지 확인한다.

## 9. 스마트 포인터, 내부 가변성, Pin

**선수 지식:** 소유권, `Drop`, trait

### 포인터와 소유 모델

* `Box<T>`는 값을 힙에 두고 단일 소유한다. 재귀 타입의 크기를 유한하게 만들거나 큰 값을 간접 저장할 때도 쓴다.
* `Rc<T>`는 단일 스레드에서 참조 횟수 기반 공유 소유를 제공한다.
* `Arc<T>`는 원자적 참조 횟수로 스레드 간 공유 소유를 지원한다. 내부 값이 자동으로 변경 가능해지는 것은 아니다.
* `Weak<T>`는 소유권을 유지하지 않는 약한 참조다. 순환 참조를 끊거나 cache, parent link를 표현한다. 접근할 때 값이 이미 소멸했을 수 있어 `upgrade()`가 `Option`을 반환한다.
* `Cow<'a, B>`는 빌린 값과 소유 값을 하나의 타입으로 다루고, 변경이 필요할 때만 복제하는 clone on write 패턴을 지원한다.

참조 횟수는 cycle을 자동 수집하지 않는다. `Rc` 또는 `Arc`끼리 강한 순환을 만들면 메모리가 해제되지 않을 수 있다.

`Arc`, `Mutex`, `RwLock`의 스레드 안전성 및 동기화 의미는 12단계에서 `Send`, `Sync`와 함께 완성한다. 이 단계에서는 각 타입이 소유와 변경 권한을 어떻게 표현하는지에 집중한다.

### 내부 가변성

내부 가변성은 공유 참조 `&T`를 통해서도 정해진 규칙 아래 값을 바꾸는 패턴이다. 핵심 저수준 장치는 `UnsafeCell<T>`이며 안전한 wrapper가 규칙을 대신 검사한다.

* `Cell<T>`는 값을 빌려 주지 않고 복사하거나 교체하는 단일 스레드 도구다.
* `RefCell<T>`는 단일 스레드에서 빌림 규칙을 실행 시점에 검사한다. 위반하면 panic이다.
* `Mutex<T>`는 한 번에 한 실행 흐름의 변경 접근을 허용한다.
* `RwLock<T>`는 여러 reader 또는 하나의 writer를 허용한다.
* `OnceLock<T>`와 `LazyLock<T>`는 값을 한 번 초기화하거나 최초 접근 때 초기화하는 표준 도구다. 전역 가변 상태를 무제한 허용하는 장치가 아니라 초기화 정책을 타입으로 제한한다.

`RefCell`이 borrow checker를 끄는 것은 아니다. 컴파일 시점 검사를 실행 시점으로 옮기면서도 같은 기본 규칙을 지킨다. `Mutex`와 `RwLock`은 잠금 순서, 긴 critical section, poisoning 정책을 함께 설계해야 한다.

### `Pin`과 `Unpin` [고급]

`Pin<P>`는 포인터 자체가 아니라 pointee와 맺는 계약이다. pointee가 `Unpin`이 아닐 때 안전한 API는 그 값을 현재 위치에서 이동하거나 `Drop` 없이 무효화하지 못하게 한다. self referential state나 일부 `Future`처럼 이동하면 내부 포인터가 무효가 되는 타입에 필요하다. `T: Unpin`이면 이 이동 제한은 의미가 없어 안전하게 값을 꺼내거나 교체할 수 있으며, 대부분의 일반 타입에서는 `Pin`을 특별히 의식하지 않아도 된다.

`Pin`은 메모리를 물리적으로 고정하는 운영체제 기능이 아니고, 값의 소멸을 막지도 않는다. 포인터와 대상 타입이 맺는 API 계약이다. 직접 pin projection을 구현하거나 `unsafe`로 pin 보장을 다룰 때는 값이 소멸할 때까지 이동하지 않는다는 불변 조건을 정확히 문서화해야 한다.

**흔한 오해:** `Arc<T>`가 있으면 `T`가 thread safe해지는 것이 아니다. 스레드 간 이동과 공유 가능성은 `T`의 `Send`, `Sync` 구현에도 달려 있다.

## 10. 오류 설계

**선수 지식:** enum, `Option`, `Result`, trait, 변환

오류 처리는 문법보다 API 설계 문제다. 호출자가 복구하거나 분기할 수 있는 실패는 `Result<T, E>`, 값이 없다는 사실만 중요한 경우는 `Option<T>`가 자연스럽다. 불변 조건 위반이나 계속 실행할 수 없는 프로그래머 오류는 panic 후보지만, 라이브러리의 정상 입력 실패를 panic으로 처리하면 호출자의 선택을 빼앗는다.

### `?`와 오류 전파

`?`는 실패 시 현재 함수에서 일찍 반환하고, 성공이면 내부 값을 꺼낸다. 반환 오류 타입 사이에 적절한 변환이 있으면 `From`을 통해 바뀐다. 그래서 `?`는 오류를 무시하는 문법이 아니라 제어 흐름과 변환을 간결하게 표현하는 문법이다.

```rust
use std::num::ParseIntError;

fn doubled(input: &str) -> Result<i32, ParseIntError> {
    let number = input.trim().parse::<i32>()?;
    Ok(number * 2)
}
```

### 사용자 정의 오류

라이브러리 오류는 호출자가 분기할 수 있도록 의미 있는 enum variant와 관련 데이터를 제공하는 편이 좋다. `std::fmt::Display`는 사람을 위한 설명, `std::error::Error`는 오류 연결과 source를 위한 공통 인터페이스다. 내부 구현 오류를 공개 enum에 그대로 노출하면 API 호환성이 구현 세부 사항에 묶일 수 있다.

애플리케이션 경계에서는 서로 다른 오류를 하나의 보고 형태로 모을 수 있지만, 어떤 작업이 실패했고 사용자가 무엇을 할 수 있는지 context를 보존해야 한다. 로그를 남기고 다시 반환하는 식으로 같은 오류를 여러 층에서 중복 보고하지 않도록 책임을 정한다.

### panic 경계

panic은 설정에 따라 unwind하거나 프로세스를 abort할 수 있다. `catch_unwind`는 unwind panic을 잡는 제한된 경계이며 모든 실패를 잡는 일반 예외 처리 장치가 아니다. FFI 경계나 thread entry처럼 panic이 넘어가면 계약을 깨는 곳에서는 panic 정책을 명확히 해야 한다. destructor 안에서 panic이 겹치면 프로세스가 중단될 수 있으므로 `Drop`은 실패하지 않게 설계하는 편이 안전하다.

**흔한 오해:** `unwrap`과 `expect`가 무조건 나쁜 것은 아니다. 테스트, 예제, 코드상 불가능함이 명확한 내부 불변 조건에는 쓸 수 있다. 다만 외부 입력이나 정상적인 실패 가능성을 처리하는 기본 수단으로 삼으면 안 된다.

## 11. 테스트, 문서, 품질 도구

**선수 지식:** 모듈, Cargo, 오류 설계

단위 테스트는 보통 구현 가까이에서 비공개 세부 동작까지 검사하고, 통합 테스트는 공개 API를 외부 사용자처럼 사용한다. 문서 테스트는 예제가 실제 API와 함께 컴파일되고 실행되는지 확인한다. 테스트는 정상 경로뿐 아니라 경계값, 실패, 상태 전이, 회귀를 다뤄야 한다.

`#[cfg(test)]`, `#[test]`, `assert!`, `assert_eq!` 같은 안정된 도구와 `Result`를 반환하는 테스트를 익힌다. panic을 기대하는 테스트는 메시지까지 좁혀 엉뚱한 panic이 통과하지 않게 한다. 시간, 난수, 환경, 네트워크를 직접 읽는 코드는 경계를 분리해야 재현 가능한 테스트가 된다.

문서 주석 `///`와 `//!`는 Markdown으로 쓰며 `cargo doc`과 rustdoc이 API 문서를 만든다. 공개 API 문서에는 기능 설명, 입력과 출력, 오류, panic, 안전성, 예제를 필요한 만큼 적는다. `unsafe fn`이나 `unsafe trait`에는 호출자와 구현자가 지켜야 할 `# Safety` 계약이 특히 중요하다.

품질 도구의 역할은 서로 다르다.

* `rustfmt`는 일관된 형식을 만든다.
* Clippy는 의심스러운 코드와 비관용적 패턴을 찾는다.
* 컴파일러 lint는 unused, unsafe, API 관련 문제를 제어한다.
* CI는 지원 target과 feature 조합에서 검사, 테스트, 문서 생성을 반복한다.

lint를 모두 기계적으로 허용하거나 금지하기보다 팀이 요구하는 수준을 정하고, 예외에는 이유를 가까이 기록한다.

## 12. 스레드와 동시성

**선수 지식:** 소유권, 스마트 포인터, 내부 가변성, 오류 설계

### 스레드와 메시지 전달

`std::thread::spawn`은 새 스레드를 만들고 `JoinHandle`로 완료와 panic 결과를 관찰한다. 클로저가 스레드보다 짧게 사는 값을 빌리면 안 되므로 `move`가 자주 필요하다. scoped thread는 정해진 scope 안에서 join됨을 보장해 지역 값을 안전하게 빌릴 수 있다.

channel은 소유 값을 메시지로 보내 실행 흐름 사이의 결합을 줄인다. 다중 생산자와 단일 또는 다중 소비자 여부, bounded capacity, 송수신자 종료가 API마다 다르다. 표준 채널과 선택적 생태계 채널을 고를 때 이 의미를 먼저 확인한다.

### 공유 상태와 `Send`, `Sync`

`Send`는 값의 소유권을 다른 스레드로 옮길 수 있음을, `Sync`는 `&T`를 여러 스레드가 안전하게 공유할 수 있음을 뜻하는 unsafe auto trait다. 많은 타입은 구성 요소에 따라 자동 구현된다. 이를 직접 구현하는 일은 드물며 잘못된 구현은 undefined behavior로 이어질 수 있다.

`Arc<Mutex<T>>`는 공유 소유와 상호 배제를 조합한다. 하지만 타입이 컴파일된다는 사실만으로 교착, 기아, lock contention, 논리적 race가 사라지지는 않는다. 가능한 한 공유 상태의 범위와 잠금 시간을 줄이고, 잠금 순서를 정한다.

### 원자 연산과 메모리 순서 [고급]

atomic 타입은 lock 없이 개별 메모리 연산의 원자성을 제공한다. `Ordering::Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst`는 다른 메모리 연산과의 관찰 순서를 정한다.

* `Relaxed`는 해당 원자 값의 원자성만 필요할 때 쓴다.
* `Release` 저장과 이를 읽는 `Acquire` 로드는 앞선 쓰기를 뒤의 읽기와 동기화할 수 있다.
* `SeqCst`는 더 강한 전역 순서 모델을 제공하지만 알고리즘 전체의 정확성을 자동 보장하지 않는다.

memory ordering은 직관으로 낮추면 안 된다. 먼저 mutex나 channel로 정확한 설계를 만들고, 측정된 필요와 검증 가능한 happens before 관계가 있을 때 atomic 알고리즘을 고려한다. ABA 문제, false sharing, lock free와 wait free의 차이도 별도 학습 대상이다.

**흔한 오해:** data race가 없다는 말은 race condition이 없다는 뜻이 아니다. Rust는 안전한 코드의 data race를 막지만 작업 순서에 따른 논리 버그까지 제거하지는 않는다.

## 13. 비동기 프로그래밍

**선수 지식:** 클로저, trait, `Pin`, 스레드와 channel, 오류 설계

### `Future`와 executor

`async fn`은 호출 즉시 작업을 끝까지 실행하는 함수가 아니라 `Future`를 반환한다. future는 `poll`될 때 진행되며 준비되지 않았다면 `Poll::Pending`, 완료되면 `Poll::Ready`를 반환한다. `Waker`는 다시 진행할 수 있을 때 executor에 알린다. `.await`는 현재 future를 중단할 수 있는 지점이며 스레드를 반드시 차단하는 호출이 아니다.

**Rust 표준 라이브러리에는 async runtime이 포함되어 있지 않다.** 표준 라이브러리는 `Future`, `Poll`, `Waker` 같은 핵심 추상화를 제공한다. 실제 task scheduling, timer, 비동기 I/O는 executor 또는 runtime이 맡으며, 필요하다면 프로젝트 요구에 맞는 생태계 구현을 선택한다.

future는 기본적으로 지연 실행되고, executor가 poll하지 않으면 진행되지 않는다. CPU를 오래 점유하는 동기 작업을 async task 안에서 그대로 실행하면 같은 executor의 다른 task가 굶을 수 있다. blocking 작업은 runtime이 제공하는 별도 경계나 전용 스레드로 분리한다.

### 취소와 자원 정리

비동기 취소는 흔히 future를 더 이상 poll하지 않고 drop하는 방식으로 일어난다. 그러므로 `.await` 사이에서 부분적으로 변경된 상태, 잠금 guard, 외부 작업이 어떤 상태로 남는지 고려해야 한다. cancel safety는 future가 어느 await 지점에서 drop되어도 재시도나 상위 상태의 불변 조건을 깨지 않는 성질이다. 다만 spawn된 task의 handle을 drop하는 것이 task 취소를 뜻하는지는 runtime API마다 다르다. detach, 명시적 abort, cooperative cancellation, join 대기 중 어떤 의미인지 해당 runtime의 계약을 확인한다.

구조화된 동시성 관점에서는 child task의 소유자, 완료 대기, 오류 전파, 취소 전파를 명확히 한다. task를 무심코 분리하면 실패가 관찰되지 않거나 종료 뒤에도 작업이 남을 수 있다.

### backpressure와 흐름 제어

생산자가 소비자보다 빠르면 무제한 queue는 메모리를 계속 늘린다. bounded channel, concurrency limit, demand driven stream, timeout, load shedding으로 시스템이 감당할 수 있는 양을 표현한다. backpressure는 단순한 성능 최적화가 아니라 과부하에서 자원을 지키는 정확성 조건이다.

async mutex는 잠금 대기 중 스레드를 막지 않는 목적이 있지만, 어떤 mutex를 써야 하는지는 guard를 `.await` 너머로 유지하는지와 critical section 특성에 달려 있다. `.await`를 사이에 둔 잠금은 교착과 긴 점유 시간을 만들기 쉬우므로 피하거나 근거를 분명히 한다.

**흔한 오해:** async는 자동 병렬화가 아니다. 여러 작업을 효율적으로 교차 진행하는 모델이며 CPU 병렬성은 별도 스레드나 실행 전략이 필요하다.

## 14. 매크로와 조건부 컴파일

**선수 지식:** 패턴, trait, 모듈, attribute

매크로는 Rust 코드를 입력받아 Rust 코드를 생성한다. 함수보다 앞선 단계에서 동작하므로 가변 인수, 반복되는 선언, 새 문법에 가까운 인터페이스를 표현할 수 있다. 대신 오류 메시지와 도구 지원, 컴파일 시간이 복잡해질 수 있어 함수와 trait로 충분한 문제에는 그것들을 우선한다.

### 선언형 매크로

`macro_rules!`는 token pattern을 맞춰 코드를 확장한다. 반복 문법, fragment specifier, hygiene, `$crate` 경로를 이해해야 재사용 가능한 매크로를 만들 수 있다.

```rust
macro_rules! count_items {
    ($($item:expr),* $(,)?) => {
        <[()]>::len(&[$({ let _ = stringify!($item); () }),*])
    };
}

assert_eq!(count_items!("a", "b", "c"), 3);
```

이 구현은 `stringify!`를 사용하므로 입력 표현식을 실행하지 않고 쉼표로 구분된 `expr` fragment의 개수만 센다.

매크로 인수 expression을 확장 결과에서 여러 번 사용하면 부작용도 여러 번 평가될 수 있다. 입력을 한 번만 평가해야 한다면 임시 바인딩을 만드는 등 확장 의미를 점검한다.

### 절차 매크로 [고급]

procedural macro는 token stream을 Rust 코드로 변환하며 별도 proc macro 크레이트에서 작성한다.

* derive macro는 `#[derive(Name)]`로 구현을 생성한다.
* attribute macro는 item에 붙어 구조를 바꾼다.
* function like macro는 `name!(...)` 형태로 자유로운 입력을 받는다.

절차 매크로는 타입 검사 전 token을 다루므로 타입 정보를 직접 아는 것이 아니다. 생성 코드의 경로, generic, 수명, hygiene, span과 진단 품질을 신중히 다뤄야 한다.

`#[cfg]`와 Cargo feature는 매크로와 자주 함께 쓰이지만 역할이 다르다. 전자는 코드를 조건부로 포함하고, 후자는 의존성 그래프에서 기능을 선택해 cfg를 활성화한다.

**안정성 주의:** 일부 매크로 내부 기능, compiler plugin 성격의 기능, 진단 API는 nightly 또는 불안정일 수 있다. stable에서 제공되는 `macro_rules!`와 안정화된 proc macro API를 기준으로 시작하고, 세부 기능은 공식 문서에서 상태를 확인한다.

## 15. unsafe, 메모리 불변 조건, FFI

**선수 지식:** 소유권과 수명, 스마트 포인터, 동시성, 오류 설계

> **고급 주제:** `unsafe`는 borrow checker를 끄는 모드가 아니다. 컴파일러가 검증할 수 없는 몇 가지 작업을 개발자가 정해진 불변 조건 아래 수행하겠다고 약속하는 경계다. 안전한 Rust의 나머지 규칙은 계속 적용된다.

### unsafe가 허용하는 일

unsafe context에서는 raw pointer 역참조, unsafe 함수 호출, mutable static 접근, unsafe trait 구현, union field 접근 같은 작업을 할 수 있다. 각 unsafe block은 가능한 한 작게 두고, 바로 앞에서 왜 선행 조건이 만족되는지 설명한다. 더 중요한 것은 외부에 안전한 API를 제공한다면 어떤 입력에서도 내부 unsafe의 전제가 깨지지 않게 만드는 것이다.

### 메모리 불변 조건과 undefined behavior

다음은 핵심 검토 대상이다.

* 포인터가 정렬되고, 유효하며, 해당 접근 크기만큼 할당된 객체를 가리키는가
* 참조가 null이 아니고 정렬되어 있으며 수명 동안 aliasing 규칙을 지키는가
* 값의 bit pattern이 타입에 유효한가
* 초기화되지 않은 메모리를 읽지 않는가
* 이미 이동하거나 해제한 값을 쓰지 않는가
* 여러 스레드가 동기화 없이 같은 메모리를 충돌 접근하지 않는가
* 포인터 연산이 같은 allocation과 허용 범위 안에 있는가

이 조건을 어기면 undefined behavior가 생길 수 있다. 당장 원하는 결과가 나오거나 debug에서 통과했다는 사실은 안전성 증명이 아니다.

### raw pointer, 초기화, 소멸 제어

`*const T`와 `*mut T`는 raw pointer이며 자동 수명 검사를 받지 않는다. 참조로 바꾸기 전에 정렬, 유효성, 별칭, 수명을 보장해야 한다.

`MaybeUninit<T>`는 아직 초기화되지 않은 메모리를 타입에 거짓말하지 않고 표현한다. 완전히 초기화됐다는 증거 없이 `assume_init`을 호출하면 안 된다. `ManuallyDrop<T>`는 자동 `Drop`을 억제한다. 내용물을 명시적으로 소멸시키지 않으면 자원이 누수될 수 있지만, 반드시 정확히 한 번 소멸시켜야 하는 것은 아니다. 다만 직접 소멸시킨 뒤 다시 접근하거나 두 번 소멸시키면 undefined behavior가 될 수 있으므로 초기화 및 소멸 상태를 별도로 추적해야 한다. `MaybeUninit`과 `ManuallyDrop`은 일반 코드의 성능 요령이 아니라 저수준 자료 구조와 FFI에서 제한적으로 쓰는 도구다.

### layout, provenance, 별칭 [고급]

Rust의 기본 표현은 필드 순서나 padding을 안정된 ABI로 약속하지 않는다. `std::alloc::Layout`은 크기와 정렬을 함께 표현한다. pointer provenance는 포인터가 어느 allocation에서 어떤 접근 권한을 유래했는지를 설명하는 메모리 모델 개념이다. 정수를 포인터로 바꿨다가 다시 쓰는 코드나 allocation 경계를 넘는 산술은 주소 숫자만 맞는다고 정당화되지 않는다.

별칭 모델과 strict provenance의 세부 사항은 발전 중인 영역이다. **일부 관련 API와 언어 모델은 nightly 또는 불안정할 수 있으므로** 현재 Reference, 표준 API 문서, Nomicon을 확인하고 실험 도구의 결과를 언어 전체의 확정 규칙으로 과장하지 않는다.

### FFI

외부 ABI 경계에서는 Rust와 상대 언어 양쪽의 계약을 맞춰야 한다.

* `extern "C"`는 C ABI로 함수 경계를 선언한다.
* Rust 2024 edition에서 외부 항목 선언은 `unsafe extern "C" { ... }` 블록에 둔다.
* `#[repr(C)]`는 struct 또는 enum 표현을 C와 호환 가능한 규칙에 맞춘다. 모든 Rust 타입이 곧바로 C 호환이 되는 것은 아니다.
* C 문자열은 `CStr`과 `CString`으로 다룬다. 내부 NUL, 종료 NUL, 인코딩 정책을 구분한다.
* 포인터의 null 가능성, 길이, 정렬, 변경 권한, 유효 기간을 문서화한다.
* 누가 할당하고 누가 해제하는지, allocator와 소멸 함수를 어느 쪽이 제공하는지 정한다.
* 일반 `"C"` ABI는 unwind를 허용하지 않는다. Rust panic이 경계에 도달하면 abort할 수 있고, 외부 예외가 이 경계를 넘어오는 것은 undefined behavior가 될 수 있다. unwind가 계약의 일부일 때만 `"C-unwind"` 계열 ABI를 의도적으로 선택한다.
* panic과 외부 오류를 어느 경계에서 잡아 Rust 타입이나 오류 코드로 변환할지 정책을 둔다.
* callback이 저장되는 기간과 호출 스레드, 동시 호출 가능성을 계약에 넣는다.

안전한 wrapper는 FFI의 raw 계약을 한곳에서 검증하고 외부에는 참조, 슬라이스, `Result`, 소유 타입처럼 Rust다운 API를 제공한다.

**흔한 오해:** unsafe block 안에서 문제가 발생하지 않았다고 unsafe가 올바른 것은 아니다. 잘못 만든 참조가 나중의 안전한 코드에서 최적화와 충돌하며 문제가 드러날 수 있다.

## 16. 고급 타입 시스템

**선수 지식:** 제네릭, trait, 수명, unsafe 기초

> 이 단계는 라이브러리 설계, 저수준 코드, 복잡한 비동기 및 추상화에서 필요하다. 모든 애플리케이션이 직접 써야 하는 목록은 아니다.

### const generic

const generic은 값 수준의 상수를 타입 매개변수로 받는다. `[T; N]`의 `N`처럼 크기를 타입에 보존해 컴파일 시점 제약과 구현 재사용을 가능하게 한다. 안정 Rust에서 사용할 수 있는 범위가 있지만, 복잡한 generic const expression 등 일부 표현은 여전히 제한되거나 불안정할 수 있다.

```rust
struct Window<T, const N: usize> {
    values: [T; N],
}
```

### GAT와 HRTB

GAT, generic associated type은 associated type 자체가 수명이나 타입 매개변수를 받게 한다. 빌린 데이터를 반환하는 iterator 계열 추상화처럼 출력 타입이 빌림 수명에 의존할 때 유용하다. GAT의 핵심 형태는 stable이지만 모든 고급 제약과 주변 기능이 동일하게 안정화됐다고 가정하면 안 된다.

HRTB, higher ranked trait bound는 `for<'a>`로 모든 적절한 수명에 대해 trait가 성립해야 함을 표현한다. 특정 하나의 호출 수명을 고르는 것과, 어떤 호출 수명에도 동작해야 하는 것은 다르다.

```rust
fn apply_to_text<F>(function: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    function("rust")
}
```

### variance와 `PhantomData`

variance는 수명 또는 타입의 subtype 관계가 컨테이너를 통해 어떻게 전달되는지 설명한다. 공유 참조, 가변 참조, 함수 인수, raw pointer는 서로 다른 variance를 가질 수 있다. 일반 애플리케이션에서는 추론되지만 unsafe container, 수명 wrapper, FFI handle 설계에서는 soundness에 직접 영향을 준다.

`PhantomData<T>`는 실제 필드로 저장하지 않는 타입이나 수명과의 논리적 관계를 컴파일러에 알린다. variance, auto trait, drop checking에 영향을 줄 수 있으므로 단순히 사용하지 않는 generic 경고를 없애는 표지가 아니다.

### DST, `Sized`, never type, ZST

DST, dynamically sized type은 컴파일 시점에 크기가 정해지지 않는 타입이다. `[T]`, `str`, `dyn Trait`가 대표적이며 보통 fat pointer 뒤에 둔다. generic parameter에는 기본적으로 `Sized` bound가 있고 `T: ?Sized`는 크기가 동적일 가능성을 허용한다.

never type `!`는 값이 절대 돌아오지 않음을 나타내며 `panic!`, 무한 `loop`, 프로세스 종료 같은 발산 표현과 관계가 있다. **사용 문맥에 따라 `!` 관련 일부 기능의 안정성이나 추론 제약이 다를 수 있으므로** 현재 문서를 확인한다.

ZST, zero sized type은 크기가 0인 타입이다. unit `()`, 데이터 없는 표지 타입 등이 있으며 상태나 권한을 런타임 저장 비용 없이 타입에 표현할 수 있다. 크기가 0이어도 정렬, 고유 주소 기대, raw pointer 산술에는 주의가 필요하다.

### newtype과 typestate

newtype은 기존 타입을 단일 필드 구조체로 감싸 의미, 검증, trait 구현, 단위 구분을 추가한다. typestate는 가능한 작업을 타입 상태로 나눠 잘못된 호출 순서를 컴파일되지 않게 한다.

```rust
struct Draft(String);
struct Published(String);

impl Draft {
    fn publish(self) -> Published {
        Published(self.0)
    }
}
```

typestate는 상태 수가 적고 전이가 명확할 때 강력하다. 상태 조합이 폭발하거나 런타임에서 동적으로 바뀌는 경우에는 enum과 검증 로직이 더 읽기 쉬울 수 있다.

### 안정성과 nightly 구분

specialization, 일반적인 negative impl, 일부 const trait 기능, 복잡한 generic const expression, 여러 compiler 내부 attribute는 대표적으로 안정 채널에서 제한될 수 있는 영역이다. 기능 이름을 봤다는 이유만으로 stable이라고 가정하지 말고, 현재 Reference와 tracking issue를 확인한다. nightly 기능을 쓴다면 crate root의 feature gate, 사용 이유, 제거 조건, CI toolchain을 명시한다.

## 17. 성능과 프로덕션 운영

**선수 지식:** 앞 단계 전체, 특히 소유권, Cargo, 동시성, unsafe

### 측정과 비용 모델

성능 개선은 측정에서 시작한다. 알고리즘 복잡도, 할당 횟수, cache locality, 복사량, lock contention, I/O 대기, code size를 구분한다. release profile에서 실제 작업 부하를 측정하고, 평균뿐 아니라 tail latency와 메모리 최고점도 본다.

Rust의 zero cost abstraction은 추상화가 항상 비용 0이라는 뜻이 아니다. 수작업 저수준 코드에 견줄 수 있도록 설계됐다는 방향이며, bounds check, allocation, dynamic dispatch, reference counting, cloning에는 상황별 비용이 있다. `clone()`을 없애는 것보다 불필요한 소유권 왕복과 자료 구조 선택을 먼저 살핀다.

프로파일 설정에는 최적화, debug 정보, LTO, codegen unit, panic 전략 등이 있다. 값을 바꾸기 전 빌드 시간, 실행 시간, binary size, 디버깅과 배포 요구 사이의 tradeoff를 기록한다.

### 의존성과 공급망

의존성은 기능뿐 아니라 빌드 시간, binary size, MSRV, target 지원, 라이선스, 유지보수, 보안 표면을 가져온다. 직접 의존성과 전이 의존성을 함께 검토하고 lockfile과 감사 도구로 실제 해석 결과를 관리한다. 선택적 생태계 도구를 쓸 때도 프로젝트 정책과 신뢰 모델을 먼저 정한다.

Cargo feature는 합쳐질 수 있으므로 비활성화가 안전성 조건이 되게 설계하지 않는다. default feature를 끌 때는 의존성 그래프 전체에서 다시 켜질 수 있음을 이해한다.

### MSRV와 SemVer

MSRV, minimum supported Rust version은 프로젝트가 지원하겠다고 약속한 가장 오래된 compiler 기준이다. 패키지의 MSRV는 `Cargo.toml`의 `[package] rust-version`에 명시하고 해당 버전과 최신 stable에서 CI를 실행해 검증한다. 최신 언어 기능을 쓰는 것과 지원 범위를 넓게 유지하는 것은 tradeoff이며, MSRV 변경은 프로젝트의 호환성 정책에 따라 문서화한다.

SemVer는 공개 API 호환성의 약속이다. 함수 시그니처뿐 아니라 trait 구현, enum 완전성, feature, re-export, 오류 타입, 동작 계약도 호환성에 영향을 준다. 공개 enum에 variant를 추가하거나 blanket impl을 추가하는 일도 소비자 코드와 충돌할 수 있다. `#[non_exhaustive]`는 향후 확장 가능성을 알리지만 무분별한 변경을 정당화하지는 않는다.

### 보안과 입력 경계

안전한 Rust도 논리 취약점, 권한 오류, 경로 순회, 자원 고갈, panic 기반 서비스 중단, 부적절한 암호 사용을 자동으로 막지는 않는다. 외부 입력의 길이와 형식, 정수 계산, 압축 해제 크기, queue 용량, timeout을 경계에서 제한한다. 비밀 값은 로그와 오류에 남기지 않고, 임시 파일과 권한, symlink, path canonicalization의 경쟁 조건을 고려한다.

unsafe와 FFI가 있다면 안전성 계약을 별도 검토한다. 의존성 권고와 lockfile 변경을 정기적으로 확인하고, 필요 없는 build script와 native dependency를 줄인다.

### cross compilation과 target

cross compilation은 host에서 다른 target용 코드를 만드는 일이다. Rust target 설치만으로 끝나지 않을 수 있으며 linker, C toolchain, system library가 필요하다. `cfg(target_os)`, `cfg(target_arch)`, `cfg(target_pointer_width)`, target feature를 통해 차이를 표현하되 실제 target에서 빌드하고 가능하면 실행해 검증한다.

정수 크기, endian, 정렬, 원자 연산 지원, 파일 시스템과 프로세스 API가 target마다 다를 수 있다. `usize`를 직렬화 형식에 그대로 쓰거나 native layout을 네트워크 형식으로 간주하면 이식성이 깨진다.

### `no_std` [고급]

`#![no_std]`는 표준 라이브러리 대신 `core`를 중심으로 빌드하는 환경을 위한 선택이다. allocator가 있으면 `alloc`의 `Vec`, `String`, `Box` 등을 쓸 수 있지만 운영체제 기능, 스레드, 파일 I/O가 자동 제공되지는 않는다. panic handler, allocator, entry point, target 지원은 환경이 결정한다.

라이브러리는 핵심 로직을 `core` 또는 `alloc`에 맞추고 운영체제 통합을 feature로 분리할 수 있다. 하지만 실제 필요 없이 `no_std` 호환성을 약속하면 API와 테스트 행렬만 복잡해질 수 있다.

## 18. 프로젝트 진행 방향

이 지도는 곧바로 작업 목록을 정하는 문서가 아니다. 앞으로 프로젝트를 설계할 때는 아래 순서로 학습 범위를 넓히면 개념 사이의 빈틈을 찾기 쉽다.

### 첫 번째 흐름: 정확한 단일 실행 경로

값과 함수, 제어 흐름을 사용해 입력을 출력으로 바꾸는 작은 프로그램에서 시작한다. 이때 목표는 문법을 많이 쓰는 것이 아니라 소유자, 이동 시점, 오류 반환을 설명할 수 있게 만드는 것이다. 문자열과 컬렉션을 추가할 때는 인코딩과 부재를 타입으로 표현한다.

### 두 번째 흐름: 경계가 있는 라이브러리

데이터 모델을 struct와 enum으로 만들고, 모듈과 가시성으로 공개 API를 좁힌다. generic과 trait는 실제 중복이나 교체 가능한 정책이 나타났을 때 도입한다. 문서 테스트와 통합 테스트가 사용자의 관점에서 API 계약을 고정한다.

### 세 번째 흐름: 자원과 실패를 다루는 프로그램

파일, 프로세스, 네트워크 같은 외부 경계를 붙이면 `Path`, `OsStr`, custom error, timeout, cleanup이 중요해진다. RAII로 정상 종료와 조기 반환에서 같은 정리 규칙이 작동하게 하고, panic이 넘어가면 안 되는 경계를 찾는다.

### 네 번째 흐름: 동시 실행

먼저 thread와 bounded channel로 소유권 이전과 종료를 명확히 한다. 공유 상태가 필요하면 `Arc`, 잠금, `Send`, `Sync`를 추가한다. I/O 동시성이 실제 요구가 될 때 async로 옮기며 cancellation, backpressure, task ownership을 설계에 포함한다.

### 다섯 번째 흐름: 저수준 또는 배포 제약

측정 결과가 요구할 때 성능을 조정한다. FFI, custom allocation, pin projection, atomic 알고리즘, `no_std`는 각각의 불변 조건과 환경 제약이 분명할 때만 도입한다. nightly 기능도 문제와 제거 계획이 구체적일 때 제한적으로 검토한다.

프로젝트 규모가 커질수록 새 문법의 수보다 다음 질문에 답하는 능력이 중요해진다.

* 이 값은 누가 소유하고 언제 소멸하는가?
* 이 API가 허용하는 상태와 금지하는 상태는 무엇인가?
* 실패, 취소, panic 중에 어떤 불변 조건이 유지되는가?
* 여러 실행 흐름이 어떤 순서와 권한으로 데이터를 보는가?
* 공개 계약, 지원 target, MSRV, feature가 어떻게 검증되는가?
* unsafe가 있다면 안전한 호출자가 그 전제를 깨뜨릴 수 없는가?

## 공식 참고 자료

개념 지도는 탐색 순서를 제공하지만 세부 규칙의 최종 기준은 공식 문서다.

* [The Rust Programming Language](https://doc.rust-lang.org/stable/book/): 언어 전반을 배우는 공식 입문서
* [The Rust Reference](https://doc.rust-lang.org/reference/): 문법과 언어 의미의 상세 기준
* [표준 라이브러리 문서](https://doc.rust-lang.org/std/): 타입, trait, 함수, 모듈별 API와 안정화 정보
* [The Cargo Book](https://doc.rust-lang.org/cargo/): package, dependency, feature, profile, 배포 설명
* [rustup 문서](https://rust-lang.github.io/rustup/): toolchain, component, target 관리
* [The rustdoc Book](https://doc.rust-lang.org/rustdoc/): API 문서와 문서 테스트 작성법
* [Clippy 문서](https://doc.rust-lang.org/clippy/): lint 사용법과 설정
* [rustfmt 문서](https://rust-lang.github.io/rustfmt/): 코드 형식 도구와 설정
* [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/): `Future`, executor, async 설계의 공식 학습 자료
* [The Rustonomicon](https://doc.rust-lang.org/nomicon/): unsafe Rust와 고급 메모리 주제
* [The Edition Guide](https://doc.rust-lang.org/edition-guide/): edition별 변화와 이전 방법

API의 stable 여부는 표준 라이브러리 문서의 안정화 표시를 확인한다. 언어 기능은 Reference와 공식 릴리스 자료를 기준으로 판단하고, nightly 전용 기능은 현재 상태가 바뀔 수 있음을 전제로 다룬다.
