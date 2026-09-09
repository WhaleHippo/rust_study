#[derive(Default)]
struct AveragedCollection {
    // 필드는 비공개다. 외부 코드는 메서드만 통해 값을 바꾸므로 `values`와 `average`가 항상 함께 갱신된다는 불변식을 지킨다.
    values: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    fn add(&mut self, value: i32) {
        self.values.push(value);
        // 새 원소를 넣은 직후 캐시를 갱신해 이후 `average`가 오래된 값을 노출하지 않게 한다.
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
        match self.values.pop() {
            Some(value) => {
                // 제거도 상태 변경이므로, 반환 전에 캐시를 같은 컬렉션 내용으로 다시 계산한다.
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    const fn average(&self) -> Option<f64> {
        self.average
    }

    fn update_average(&mut self) {
        self.average = match self.values.as_slice() {
            // 빈 컬렉션에는 평균이 없으므로 `0.0` 같은 임의의 값 대신 `None`으로 부재를 표현한다.
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

// `&self`만 받고 구체 `Self`를 반환하거나 제네릭 메서드를 요구하지 않으므로, 이 트레이트는 객체 안전하며 `dyn Draw`로 만들 수 있다.
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
    // `Box`는 크기가 다른 구현체를 같은 크기의 포인터로 감싸 하나의 Vec에 저장하게 한다.
    // `dyn Draw`의 호출은 실행 중 vtable을 통해 실제 Button 또는 SelectBox 구현으로 동적 디스패치된다.
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn render(&self) -> Vec<&str> {
        // 호출 지점은 구체 타입을 알 필요가 없고, 각 객체의 `draw` 구현이 선택된다.
        self.components
            .iter()
            .map(|component| component.draw())
            .collect()
    }
}

fn main() {
    println!("Chapter 18: Object-Oriented Programming Features");

    println!("\n== Encapsulated State ==");
    let mut collection = AveragedCollection::default();
    collection.add(10);
    collection.add(20);
    println!("encapsulated average: {:?}", collection.average());
    let removed = collection.remove();
    println!(
        "removed: {removed:?}, updated average: {:?}",
        collection.average()
    );
    println!("Only methods can update the private values and their cached average.");

    println!("\n== Trait Objects and Dynamic Dispatch ==");
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
    println!("One Screen renders different concrete types through dyn Draw.");
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
    fn updates_remaining_average_when_one_of_multiple_values_is_removed() {
        // Given: a collection containing three values.
        let mut collection = AveragedCollection::default();
        collection.add(10);
        collection.add(20);
        collection.add(30);

        // When: the most recently added value is removed.
        let removed = collection.remove();

        // Then: the cached average reflects only the remaining values.
        assert_eq!(removed, Some(30));
        assert_eq!(collection.average(), Some(15.0));
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
