#[test]
fn greeting_when_hosting_is_reexported_from_the_library() {
    // Given: a consumer outside the chapter07 library crate.
    // When: the hosting module is reached through its root re-export.
    let greeting = chapter07::hosting::greeting();
    // Then: the public external path exposes the greeting.
    assert_eq!(greeting, "Welcome");
}
