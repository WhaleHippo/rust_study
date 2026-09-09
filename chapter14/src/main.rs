// 같은 패키지의 바이너리 크레이트는 라이브러리 크레이트 이름으로 공개 재공개 항목을 가져와 소비한다.
// 따라서 이 import는 `math::add_one`의 내부 경로 대신 라이브러리가 약속한 `chapter14::add_one` API를 사용한다.
use chapter14::add_one;

fn main() {
    println!("Chapter 14: More About Cargo and Crates.io");
    println!("profiles: dev compiles quickly; release optimizes output.");
    println!("docs: cargo doc builds API documentation; cargo test --doc runs doctests.");
    println!("install: cargo install <crate> installs a published binary.");
    println!("workspaces: packages can share one repository and lockfile.");
    println!("custom commands: cargo-foo is invoked as cargo foo.");
    println!(
        "library re-export: chapter14::add_one(41) = {}",
        // 바이너리는 형제 라이브러리의 공개 함수만 호출하며, 라이브러리 내부 구현에는 의존하지 않는다.
        add_one(41)
    );
}
