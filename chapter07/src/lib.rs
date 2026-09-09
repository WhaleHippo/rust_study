mod back_of_house;
pub mod front_of_house;

pub use crate::front_of_house::hosting;

fn school_name() -> &'static str {
    "Rust Study"
}

pub fn breakfast_toast() -> String {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = "Wheat".into();
    format!("{} toast with {}", meal.toast, meal.fruit())
}

pub fn menu_summary() -> String {
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
        self::hosting::greeting()
    )
}

#[cfg(test)]
mod tests {
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
