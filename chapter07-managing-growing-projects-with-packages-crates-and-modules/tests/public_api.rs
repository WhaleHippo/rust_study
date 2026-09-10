// [학습 실습: C07-01]
// 목표: 외부 소비자가 `chapter07_managing_growing_projects_with_packages_crates_and_modules::hosting` 재내보내기 경로를 사용할 수 있음을 검증한다.
// 학습자 행동: 로컬에서 `pub use crate::front_of_house::hosting;`을 임시 제거한 뒤 직접 복구한다.
// RED: `cargo test -p chapter07-managing-growing-projects-with-packages-crates-and-modules greeting_when_hosting_is_reexported_from_the_library`가 `chapter07_managing_growing_projects_with_packages_crates_and_modules::hosting`을 찾지 못해 컴파일에 실패한다.
// GREEN: 재내보내기를 복구하면 외부 소비자 통합 테스트와 장 전체 테스트가 통과한다.
// 힌트: 통합 테스트는 라이브러리 밖에서 컴파일되므로 공개 크레이트 경로만 사용할 수 있다.
#[test]
fn greeting_when_hosting_is_reexported_from_the_library() {
    // `tests/`의 통합 테스트는 라이브러리와 별도 크레이트로 컴파일된다.
    // 그러므로 `super`나 비공개 모듈 대신 실제 외부 사용자와 같은 `chapter07_managing_growing_projects_with_packages_crates_and_modules::...` 공개 API만 사용한다.
    // 준비: chapter07_managing_growing_projects_with_packages_crates_and_modules 라이브러리 크레이트 밖의 소비자다.
    // 실행: 루트 재내보내기를 통해 hosting 모듈에 접근한다.
    let greeting =
        chapter07_managing_growing_projects_with_packages_crates_and_modules::hosting::greeting();
    // 검증: 공개 외부 경로가 인사말을 노출한다.
    assert_eq!(greeting, "Welcome");
}
