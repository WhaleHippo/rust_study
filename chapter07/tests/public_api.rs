#[test]
fn greeting_when_hosting_is_reexported_from_the_library() {
    // `tests/`의 통합 테스트는 라이브러리와 별도 크레이트로 컴파일된다.
    // 그러므로 `super`나 비공개 모듈 대신 실제 외부 사용자와 같은 `chapter07::...` 공개 API만 사용한다.
    // Given: a consumer outside the chapter07 library crate.
    // When: the hosting module is reached through its root re-export.
    let greeting = chapter07::hosting::greeting();
    // Then: the public external path exposes the greeting.
    assert_eq!(greeting, "Welcome");
}
