# Rust 학습 용어집

이 용어집은 이 저장소에서 처음 등장하는 장을 기준으로 정리했다. 각 항목의 장 링크를 누르면 해당 개념의 코드, 실행 관찰, 테스트 우선 실습으로 돌아간다. 정의는 이 과정에서 쓰는 Rust 의미에 한정한다.

## 03장

과정으로 돌아가기: [03장 일반적인 프로그래밍 개념](CURRICULUM.md#chapter-03)

<a id="term-binding"></a>
**바인딩, binding:** `let name = value;`처럼 값에 이름을 연결하는 것. 기본은 불변이며 `mut`를 붙여야 같은 바인딩의 값을 바꿀 수 있다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

<a id="term-shadowing"></a>
**섀도잉, shadowing:** 같은 이름으로 새 `let` 바인딩을 만들어 이전 바인딩을 가리는 것. `mut`와 달리 타입도 바꿀 수 있으며 새 바인딩이다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

**상수, constant:** `const`로 선언하며 반드시 타입을 적는 컴파일 시간 값. `mut`를 붙일 수 없고 이름은 보통 대문자 스네이크 표기법을 쓴다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

<a id="term-scalar"></a>
**스칼라 타입, scalar type:** 하나의 값을 나타내는 정수, 부동소수점, 불리언, 문자 타입. 튜플과 배열 같은 복합 타입과 구분한다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

**튜플, tuple:** 서로 다른 타입의 고정 개수 값을 순서대로 묶는 복합 타입. `.0` 같은 위치나 패턴으로 값을 꺼낸다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

**배열, array:** 같은 타입의 고정 개수 값을 연속으로 저장하는 복합 타입. 길이가 타입의 일부다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

<a id="term-expression"></a>
**표현식, expression:** 평가되어 값을 만드는 코드. 세미콜론 없는 블록의 마지막 식, `if`, 값을 담아 `break`하는 `loop`가 예다. 문장은 동작을 수행하고 보통 값을 만들지 않는다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

**`const fn`:** 상수 문맥에서도 평가할 수 있도록 제한된 함수. 일반 런타임 호출도 가능하다. 최초 등장: [03장](CURRICULUM.md#chapter-03).

**제어 흐름, control flow:** `if`, `loop`, `while`, `for`로 실행 순서와 반복을 정하는 구조. 최초 등장: [03장](CURRICULUM.md#chapter-03).

## 04장

과정으로 돌아가기: [04장 소유권 이해하기](CURRICULUM.md#chapter-04)

<a id="term-ownership"></a>
**소유권, ownership:** 각 값에 소유자가 하나 있고, 소유자가 범위를 벗어나면 값이 해제되며, 소유권이 이동할 수 있다는 Rust의 메모리 관리 규칙. 가비지 컬렉터 없이 메모리 안전성을 지킨다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

<a id="term-move"></a>
**이동, move:** 값을 새 변수나 함수 인수에 넘기면서 소유권도 넘기는 것. `String`처럼 `Copy`가 아닌 값은 이동 뒤 원래 바인딩으로 사용할 수 없다. 클로저의 `move` 키워드는 캡처한 값의 소유권을 클로저 안으로 가져오도록 한다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

<a id="term-borrow"></a>
**빌림, borrow:** 소유권을 넘기지 않고 `&T` 또는 `&mut T` 참조로 값에 접근하는 것. 여러 불변 참조 또는 하나의 가변 참조만 같은 시점에 허용된다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

<a id="term-copy"></a>
**`Copy`:** 대입과 인수 전달 때 이동 대신 비트 단위 복사가 일어남을 나타내는 트레이트. 정수처럼 작고 단순한 타입이 주로 구현한다. `clone`은 명시적으로 복제할 수 있는 `Clone` 트레이트 메서드며, `Copy`와 다르다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

**참조, reference:** 유효한 값을 빌려 가리키는 `&T` 또는 `&mut T`. 참조는 소유하지 않으며 수명 동안 댕글링하지 않아야 한다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

**슬라이스, slice:** 컬렉션의 연속 구간을 빌리는 동적 크기 뷰. 문자열 슬라이스는 `&str`, 일반 슬라이스는 `&[T]`로 쓴다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

<a id="term-string-str"></a>
**`String`과 `&str`:** `String`은 힙에 저장된 UTF-8 문자열 버퍼를 소유하고 늘릴 수 있다. `&str`은 문자열 데이터의 유효한 UTF-8 구간을 빌리며 자체 버퍼를 소유하지 않는다. 문자열 리터럴의 타입도 `&'static str`이다. 최초 등장: [04장](CURRICULUM.md#chapter-04).

## 05장

과정으로 돌아가기: [05장 구조체로 연관된 데이터 구성하기](CURRICULUM.md#chapter-05)

<a id="term-struct"></a>
**구조체, struct:** 관련 필드를 하나의 이름 있는 타입으로 묶는 사용자 정의 타입. 필드 구조체, 튜플 구조체, 필드가 없는 유닛 구조체가 있다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

**튜플 구조체, tuple struct:** 구조체 이름은 있지만 필드 이름 없이 위치로 접근하는 구조체. 같은 필드 타입을 가진 튜플과도 별도 타입이다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

**유닛 구조체, unit-like struct:** 필드가 없는 구조체. 데이터를 담지 않고 타입이나 트레이트 구현 자체에 의미를 둘 때 쓴다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

<a id="term-struct-update"></a>
**구조체 갱신 문법, struct update syntax:** `User { email, ..old }`처럼 일부 필드만 새로 주고 나머지를 다른 인스턴스에서 가져오는 문법. `Copy`가 아닌 필드는 이동될 수 있다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

<a id="term-method"></a>
**메서드, method:** `impl` 안에 정의하며 첫 매개변수로 `self`, `&self`, `&mut self` 중 하나를 받는 함수. `value.method()` 문법으로 호출한다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

<a id="term-associated-function"></a>
**연관 함수, associated function:** 타입의 `impl` 안에 있지만 `self`를 받지 않는 함수. 생성자 관례인 `Type::new`나 `Rectangle::square`가 예다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

**`Self`:** 현재 정의하거나 구현하는 타입을 가리키는 별칭. 반환 타입과 같은 타입의 인수에 자주 쓴다. 최초 등장: [05장](CURRICULUM.md#chapter-05).

## 06장

과정으로 돌아가기: [06장 열거형과 패턴 매칭](CURRICULUM.md#chapter-06)

<a id="term-enum"></a>
**열거형, enum:** 가능한 여러 형태 중 정확히 하나를 값으로 갖는 사용자 정의 타입. 각 형태는 다른 데이터를 담을 수 있다. 최초 등장: [06장](CURRICULUM.md#chapter-06).

<a id="term-variant"></a>
**배리언트, variant:** 열거형이 가질 수 있는 개별 형태. `Option`의 `Some(T)`와 `None`이 대표적이다. 최초 등장: [06장](CURRICULUM.md#chapter-06).

<a id="term-option-result"></a>
**`Option<T>`와 `Result<T, E>`:** `Option`은 `Some(T)` 또는 `None`으로 값의 존재 여부를 나타낸다. `Result`는 `Ok(T)` 또는 `Err(E)`로 성공과 실패 정보를 나타낸다. 부재 자체만 중요하면 `Option`, 실패 이유를 전달해야 하면 `Result`를 고른다. `Option` 최초 등장: [06장](CURRICULUM.md#chapter-06). `Result` 최초 등장: [09장](CURRICULUM.md#chapter-09).

<a id="term-match"></a>
**`match`:** 값을 패턴과 차례로 비교해 처음 맞는 갈래를 실행하는 표현식. 가능한 경우를 빠짐없이 다뤄야 한다. 최초 등장: [06장](CURRICULUM.md#chapter-06).

**`if let`:** 한 패턴이 맞을 때만 코드를 실행하는 간결한 조건문. 빠짐없는 처리가 필요할 때는 `match`가 더 알맞다. 최초 등장: [06장](CURRICULUM.md#chapter-06).

<a id="term-let-else"></a>
**`let else`:** `let PATTERN = VALUE else { ... };` 형태로 성공 패턴을 바인딩한다. 패턴이 맞지 않는 `else` 블록은 `return`, `break`, 패닉처럼 현재 흐름을 끝내야 한다. 최초 등장: [06장](CURRICULUM.md#chapter-06).

## 07장

과정으로 돌아가기: [07장 패키지, 크레이트, 모듈로 프로젝트 관리하기](CURRICULUM.md#chapter-07)

<a id="term-package"></a>
**패키지, package:** 기능 묶음을 빌드하고 배포하는 Cargo 단위. 하나의 `Cargo.toml`을 가지며 여러 바이너리 크레이트와 최대 하나의 라이브러리 크레이트를 포함할 수 있다. 이 저장소의 `chapter07-managing-growing-projects-with-packages-crates-and-modules/`가 한 패키지다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

<a id="term-crate"></a>
**크레이트, crate:** Rust 컴파일 단위이자 모듈 트리의 루트. 라이브러리 또는 바이너리 형태다. 패키지는 Cargo의 묶음이고 크레이트는 컴파일되는 대상이라는 점이 다르다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

<a id="term-module"></a>
**모듈, module:** 크레이트 안의 항목을 이름 공간과 가시성 경계로 묶는 구조. `mod` 선언이 모듈 트리에 연결하며, 파일과 폴더만 존재한다고 자동으로 모듈이 되지는 않는다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

<a id="term-library-crate"></a>
**라이브러리 크레이트, library crate:** 다른 코드가 호출할 API를 제공하는 크레이트. 관례적 루트는 `src/lib.rs`다. 실행 진입점 `main`이 필수는 아니다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

<a id="term-binary-crate"></a>
**바이너리 크레이트, binary crate:** 실행 파일로 컴파일되는 크레이트. `fn main()`이 있으며 기본 루트는 `src/main.rs`다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

<a id="term-visibility"></a>
**가시성, visibility:** 항목을 어느 범위에서 접근할 수 있는지 정하는 규칙. 기본은 비공개고 `pub`로 공개 범위를 넓힌다. 공개 구조체 안의 필드도 따로 `pub`가 없으면 비공개다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

**절대 경로와 상대 경로:** 절대 경로는 `crate` 또는 외부 크레이트 이름에서 시작한다. 상대 경로는 `self`, `super`, 현재 모듈의 이름에서 시작한다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

**`use`:** 긴 경로의 항목을 현재 범위로 가져오는 선언. `as`로 별칭을 줄 수 있지만 그 자체로 외부에 다시 공개되지는 않는다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

<a id="term-reexport"></a>
**재노출, re-export:** `pub use`로 내부 항목을 새 공개 경로에 다시 내보내는 것. `chapter07_managing_growing_projects_with_packages_crates_and_modules::front_of_house::hosting`을 `chapter07_managing_growing_projects_with_packages_crates_and_modules::hosting`으로 제공하는 예가 있다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

**통합 테스트, integration test:** 패키지의 `tests/` 아래에서 라이브러리를 외부 사용자처럼 불러 검사하는 별도 크레이트. 비공개 항목에는 접근할 수 없다. 최초 등장: [07장](CURRICULUM.md#chapter-07).

## 08장

과정으로 돌아가기: [08장 일반적인 컬렉션](CURRICULUM.md#chapter-08)

<a id="term-vec"></a>
**`Vec<T>`:** 같은 타입의 값을 힙에 연속으로 저장하는 가변 길이 컬렉션. 벡터가 값을 소유한다. 최초 등장: [08장](CURRICULUM.md#chapter-08).

<a id="term-utf8"></a>
**UTF-8:** Rust 문자열이 사용하는 가변 길이 유니코드 인코딩. 한 사용자 인식 문자나 유니코드 스칼라 값이 바이트 하나와 일치하지 않을 수 있어 문자열 정수 인덱싱은 허용되지 않는다. 최초 등장: [08장](CURRICULUM.md#chapter-08).

<a id="term-hashmap"></a>
**`HashMap<K, V>`:** 해시 가능한 키를 값에 연결하는 컬렉션. 반복 및 디버그 출력 순서는 삽입 순서 계약이 아니다. 최초 등장: [08장](CURRICULUM.md#chapter-08).

<a id="term-entry"></a>
**엔트리 API, entry API:** 키의 존재 여부에 따라 삽입하거나 기존 값을 바꾸는 `HashMap::entry` 기반 API. `and_modify`와 `or_insert`를 이어 빈도 계산을 한 번의 조회 흐름으로 표현한다. 최초 등장: [08장](CURRICULUM.md#chapter-08).

## 09장

과정으로 돌아가기: [09장 오류 처리](CURRICULUM.md#chapter-09)

<a id="term-panic"></a>
**패닉, panic:** 현재 실행을 정상적으로 계속할 수 없음을 나타내며 스택을 되감거나 프로세스를 중단하는 실패 방식. 예상 가능한 입력 오류에는 보통 `Result`가 낫다. 최초 등장: [09장](CURRICULUM.md#chapter-09).

<a id="term-question-mark"></a>
**`?` 연산자:** `Result`나 `Option`의 성공값을 꺼내고 실패 또는 부재면 현재 함수에서 일찍 반환한다. 반환 오류 타입 변환에는 `From`이 관여할 수 있다. 최초 등장: [09장](CURRICULUM.md#chapter-09).

<a id="term-typed-error"></a>
**타입 오류, typed error:** 문자열 하나가 아니라 열거형과 구조체로 실패 종류와 데이터를 모델링한 값. 컴파일러가 호출자의 모든 오류 분기 처리를 도울 수 있다. 최초 등장: [09장](CURRICULUM.md#chapter-09).

**복구 가능한 오류:** 호출자가 재시도, 대체값, 사용자 안내 같은 선택을 할 수 있는 실패. 보통 `Result`로 표현한다. 최초 등장: [09장](CURRICULUM.md#chapter-09).

## 10장

과정으로 돌아가기: [10장 제네릭 타입, 트레이트, 수명](CURRICULUM.md#chapter-10)

<a id="term-generic"></a>
**제네릭, generic:** 구체 타입 대신 타입 매개변수로 함수, 구조체, 열거형, 메서드를 정의하는 방식. 컴파일러는 사용된 구체 타입에 맞게 코드를 단형성화한다. 최초 등장: [10장](CURRICULUM.md#chapter-10).

<a id="term-trait"></a>
**트레이트, trait:** 여러 타입이 공유할 수 있는 동작 계약. 메서드 시그니처와 기본 구현을 담고, 타입은 `impl Trait for Type`으로 구현한다. 데이터 모양을 정의하는 구조체나 가능한 상태를 정의하는 열거형과 다르다. 최초 등장: [10장](CURRICULUM.md#chapter-10).

<a id="term-trait-bound"></a>
**트레이트 바운드, trait bound:** 제네릭 타입이 구현해야 할 트레이트를 제한하는 조건. `T: Summary`, `impl Summary`, `where T: Summary`로 표현할 수 있다. 트레이트는 계약 자체고 바운드는 제네릭 매개변수에 그 계약을 요구하는 조건이다. 최초 등장: [10장](CURRICULUM.md#chapter-10).

**기본 메서드, default method:** 트레이트가 제공하는 메서드 본문. 구현 타입은 그대로 쓰거나 덮어쓸 수 있다. 최초 등장: [10장](CURRICULUM.md#chapter-10).

<a id="term-lifetime"></a>
**수명, lifetime:** 참조가 유효한 범위와 여러 참조 사이의 관계를 컴파일러가 추적하는 개념. `'a` 표기는 값을 오래 살게 하지 않으며 입력과 출력 참조의 관계를 명시한다. 최초 등장: [10장](CURRICULUM.md#chapter-10).

**단형성화, monomorphization:** 제네릭 코드를 실제 사용 타입별 구체 코드로 컴파일하는 과정. 정적 디스패치와 함께 런타임 추상화 비용을 줄인다. 최초 등장: [10장](CURRICULUM.md#chapter-10).

## 11장

과정으로 돌아가기: [11장 자동화 테스트 작성하기](CURRICULUM.md#chapter-11)

<a id="term-unit-test"></a>
**단위 테스트, unit test:** 보통 소스 파일의 `#[cfg(test)]` 모듈 안에서 작은 함수나 타입을 검사하는 테스트. 같은 모듈 계층의 비공개 항목도 검사할 수 있다. 최초 등장: [11장](CURRICULUM.md#chapter-11).

<a id="term-assertion"></a>
**어설션, assertion:** 기대 조건을 검사하고 거짓이면 테스트를 실패시키는 매크로 호출. `assert!`, `assert_eq!`, `assert_ne!`가 대표적이다. 최초 등장: [11장](CURRICULUM.md#chapter-11).

<a id="term-should-panic"></a>
**`#[should_panic]`:** 테스트 본문이 패닉해야 성공하도록 하는 속성. `expected` 문자열을 주면 의도한 원인의 패닉인지 더 좁게 확인한다. 최초 등장: [11장](CURRICULUM.md#chapter-11).

<a id="term-boundary-value"></a>
**경계값, boundary value:** 허용 범위의 끝이나 분기 직전과 직후에 있는 입력. `100` 허용과 `101` 거부처럼 오프바이원 오류를 잘 드러낸다. 최초 등장: [11장](CURRICULUM.md#chapter-11).

**TDD:** 실패하는 테스트 RED, 최소 구현으로 통과하는 GREEN, 동작을 지키며 정리하는 리팩터를 반복하는 개발 방식. 최초 등장: [11장](CURRICULUM.md#chapter-11).

## 12장

과정으로 돌아가기: [12장 I/O 프로젝트](CURRICULUM.md#chapter-12)

<a id="term-config"></a>
**구성, configuration:** 프로그램 동작을 정하는 입력을 검증된 값으로 묶은 것. 이 장의 `Config`는 쿼리, 본문, 대소문자 구분 여부를 담는다. 최초 등장: [12장](CURRICULUM.md#chapter-12).

<a id="term-determinism"></a>
**결정성, determinism:** 같은 입력에서 같은 관찰 결과가 나오는 성질. 이 장은 프로세스, 파일, 환경, 네트워크 대신 고정 메모리 입력을 써 결정적이다. 최초 등장: [12장](CURRICULUM.md#chapter-12).

<a id="term-dependency-injection"></a>
**의존성 주입, dependency injection:** 함수가 외부 자원을 직접 찾지 않고 필요한 값이나 기능을 인수로 받는 설계. 테스트가 원하는 입력을 바로 제공할 수 있다. 최초 등장: [12장](CURRICULUM.md#chapter-12).

<a id="term-pure-function"></a>
**순수 함수, pure function:** 같은 입력에 같은 출력을 만들고 외부 상태를 바꾸지 않는 함수. `search`는 입력 슬라이스만 읽고 일치하는 줄을 반환한다. 최초 등장: [12장](CURRICULUM.md#chapter-12).

**미니그렙, minigrep:** Rust Book에서 명령줄 파싱, 파일 읽기, 문자열 검색, 오류 처리를 합쳐 배우는 작은 grep형 프로젝트. 이 저장소 버전은 검색 핵심만 메모리 입력으로 재현한다. 최초 등장: [12장](CURRICULUM.md#chapter-12).

## 13장

과정으로 돌아가기: [13장 반복자와 클로저](CURRICULUM.md#chapter-13)

<a id="term-closure"></a>
**클로저, closure:** 이름 없이 정의할 수 있고 주변 환경을 캡처하는 호출 가능 값. `|arg| expression` 문법을 쓰며 캡처 방식에 따라 호출 트레이트가 정해진다. 최초 등장: [13장](CURRICULUM.md#chapter-13).

<a id="term-fn-traits"></a>
**`Fn`, `FnMut`, `FnOnce`:** 클로저 호출 방식을 나타내는 트레이트. `Fn`은 캡처를 불변으로 빌려 반복 호출할 수 있고, `FnMut`는 가변으로 빌리며, `FnOnce`는 캡처를 소비할 수 있어 최소 한 번 호출 가능하다. 한 클로저가 여러 호출 트레이트를 함께 구현할 수도 있다. 최초 등장: [13장](CURRICULUM.md#chapter-13).

<a id="term-iterator"></a>
**반복자, iterator:** `next(&mut self) -> Option<Item>`으로 값의 연속을 제공하는 `Iterator` 트레이트 구현체. `None`은 소진을 뜻한다. 최초 등장: [13장](CURRICULUM.md#chapter-13).

**반복자 어댑터, iterator adapter:** `map`, `filter`처럼 반복자를 다른 반복자로 바꾸는 메서드. 소비 어댑터가 값을 요청하기 전에는 보통 계산하지 않는다. 최초 등장: [13장](CURRICULUM.md#chapter-13).

<a id="term-lazy"></a>
**지연 평가, lazy evaluation:** 결과가 실제로 필요할 때까지 계산을 미루는 방식. 반복자 어댑터 체인은 `collect`, `sum`, `next` 같은 소비가 있어야 진행된다. 최초 등장: [13장](CURRICULUM.md#chapter-13).

## 14장

과정으로 돌아가기: [14장 Cargo와 Crates.io](CURRICULUM.md#chapter-14)

<a id="term-doctest"></a>
**문서 테스트, doctest:** `///` 문서 주석의 Rust 코드 블록을 컴파일하고 실행하는 테스트. API 설명과 실제 사용법이 어긋나는 일을 막는다. 최초 등장: [14장](CURRICULUM.md#chapter-14).

<a id="term-profile"></a>
**Cargo 프로필, profile:** 개발과 릴리스 등 빌드 상황별 최적화 및 컴파일 설정 묶음. 기본 `dev`는 빠른 개발 빌드, `release`는 최적화된 결과에 초점을 둔다. 최초 등장: [14장](CURRICULUM.md#chapter-14).

**Crates.io:** Rust 커뮤니티의 공개 패키지 레지스트리. 라이브러리 배포와 의존성 다운로드에 쓰인다. 최초 등장: [14장](CURRICULUM.md#chapter-14).

**`cargo doc`:** 현재 패키지와 의존성의 API 문서를 생성하는 명령. `--open`을 붙이면 브라우저로 연다. 최초 등장: [14장](CURRICULUM.md#chapter-14).

**`cargo install`:** 주로 Crates.io에 게시된 바이너리 크레이트를 로컬 실행 경로에 설치하는 명령. 라이브러리를 프로젝트 의존성에 추가하는 명령은 아니다. 최초 등장: [14장](CURRICULUM.md#chapter-14).

## 15장

과정으로 돌아가기: [15장 스마트 포인터](CURRICULUM.md#chapter-15)

**스마트 포인터, smart pointer:** 포인터처럼 동작하면서 소유권, 참조 수, 내부 가변성 같은 추가 메타데이터와 동작을 제공하는 자료 구조. 최초 등장: [15장](CURRICULUM.md#chapter-15).

<a id="term-box"></a>
**`Box<T>`:** 값을 힙에 두고 단일 소유권을 갖는 스마트 포인터. 크기를 컴파일 시점에 알 수 없는 재귀 연결을 포인터 크기로 간접화한다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

<a id="term-deref"></a>
**`Deref`:** `*value` 역참조와 역참조 강제 변환의 대상 타입을 정의하는 트레이트. 스마트 포인터가 참조처럼 동작하게 한다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

<a id="term-drop"></a>
**`Drop`:** 값이 범위를 벗어나 파괴될 때 실행할 정리 코드를 정의하는 트레이트. 직접 `drop` 메서드를 호출하지 않고 `std::mem::drop`으로 조기 해제한다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

<a id="term-rc"></a>
**`Rc<T>`:** 단일 스레드에서 강한 참조 수로 값을 공유 소유하는 스마트 포인터. `Rc::clone`은 내부 값의 깊은 복제가 아니라 소유자 수 증가다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

<a id="term-refcell"></a>
**`RefCell<T>`:** 빌림 규칙을 컴파일 시간이 아닌 런타임에 검사해 불변 외피 안의 값을 바꿀 수 있게 하는 타입. 규칙 위반은 패닉한다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

**내부 가변성, interior mutability:** 외부에는 불변 참조만 있어도 안전한 런타임 검사나 동기화를 통해 내부 값을 바꾸는 패턴. `RefCell`과 `Mutex`가 서로 다른 환경에서 이를 제공한다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

<a id="term-weak"></a>
**`Weak<T>`:** `Rc` 값에 대한 비소유 참조. 강한 참조 수를 늘리지 않으며 원본이 살아 있을 때만 `upgrade`가 `Some(Rc<T>)`를 반환한다. 참조 순환을 끊는 데 쓴다. 최초 등장: [15장](CURRICULUM.md#chapter-15).

## 16장

과정으로 돌아가기: [16장 겁 없는 동시성](CURRICULUM.md#chapter-16)

<a id="term-thread"></a>
**스레드, thread:** 한 프로세스 안의 독립 실행 흐름. `thread::spawn`은 새 스레드를 만들고 `JoinHandle::join`은 종료와 결과를 기다린다. 최초 등장: [16장](CURRICULUM.md#chapter-16).

<a id="term-channel"></a>
**채널, channel:** 송신자와 수신자 사이로 값의 소유권을 전달하는 통신 수단. `mpsc`는 여러 생산자와 하나의 소비자를 뜻한다. 모든 송신자가 사라지면 수신 반복이 끝난다. 최초 등장: [16장](CURRICULUM.md#chapter-16).

<a id="term-arc"></a>
**`Arc<T>`:** 원자적 참조 수를 쓰는 스레드 안전 공유 소유 스마트 포인터. 값 변경 자체를 동기화하지 않으므로 보통 `Mutex<T>`와 결합한다. 최초 등장: [16장](CURRICULUM.md#chapter-16).

<a id="term-mutex"></a>
**`Mutex<T>`:** 한 번에 한 스레드만 내부 값에 접근하도록 잠그는 동기화 타입. 잠금 가드는 범위를 벗어나면 잠금을 해제한다. 잠금 중 패닉하면 오염 상태가 될 수 있다. 최초 등장: [16장](CURRICULUM.md#chapter-16).

<a id="term-send-sync"></a>
**`Send`와 `Sync`:** `Send`는 값의 소유권을 스레드 사이로 보낼 수 있음을, `Sync`는 `&T`를 스레드 사이에서 안전하게 공유할 수 있음을 나타내는 마커 트레이트다. 최초 등장: [16장](CURRICULUM.md#chapter-16).

**동시성과 병렬성:** 동시성은 여러 작업의 진행을 구성하는 성질이고, 병렬성은 여러 작업이 실제로 같은 시각에 실행되는 성질이다. 스레드를 쓴다고 항상 병렬 실행이 보장되지는 않는다. 최초 등장: [16장](CURRICULUM.md#chapter-16).

## 17장

과정으로 돌아가기: [17장 비동기 프로그래밍](CURRICULUM.md#chapter-17)

<a id="term-future"></a>
**future:** 아직 준비되지 않았을 수 있는 값을 나타내는 `Future` 트레이트 구현체. executor가 `poll`해야 진행되며 생성만 해서는 작업이 실행되지 않는다. 최초 등장: [17장](CURRICULUM.md#chapter-17).

<a id="term-async-await"></a>
**`async`와 `.await`:** `async` 블록이나 함수는 future를 만든다. `.await`는 현재 future가 다른 future의 준비를 기다리며 양보할 수 있는 지점이고, 준비되면 결과값을 돌려받는다. 최초 등장: [17장](CURRICULUM.md#chapter-17).

<a id="term-executor"></a>
**executor:** future를 반복해서 poll하고 깨어난 작업을 다시 진행시키는 실행기. 이 장의 `block_on`은 한 future가 끝날 때까지 현재 스레드를 막아 실행한다. 최초 등장: [17장](CURRICULUM.md#chapter-17).

<a id="term-stream"></a>
**stream:** 시간에 따라 여러 값을 비동기로 내는 흐름. 동기 `Iterator`와 비슷하지만 다음 값 준비에 `.await`가 필요할 수 있다. 최초 등장: [17장](CURRICULUM.md#chapter-17).

**`join`:** 여러 future를 함께 진행하고 모두 준비되면 결과 튜플을 만드는 결합 연산. 운영체제 스레드를 새로 만든다는 뜻은 아니다. 최초 등장: [17장](CURRICULUM.md#chapter-17).

<a id="term-package-local-dependency"></a>
**패키지 로컬 의존성:** 워크스페이스 전체가 아닌 특정 자식 패키지의 `Cargo.toml`에만 선언된 의존성. 이 저장소에서는 `futures`가 `chapter17-fundamentals-of-asynchronous-programming`에만 직접 필요하다. 최초 등장: [17장](CURRICULUM.md#chapter-17).

## 18장

과정으로 돌아가기: [18장 Rust의 객체 지향 기능](CURRICULUM.md#chapter-18)

<a id="term-encapsulation"></a>
**캡슐화, encapsulation:** 데이터와 이를 다루는 메서드를 묶고 내부 상태를 비공개로 감춰 불변 조건을 지키는 설계. 최초 등장: [18장](CURRICULUM.md#chapter-18).

<a id="term-trait-object"></a>
**트레이트 객체, trait object:** `dyn Draw`처럼 특정 트레이트를 구현한 어떤 구체 타입의 값을 가리키는 동적 타입. 크기를 직접 알 수 없어 `&dyn Trait`나 `Box<dyn Trait>` 같은 포인터 뒤에 둔다. 트레이트 자체는 공유 동작 계약이고, 바운드는 제네릭 제약이며, 트레이트 객체는 런타임에 구체 타입을 지운 값이라는 점이 다르다. 최초 등장: [18장](CURRICULUM.md#chapter-18).

<a id="term-dynamic-dispatch"></a>
**동적 디스패치, dynamic dispatch:** 런타임의 실제 타입에 맞는 트레이트 메서드 구현을 간접 호출하는 방식. 제네릭 단형성화의 정적 디스패치와 달리 이질적 타입을 한 컬렉션에 담을 수 있지만 간접 호출 비용이 있다. 최초 등장: [18장](CURRICULUM.md#chapter-18).

<a id="term-object-safety"></a>
**객체 안전성, dyn compatibility:** 트레이트를 `dyn Trait`로 만들 수 있게 하는 메서드 규칙. 현재 Rust 문서에서는 dyn compatibility라고도 부른다. 반환 위치의 `Self`나 제네릭 메서드 등 일부 형태는 트레이트 객체에서 호출할 수 없다. 최초 등장: [18장](CURRICULUM.md#chapter-18).

## 19장

과정으로 돌아가기: [19장 패턴과 매칭](CURRICULUM.md#chapter-19)

<a id="term-pattern"></a>
**패턴, pattern:** 값의 구조를 검사하고 일부를 이름에 바인딩하는 문법. `match` 갈래, `let`, 함수 매개변수, `for`, `if let`, `while let` 등에 나타난다. 최초 등장: [19장](CURRICULUM.md#chapter-19).

<a id="term-destructuring"></a>
**구조 분해, destructuring:** 튜플, 구조체, 열거형의 내부 필드를 패턴으로 나눠 각각 바인딩하는 것. 최초 등장: [19장](CURRICULUM.md#chapter-19).

<a id="term-or-pattern"></a>
**OR 패턴:** `p1 | p2`처럼 여러 패턴 중 하나가 맞으면 같은 갈래를 선택하는 패턴. 각 대안은 같은 이름들을 호환되는 타입으로 바인딩해야 한다. 최초 등장: [19장](CURRICULUM.md#chapter-19).

<a id="term-match-guard"></a>
**매치 가드, match guard:** `pattern if condition =>` 형태로 패턴 일치 뒤 추가 불리언 조건을 검사한다. 가드는 빠짐없음 검사에 포함되지 않으므로 별도 나머지 갈래가 필요할 수 있다. 최초 등장: [19장](CURRICULUM.md#chapter-19).

<a id="term-at-binding"></a>
**`@` 바인딩:** `name @ pattern`으로 값이 패턴에 맞는지 검사하면서 전체 값을 이름에도 묶는 문법. 최초 등장: [19장](CURRICULUM.md#chapter-19).

**`while let`:** 패턴이 계속 맞는 동안 반복하는 제어 구조. `while let Some(value) = stack.pop()`은 `None`이 나오면 끝난다. 최초 등장: [19장](CURRICULUM.md#chapter-19).

**빠짐없는 매칭, exhaustive matching:** 타입이 가질 수 있는 모든 경우를 `match` 갈래가 덮는 성질. 새 열거형 배리언트가 생겼을 때 누락을 컴파일 오류로 찾게 해 준다. 최초 등장: [19장](CURRICULUM.md#chapter-19).
