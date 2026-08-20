mod restaurant {
    pub mod front_of_house {
        pub mod hosting {
            pub fn greeting() -> &'static str {
                "Welcome"
            }
        }
    }

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Self {
                Self {
                    toast: toast.into(),
                    seasonal_fruit: "peaches".into(),
                }
            }

            pub fn fruit(&self) -> &str {
                &self.seasonal_fruit
            }
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub use front_of_house::hosting;

    pub fn breakfast_toast() -> String {
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = "Wheat".into();
        format!("{} toast with {}", meal.toast, meal.fruit())
    }

    pub fn menu_summary() -> String {
        let featured = if super::school_name() == "Rust Study" {
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
            super::school_name(),
            appetizer,
            self::hosting::greeting()
        )
    }
}

use restaurant::hosting as host;

fn school_name() -> &'static str {
    "Rust Study"
}

fn main() {
    println!("Chapter 07: Managing Growing Projects with Packages, Crates, and Modules");
    println!("{}", host::greeting());
    println!("{}", restaurant::breakfast_toast());
    println!("{}", restaurant::menu_summary());
}

#[cfg(test)]
mod tests {
    use super::host;

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
        let toast = super::restaurant::breakfast_toast();
        // Then: the public field value is observable outside the module.
        assert_eq!(toast, "Wheat toast with peaches");
    }

    #[test]
    fn menu_summary_when_relative_paths_are_used() {
        // Given: a nested module using super and self paths.
        // When: it builds the menu summary.
        let summary = super::restaurant::menu_summary();
        // Then: the parent and sibling paths resolve to the intended values.
        assert_eq!(summary, "Rust Study serves soup with Welcome");
    }
}
