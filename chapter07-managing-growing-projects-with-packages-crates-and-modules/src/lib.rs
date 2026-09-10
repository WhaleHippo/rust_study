// lib.rs는 라이브러리 크레이트의 루트다. `mod` 선언은 이 루트에서 모듈 트리를 시작한다.
// `mod back_of_house;`는 같은 이름의 물리 파일 `src/back_of_house.rs`를 붙이지만,
// `pub`이 없으므로 이 크레이트 밖에서는 `chapter07_managing_growing_projects_with_packages_crates_and_modules::back_of_house` 경로로 접근할 수 없다.
mod back_of_house;
// `pub mod`는 `src/front_of_house.rs`를 모듈로 붙이는 동시에 그 모듈 경로를 외부에 공개한다.
pub mod front_of_house;

// `pub use`는 이미 공개된 깊은 모듈을 크레이트 루트에 다시 내보낸다.
// 따라서 외부 소비자는 긴 `chapter07_managing_growing_projects_with_packages_crates_and_modules::front_of_house::hosting` 대신
// 안정적이고 짧은 `chapter07_managing_growing_projects_with_packages_crates_and_modules::hosting` 경로를 사용한다.
pub use crate::front_of_house::hosting;

// `pub`이 없는 함수는 라이브러리 내부 구현이다. 아래의 `crate::school_name()`은
// 현재 라이브러리 크레이트 루트에서 시작하는 절대 경로이며 외부 소비자에게는 보이지 않는다.
fn school_name() -> &'static str {
    "Rust Study"
}

pub fn breakfast_toast() -> String {
    // 비공개 모듈 안의 공개 타입과 생성자는 이 라이브러리 내부에서만 이 경로로 사용할 수 있다.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // `toast`는 공개 필드라 수정할 수 있지만, `seasonal_fruit`는 비공개 필드라 직접 바꿀 수 없다.
    meal.toast = "Wheat".into();
    format!("{} toast with {}", meal.toast, meal.fruit())
}

pub fn menu_summary() -> String {
    // `crate::`는 크레이트 루트에서 탐색하는 절대 경로다.
    let featured = if crate::school_name() == "Rust Study" {
        back_of_house::Appetizer::Soup
    } else {
        back_of_house::Appetizer::Salad
    };
    let appetizer = match featured {
        back_of_house::Appetizer::Soup => "soup",
        back_of_house::Appetizer::Salad => "salad",
    };
    format!(
        "{} serves {} with {}",
        crate::school_name(),
        appetizer,
        // `self::`는 현재 모듈(여기서는 라이브러리 루트)에서 시작하는 상대 경로다.
        self::hosting::greeting()
    )
}

#[cfg(test)]
mod tests {
    // `super::`는 한 단계 상위 모듈을 가리킨다. 여기서는 비공개 `tests` 모듈에서
    // 라이브러리 루트의 재내보내기를 가져오며, `as host`는 로컬 별칭이다.
    use super::hosting as host;

    #[test]
    fn greeting_when_host_module_is_public() {
        // Given: a public nested module reached through a use alias.
        // When: its greeting is requested.
        let greeting = host::greeting();
        // Then: the lesson exposes the host greeting.
        assert_eq!(greeting, "Welcome");
    }

    #[test]
    fn breakfast_toast_when_public_field_is_changed() {
        // Given: a breakfast constructed inside its private module.
        // When: its public toast field is updated through the module API.
        let toast = super::breakfast_toast();
        // Then: the public field value is observable outside the module.
        assert_eq!(toast, "Wheat toast with peaches");
    }

    #[test]
    fn menu_summary_when_relative_paths_are_used() {
        // Given: modules using crate and self paths.
        // When: they build the menu summary.
        let summary = super::menu_summary();
        // Then: the absolute and relative paths resolve to the intended values.
        assert_eq!(summary, "Rust Study serves soup with Welcome");
    }
}
