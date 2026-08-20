#[derive(Default)]
struct AveragedCollection {
    values: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    fn add(&mut self, value: i32) {
        self.values.push(value);
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
        match self.values.pop() {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn average(&self) -> Option<f64> {
        self.average
    }

    fn update_average(&mut self) {
        self.average = match self.values.as_slice() {
            [] => None,
            values => {
                let (total, count) = values.iter().fold((0.0, 0.0), |(total, count), value| {
                    (total + f64::from(*value), count + 1.0)
                });
                Some(total / count)
            }
        };
    }
}

trait Draw {
    fn draw(&self) -> &str;
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) -> &str {
        &self.label
    }
}

struct SelectBox {
    selected: String,
}

impl Draw for SelectBox {
    fn draw(&self) -> &str {
        &self.selected
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn render(&self) -> Vec<&str> {
        self.components
            .iter()
            .map(|component| component.draw())
            .collect()
    }
}

fn main() {
    println!("Chapter 18: Object-Oriented Programming Features");

    let mut collection = AveragedCollection::default();
    collection.add(10);
    collection.add(20);
    println!("encapsulated average: {:?}", collection.average());
    let removed = collection.remove();
    println!(
        "removed: {removed:?}, updated average: {:?}",
        collection.average()
    );

    let screen = Screen {
        components: vec![
            Box::new(Button {
                label: String::from("Save"),
            }),
            Box::new(SelectBox {
                selected: String::from("Rust"),
            }),
        ],
    };
    println!("dynamic dispatch: {:?}", screen.render());
}

#[cfg(test)]
mod tests {
    use super::{AveragedCollection, Button, Draw, Screen, SelectBox};

    #[test]
    fn averages_values_when_collection_changes() {
        // Given: an encapsulated collection with two values.
        let mut collection = AveragedCollection::default();
        collection.add(10);
        collection.add(20);
        // When: its cached average is requested.
        let average = collection.average();
        // Then: the average reflects both values.
        assert_eq!(average, Some(15.0));
    }

    #[test]
    fn returns_none_when_collection_is_empty() {
        // Given: an empty encapsulated collection.
        let collection = AveragedCollection::default();
        // When: its average is requested.
        let average = collection.average();
        // Then: absence is represented explicitly.
        assert_eq!(average, None);
    }

    #[test]
    fn clears_average_when_last_value_is_removed() {
        // Given: a collection containing one value.
        let mut collection = AveragedCollection::default();
        collection.add(7);
        // When: the only value is removed.
        let result = (collection.remove(), collection.average());
        // Then: the value is returned and the average becomes absent.
        assert_eq!(result, (Some(7), None));
    }

    #[test]
    fn renders_different_components_through_trait_objects() {
        // Given: heterogeneous components behind one object-safe trait.
        let components: Vec<Box<dyn Draw>> = vec![
            Box::new(Button {
                label: String::from("Save"),
            }),
            Box::new(SelectBox {
                selected: String::from("Rust"),
            }),
        ];
        let screen = Screen { components };
        // When: the screen renders through dynamic dispatch.
        let rendered = screen.render();
        // Then: each concrete component supplies its own output.
        assert_eq!(rendered, vec!["Save", "Rust"]);
    }
}
