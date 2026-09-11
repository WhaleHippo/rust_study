# Rust 학습 주제 및 완료 추적표

`day001-...`부터 `day077-...`까지의 항목은 앞으로 만들 학습 자료 폴더 이름이다. 해당 학습 자료를 완성하면 상자를 체크한다. `capstone001-...`부터 `capstone010-...`까지는 여러 단계를 아우르는 통합 완료 기준이며 day 폴더 이름이 아니다.

## 1. 도구 체인과 컴파일러 모델

* [x] `day001-toolchain-roles`: `rustup`, `rustc`, `cargo`의 책임을 구분한다.
* [x] `day002-project-units`: package, crate, module, edition, toolchain의 차이를 설명한다.
* [x] `day003-expressions-and-places`: statement와 expression, place와 value의 차이를 코드에서 찾는다.
* [x] `day004-cargo-commands`: `cargo check`, `build`, `test`, `doc`를 언제 쓰는지 안다.

## 2. 문법, 바인딩, 타입, 함수, 제어 흐름

* [x] `day005-bindings-and-constants`: 불변, `mut`, shadowing, `const`, `static`을 구분한다.
* [x] `day006-core-types`: 스칼라, 배열, 튜플, 범위의 타입과 메모리 특성을 설명한다.
* [x] `day007-function-expressions`: 함수 반환식과 세미콜론의 관계를 안다.
* [x] `day008-control-flow`: `if`, `match`, `loop`, `while`, `for`, label을 알맞게 고른다.

## 3. 소유권, 빌림, 슬라이스, 수명, 소멸

* [x] `day009-move-copy-clone`: 이동과 `Copy`, 명시적 `Clone`의 차이를 예측한다.
* [x] `day010-borrowing-and-reborrowing`: 공유 빌림, 가변 빌림, 재빌림의 유효 구간을 그릴 수 있다.
* [x] `day011-slice-ownership`: 슬라이스가 소유자와 맺는 관계를 설명한다.
* [x] `day012-lifetime-annotations`: 수명 표기가 수명을 연장하지 않는다는 사실을 안다.
* [x] `day013-drop-and-raii`: partial move, drop scope, RAII, `Drop`을 설명한다.

## 4. 문자열, UTF-8, 컬렉션

* [ ] `day014-string-types`: `String`, `str`, `&str`의 소유권과 크기 차이를 설명한다.
* [ ] `day015-unicode-units`: byte, `char`, grapheme cluster를 구분한다.
* [ ] `day016-os-strings-and-paths`: `Path`와 `OsStr`을 UTF-8 문자열로 성급히 바꾸지 않는다.
* [ ] `day017-collection-selection`: 순서, 조회, 삽입 패턴에 따라 컬렉션을 선택한다.

## 5. 구조체, 열거형, Option, Result, 패턴

* [ ] `day018-valid-state-modeling`: 구조체와 enum으로 유효한 상태만 모델링한다.
* [ ] `day019-methods-and-associated-functions`: method와 associated function을 구분한다.
* [ ] `day020-option-and-result`: `Option`과 `Result`를 null, 예외, sentinel 값과 비교해 설명한다.
* [ ] `day021-pattern-features`: destructuring, refutable pattern, guard, `@`, or pattern을 읽는다.
* [ ] `day022-exhaustive-matching`: wildcard가 완전성 검사를 약하게 만들 수 있음을 안다.

## 6. 모듈, 크레이트, 가시성, Cargo

* [ ] `day023-package-crate-module-target`: package, crate, module, target을 구분한다.
* [ ] `day024-paths-and-reexports`: 절대 경로와 상대 경로, `use`와 `pub use`를 설명한다.
* [ ] `day025-api-visibility`: 최소 공개 원칙으로 API 가시성을 정한다.
* [ ] `day026-cargo-configuration`: lockfile, feature, profile, `cfg`의 역할을 안다.

## 7. 제네릭, trait, 연관 항목, 다형성

* [ ] `day027-generics-and-bounds`: generic parameter와 trait bound를 필요한 능력으로 설명한다.
* [ ] `day028-associated-types`: associated type과 generic parameter의 선택 기준을 안다.
* [ ] `day029-static-and-dynamic-dispatch`: `impl Trait`와 `dyn Trait`의 표현력과 비용 차이를 안다.
* [ ] `day030-coherence-and-newtypes`: coherence, orphan rule, blanket impl, newtype의 관계를 설명한다.

## 8. 클로저, 반복자, 변환

* [ ] `day031-closure-call-traits`: 캡처 방식과 `move`, `FnOnce`, `FnMut`, `Fn`의 관계를 설명한다.
* [ ] `day032-function-pointers-and-closures`: 함수 포인터와 클로저를 구분한다.
* [ ] `day033-lazy-iterators`: 반복자의 지연 실행과 소비 시점을 예측한다.
* [ ] `day034-conversion-traits`: `From`, `TryFrom`, `AsRef`, `Borrow`를 의미에 따라 고른다.

## 9. 스마트 포인터, 내부 가변성, Pin

* [ ] `day035-smart-pointer-selection`: `Box`, `Rc`, `Arc`, `Weak`, `Cow`를 소유권 요구에 맞게 고른다.
* [ ] `day036-interior-mutability-tools`: `Cell`, `RefCell`, `Mutex`, `RwLock`의 검사 시점과 스레드 범위를 안다.
* [ ] `day037-cycles-and-deadlocks`: 순환 참조와 잠금 교착 가능성을 식별한다.
* [ ] `day038-pin-contracts`: `Pin`이 보장하는 것과 보장하지 않는 것을 설명한다.

## 10. 오류 설계

* [ ] `day039-failure-categories`: 부재, 복구 가능한 실패, 불변 조건 위반을 구분한다.
* [ ] `day040-question-mark-operator`: `?`가 수행하는 조기 반환과 오류 변환을 설명한다.
* [ ] `day041-custom-error-context`: 사용자 정의 오류에 의미와 source를 보존한다.
* [ ] `day042-panic-boundaries`: panic이 넘어가면 안 되는 시스템 경계를 식별한다.

## 11. 테스트, 문서, 품질 도구

* [ ] `day043-test-boundaries`: 단위, 통합, 문서 테스트의 경계를 구분한다.
* [ ] `day044-test-case-coverage`: 성공, 실패, 경계값, 회귀를 테스트한다.
* [ ] `day045-api-contract-documentation`: 공개 API의 오류, panic, safety 계약을 문서화한다.
* [ ] `day046-quality-tool-roles`: `rustfmt`, Clippy, rustdoc, CI의 역할을 구분한다.

## 12. 스레드와 동시성

* [ ] `day047-thread-ownership-and-join`: thread의 소유권 이동과 join을 설명한다.
* [ ] `day048-concurrency-model-selection`: channel과 공유 상태 중 더 자연스러운 모델을 고른다.
* [ ] `day049-send-and-sync`: `Send`, `Sync`가 보장하는 바를 설명한다.
* [ ] `day050-deadlocks-and-logical-races`: mutex를 써도 생길 수 있는 교착과 논리 race를 찾는다.
* [ ] `day051-atomic-ordering-readiness`: atomic ordering을 happens before 관계로 설명할 준비가 되었는지 판단한다.

## 13. 비동기 프로그래밍

* [ ] `day052-future-executor-model`: `Future`, `poll`, `Waker`, executor의 관계를 설명한다.
* [ ] `day053-async-runtime-boundary`: 표준 라이브러리에 async runtime이 없음을 안다.
* [ ] `day054-blocking-and-async-waits`: blocking 작업과 async 대기를 구분한다.
* [ ] `day055-async-cancellation-cleanup`: 취소 지점에서 자원과 상태가 어떻게 정리되는지 검토한다.
* [ ] `day056-backpressure-design`: bounded queue와 동시성 제한으로 backpressure를 설계한다.

## 14. 매크로와 조건부 컴파일

* [ ] `day057-functions-versus-macros`: 함수와 매크로 중 어느 쪽이 필요한지 판단한다.
* [ ] `day058-declarative-macros`: `macro_rules!`의 matching, repetition, hygiene를 설명한다.
* [ ] `day059-procedural-macro-kinds`: 세 종류의 procedural macro를 구분한다.
* [ ] `day060-macro-evaluation-count`: 생성 코드가 입력을 몇 번 평가하는지 점검한다.

## 15. unsafe, 메모리 불변 조건, FFI

* [ ] `day061-unsafe-invariants`: 각 unsafe block의 선행 조건과 유지되는 불변 조건을 적을 수 있다.
* [ ] `day062-raw-pointer-references`: raw pointer를 참조로 바꾸기 위한 조건을 설명한다.
* [ ] `day063-manual-initialization-and-drop`: `MaybeUninit`과 `ManuallyDrop`의 책임을 안다.
* [ ] `day064-layout-provenance-aliasing`: layout, provenance, aliasing을 주소값만으로 판단하지 않는다.
* [ ] `day065-ffi-boundaries`: FFI의 ABI, 문자열, ownership, panic 경계를 명시한다.

## 16. 고급 타입 시스템

* [ ] `day066-const-generics`: const generic이 값 제약을 타입에 보존하는 이유를 설명한다.
* [ ] `day067-gat-and-hrtb`: GAT와 HRTB가 해결하는 수명 의존 추상화를 구분한다.
* [ ] `day068-variance-and-phantom-data`: variance와 `PhantomData`가 soundness에 미치는 영향을 안다.
* [ ] `day069-dynamically-sized-types`: DST, `Sized`, `!`, ZST를 구분한다.
* [ ] `day070-newtype-and-typestate`: newtype과 typestate를 복잡성 비용까지 고려해 선택한다.
* [ ] `day071-stability-verification`: 고급 기능을 쓰기 전 stable 여부를 공식 문서에서 확인한다.

## 17. 성능과 프로덕션 운영

* [ ] `day072-production-performance-measurement`: release 환경의 실제 부하로 성능을 측정한다.
* [ ] `day073-performance-cost-model`: 할당, cache, dispatch, lock, I/O 비용을 구분한다.
* [ ] `day074-dependency-impact-review`: 의존성의 기능, 보안, 라이선스, MSRV, target 영향을 검토한다.
* [ ] `day075-semver-api-contracts`: SemVer를 시그니처보다 넓은 API 계약으로 이해한다.
* [ ] `day076-cross-target-validation`: cross target의 linker, layout, OS 차이를 검증한다.
* [ ] `day077-no-std-boundaries`: `no_std`, `core`, `alloc`의 제공 범위를 구분한다.

## 최종 역량 점검

아래 capstone 항목은 개별 학습일이 아니라 전체 학습 과정에서 확인하는 통합 완료 기준이다.

* [ ] `capstone001-compiler-error-model`: 컴파일 오류를 타입, 이동, 빌림, 수명 관계로 해석한다.
* [ ] `capstone002-text-and-byte-boundaries`: 문자열과 OS 문자열, byte와 Unicode 단위를 구분한다.
* [ ] `capstone003-state-modeling`: enum과 exhaustive matching으로 상태를 모델링한다.
* [ ] `capstone004-public-api-design`: 최소한의 공개 API와 명확한 오류 계약을 설계한다.
* [ ] `capstone005-polymorphism-selection`: generic, `impl Trait`, `dyn Trait` 중 필요한 다형성을 고른다.
* [ ] `capstone006-ownership-and-synchronization`: 소유, 공유, 내부 가변성에 맞는 pointer와 동기화 도구를 고른다.
* [ ] `capstone007-concurrency-lifecycle`: 동시성과 async 코드의 종료, 취소, backpressure를 설명한다.
* [ ] `capstone008-contract-verification`: 테스트, 문서, lint, CI로 계약을 반복 검증한다.
* [ ] `capstone009-unsafe-and-ffi-containment`: unsafe와 FFI의 불변 조건을 문서와 코드 경계에 가둔다.
* [ ] `capstone010-production-claims`: 성능, 호환성, 보안 주장을 실제 target과 측정으로 확인한다.
