struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    // 호출 인수에서 X와 Y를 추론한다. 컴파일러는 실제로 쓰인 각 타입 조합에 맞는 코드를
    // 단형화(monomorphization)하므로, 런타임에 타입 태그를 검사하는 범용 컨테이너가 아니다.
    const fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }

    const fn x(&self) -> &X {
        &self.x
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        // self와 other를 값으로 받으므로 두 Point의 소유권이 이 메서드로 이동한다. 아래 필드를
        // 꺼내 새 Point로 다시 소유시키며, 결과는 self.x의 타입 X와 other.y의 타입 Y2를 결합한다.
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

trait Summary {
    // 구현체마다 반드시 제공해야 하는 요구 메서드다. 이 메서드가 없으면 Summary를 구현할 수 없다.
    fn summarize(&self) -> String;

    // 기본 메서드는 모든 구현체가 재사용할 수 있으며, 여기서는 요구 메서드 summarize에 의존한다.
    // 구현체는 필요할 때만 이를 재정의한다.
    fn preview(&self) -> String {
        format!("(Read more from {})", self.summarize())
    }
}

struct Article<'a> {
    // Article은 문자열을 소유하지 않고 빌린다. 'a는 두 필드의 참조가 Article보다 오래 살아야 함을 나타낸다.
    headline: &'a str,
    author: &'a str,
}

impl Summary for Article<'_> {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

fn largest<T: Ord>(values: &[T]) -> Option<&T> {
    // T: Ord가 비교 가능함을 보장한다. max는 새 T를 만들거나 복제하지 않고 입력 슬라이스 안의
    // 원소를 빌려 반환하므로, Option<&T>는 결과가 values보다 오래 살 수 없음을 표현한다.
    values.iter().max()
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    // 'a는 세 참조의 관계를 선언한다. 반환 참조는 left와 right가 모두 유효한 공통 기간을 넘을 수 없다.
    // `>=`는 길이가 같을 때도 왼쪽 참조를 반환하는 명시적 계약이다.
    // C10-01은 값의 동등성만으로는 이 선택을 증명할 수 없고 참조 정체성을 확인해야 함을 보여 준다.
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn notify(item: &impl Summary) -> String {
    // impl Trait은 이 호출마다 구체적인 Summary 구현체를 컴파일 시에 정하는 정적 디스패치 문법이다.
    item.preview()
}

fn pair_summary<T>(left: &T, right: &T) -> String
where
    // where 절은 같은 T가 Summary를 구현해야 한다는 제약을 서명 뒤에 읽기 쉽게 분리하며,
    // 이 제네릭 호출도 구체 타입별로 단형화되는 정적 디스패치다.
    T: Summary,
{
    format!("{} | {}", left.summarize(), right.summarize())
}

fn main() {
    println!("Chapter 10: Generic Types, Traits, and Lifetimes");
    let first = Article {
        headline: "Rust traits",
        author: "Ferris",
    };
    let second = Article {
        headline: "Lifetimes",
        author: "Rustacean",
    };
    let mixed = Point::new(5, "left").mixup(Point::new('y', 10));
    println!("{}", notify(&first));
    println!("{}", pair_summary(&first, &second));
    println!("mixed x: {}", mixed.x());
    println!("largest: {:?}", largest(&[3, 9, 4]));
    println!("longest: {}", longest("short", "longer"));
}

#[cfg(test)]
mod tests {
    use super::{Article, Point, largest, longest, notify, pair_summary};

    #[test]
    fn largest_when_slice_has_values() {
        // Given: several ordered values.
        // When: the largest value is requested.
        let values = [3, 9, 4];
        // Then: it returns the maximum by reference.
        assert_eq!(largest(&values), Some(&9));
    }

    #[test]
    fn largest_when_slice_is_empty() {
        // Given: an empty slice of ordered values.
        // When: the largest value is requested.
        let values: [i32; 0] = [];
        // Then: Option represents the absence of a maximum without panicking.
        assert_eq!(largest(&values), None);
    }

    #[test]
    fn point_mixup_when_coordinates_have_different_types() {
        // Given: two generic points with different coordinate types.
        // When: their generic mixup method is called.
        let point = Point::new(5, "left").mixup(Point::new('y', 10));
        // Then: the resulting point keeps self.x and other.y.
        assert_eq!((*point.x(), point.y), (5, 10));
    }

    #[test]
    fn notify_when_summary_uses_the_default_trait_method() {
        // Given: an article implementing Summary.
        // When: it is passed through an impl Trait parameter.
        let article = Article {
            headline: "Rust traits",
            author: "Ferris",
        };
        // Then: notify uses the trait's default preview method.
        assert_eq!(notify(&article), "(Read more from Rust traits, by Ferris)");
    }

    #[test]
    fn pair_summary_when_where_bound_is_satisfied() {
        // Given: two values implementing Summary.
        // When: the generic where-bound function summarizes them.
        let first = Article {
            headline: "One",
            author: "Ada",
        };
        let second = Article {
            headline: "Two",
            author: "Lin",
        };
        // Then: both summaries are joined in input order.
        assert_eq!(pair_summary(&first, &second), "One, by Ada | Two, by Lin");
    }

    #[test]
    fn longest_when_references_have_different_lengths() {
        // Given: two borrowed string slices with different lengths.
        // When: their shared-lifetime longest value is selected.
        let value = longest("short", "longer");
        // Then: the returned borrow points to the longer slice.
        assert_eq!(value, "longer");
    }

    // [학습 실습: C10-01]
    // 목표: 수명 계약을 유지하며 동률에서 왼쪽 참조를 선택한다.
    // 학습자 행동: `>=`를 `>`로 잠시 바꾸고 아래 테스트만 실행한다.
    // RED: `cargo test -p chapter10-generic-types-traits-and-lifetimes longest_when_references_have_equal_lengths`는 오른쪽 참조가 반환되어 포인터 검증에 실패한다.
    // GREEN: `>=`로 복원해 동률에서 왼쪽 참조를 반환한다.
    // 힌트: 같은 문자열 내용 비교는 통과할 수 있으므로 `std::ptr::eq`로 빌린 대상을 확인한다.
    #[test]
    fn longest_when_references_have_equal_lengths() {
        // 준비: 길이는 같지만 서로 다른 두 빌린 문자열을 준비한다.
        let left = String::from("left");
        let right = String::from("stay");
        // 실행: 공통 수명의 longest 값을 선택한다.
        let value = longest(&left, &right);
        // 검증: 동률에서는 왼쪽 참조를 보존한다.
        assert!(std::ptr::eq(value, left.as_str()));
    }
}
