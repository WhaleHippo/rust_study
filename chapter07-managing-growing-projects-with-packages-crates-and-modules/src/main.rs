// main.rs는 바이너리 크레이트의 루트다. 같은 패키지의 lib.rs와는 별도 크레이트이므로,
// 라이브러리 공개 API를 외부 소비자처럼 크레이트 이름 `chapter07_managing_growing_projects_with_packages_crates_and_modules`으로 가져온다.
// `as host`는 긴 공개 모듈 경로를 이 바이너리 안에서만 짧게 쓰는 별칭이다.
use chapter07_managing_growing_projects_with_packages_crates_and_modules::hosting as host;

fn main() {
    println!("Chapter 07: Managing Growing Projects with Packages, Crates, and Modules");
    // 재내보낸 모듈은 별칭을 통해 사용하고, 나머지 공개 함수는 라이브러리 루트 경로로 호출한다.
    println!("{}", host::greeting());
    println!(
        "{}",
        chapter07_managing_growing_projects_with_packages_crates_and_modules::breakfast_toast()
    );
    println!(
        "{}",
        chapter07_managing_growing_projects_with_packages_crates_and_modules::menu_summary()
    );
}
