// 함수와 부모 `hosting` 모듈이 모두 `pub`이므로 재내보낸 `chapter07::hosting::greeting`
// 경로를 통해 다른 크레이트도 호출할 수 있다.
pub const fn greeting() -> &'static str {
    "Welcome"
}
