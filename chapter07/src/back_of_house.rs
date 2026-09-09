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
