struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    const fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }

    const fn x(&self) -> &X {
        &self.x
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;

    fn preview(&self) -> String {
        format!("(Read more from {})", self.summarize())
    }
}

struct Article<'a> {
    headline: &'a str,
    author: &'a str,
}

impl Summary for Article<'_> {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

fn largest<T: Ord>(values: &[T]) -> Option<&T> {
    values.iter().max()
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn notify(item: &impl Summary) -> String {
    item.preview()
}

fn pair_summary<T>(left: &T, right: &T) -> String
where
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

    #[test]
    fn longest_when_references_have_equal_lengths() {
        // Given: two different borrowed strings with equal lengths.
        let left = String::from("left");
        let right = String::from("stay");
        // When: their shared-lifetime longest value is selected.
        let value = longest(&left, &right);
        // Then: ties preserve the left reference.
        assert!(std::ptr::eq(value, left.as_str()));
    }
}
