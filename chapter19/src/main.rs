#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

const fn classify_point(point: Point) -> &'static str {
    // 구조체 패턴이 필드를 지역 변수로 분해한다. `..`는 필드가 추가되더라도 나머지를 무시하겠다는 패턴 문법이다.
    let Point { x, y, .. } = point;

    match (x, y) {
        // match 팔은 위에서 아래로 처음 맞는 것을 선택한다. 따라서 원점은 뒤의 축 OR 패턴보다 먼저 둔다.
        (0, 0) => "origin",
        // `@`는 `1..=5` 범위와 일치한 값을 `coordinate`에도 바인딩하고, guard는 구조적 일치 뒤 추가 조건을 검사한다.
        (coordinate @ 1..=5, other) if coordinate == other => "small diagonal",
        // `|`는 어느 쪽 패턴이든 맞으면 같은 팔을 선택한다. 이 예제는 추가 바인딩이 없어 두 대안의 바인딩 규칙도 단순하다.
        (0, _) | (_, 0) => "axis",
        (_, vertical) if vertical < 0 => "below",
        (_, _) => "other",
    }
}

const fn pair_total((left, right): (i32, i32)) -> i32 {
    // 함수 매개변수 위치에서도 패턴으로 튜플을 즉시 두 값으로 분해할 수 있다.
    left + right
}

fn sum_pairs(pairs: &[(i32, i32)]) -> i32 {
    let mut total = 0;
    // `&`는 슬라이스에서 얻은 참조를 맞추고, `(left, right)`는 각 튜플 위치의 값을 분해한다.
    for &(left, right) in pairs {
        total += pair_total((left, right));
    }
    total
}

fn drain_stack(values: &[i32]) -> Vec<i32> {
    let mut stack = values.to_vec();
    let mut drained = Vec::with_capacity(stack.len());
    // `pop`이 `Some`을 반환하는 동안만 값을 꺼내며, 스택이 비어 `None`이 되는 순간 조건 불일치로 종료한다.
    while let Some(value) = stack.pop() {
        drained.push(value);
    }
    drained
}

fn describe_message(message: Message) -> String {
    // 제어하는 enum의 네 변형을 모두 명시해, 새 변형이 추가되면 컴파일러가 이 match 갱신을 요구한다.
    match message {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(red, green, blue) => format!("color: {red}, {green}, {blue}"),
    }
}

fn main() {
    println!("Chapter 19: Patterns and Matching");

    println!("\n== Ranges, OR Patterns, Guards, and @ Bindings ==");
    let point = Point { x: 3, y: 3 };
    println!("point: {}", classify_point(point));
    for (label, point) in [
        ("origin", Point { x: 0, y: 0 }),
        ("vertical axis", Point { x: 0, y: 7 }),
        ("below", Point { x: 2, y: -1 }),
    ] {
        println!("{label}: {}", classify_point(point));
    }

    println!("\n== for Destructuring ==");
    let pairs = [(1, 2), (3, 4)];
    println!("pair total: {}", sum_pairs(&pairs));

    println!("\n== while let and if let ==");
    let drained = drain_stack(&[1, 2, 3]);
    if let Some(last) = drained.last() {
        println!("while-let drained last value: {last}");
    }

    println!("\n== Exhaustive Enum Destructuring ==");
    for message in [
        Message::Quit,
        Message::Move { x: 2, y: -1 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(10, 20, 30),
    ] {
        println!("message: {}", describe_message(message));
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, Point, classify_point, describe_message, drain_stack, sum_pairs};

    #[test]
    fn classifies_small_diagonal_when_range_and_guard_match() {
        // Given: a point whose coordinates share a small positive value.
        let point = Point { x: 3, y: 3 };

        // When: tuple and struct patterns classify the point.
        let classification = classify_point(point);

        // Then: the range binding and guard select the diagonal case.
        assert_eq!(classification, "small diagonal");
    }

    #[test]
    fn classifies_origin_when_both_coordinates_are_zero() {
        // Given: the point where both axes meet.
        let point = Point { x: 0, y: 0 };

        // When: the point is classified.
        let classification = classify_point(point);

        // Then: the most specific origin pattern wins.
        assert_eq!(classification, "origin");
    }

    #[test]
    fn classifies_axis_when_exactly_one_coordinate_is_zero() {
        // Given: points on the vertical and horizontal axes.
        let points = [Point { x: 0, y: 7 }, Point { x: -4, y: 0 }];

        // When: each point is classified.
        let classifications = points.map(classify_point);

        // Then: the OR pattern recognizes either axis.
        assert_eq!(classifications, ["axis", "axis"]);
    }

    #[test]
    fn keeps_drained_values_empty_when_stack_is_empty() {
        // Given: an empty stack slice.
        let values = [];

        // When: a while-let pattern drains the stack.
        let drained = drain_stack(&values);

        // Then: no value is produced.
        assert_eq!(drained, Vec::<i32>::new());
    }

    #[test]
    fn sums_pairs_when_for_pattern_destructures_each_item() {
        // Given: two integer pairs.
        let pairs = [(1, 2), (3, 4)];

        // When: a for pattern destructures and sums them.
        let total = sum_pairs(&pairs);

        // Then: every tuple member contributes.
        assert_eq!(total, 10);
    }

    #[test]
    fn describes_every_message_variant_without_fallback() {
        // Given: one value of every owned enum variant.
        let messages = [
            Message::Quit,
            Message::Move { x: 2, y: -1 },
            Message::Write(String::from("hello")),
            Message::ChangeColor(10, 20, 30),
        ];

        // When: the exhaustive match describes each message.
        let descriptions: Vec<String> = messages.into_iter().map(describe_message).collect();

        // Then: each variant's destructured data appears in its result.
        assert_eq!(
            descriptions,
            vec![
                String::from("quit"),
                String::from("move to (2, -1)"),
                String::from("write: hello"),
                String::from("color: 10, 20, 30"),
            ]
        );
    }
}
