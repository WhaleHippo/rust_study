struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Chapter 05: Using Structs to Structure Related Data");

    let user = build_user(
        String::from("learner@example.com"),
        String::from("rustacean"),
    );
    let updated_user = User {
        email: String::from("updated@example.com"),
        ..user
    };
    let black = Color(0, 0, 0);
    let _subject = AlwaysEqual;
    let screen = Rectangle {
        width: 30,
        height: 50,
    };
    let icon = Rectangle::square(10);

    println!(
        "user: active={}, name={}, email={}, sign_ins={}",
        updated_user.active, updated_user.username, updated_user.email, updated_user.sign_in_count
    );
    println!("tuple struct RGB=({}, {}, {})", black.0, black.1, black.2);
    println!(
        "rectangle area={}, holds_icon={}",
        screen.area(),
        screen.can_hold(&icon)
    );
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn computes_area_when_rectangle_has_width_and_height() {
        // Given: a rectangle with nonzero dimensions.
        let rectangle = Rectangle {
            width: 3,
            height: 10,
        };
        // When: its area method is called.
        let area = rectangle.area();
        // Then: width is multiplied by height.
        assert_eq!(area, 30);
    }

    #[test]
    fn contains_smaller_rectangle_when_both_dimensions_fit() {
        // Given: a larger rectangle and a smaller one.
        let outer = Rectangle::square(10);
        let inner = Rectangle::square(9);
        // When: containment is checked.
        let contains_inner = outer.can_hold(&inner);
        // Then: the larger rectangle can hold the smaller one.
        assert!(contains_inner);
    }

    #[test]
    fn rejects_equal_rectangle_when_no_dimension_is_larger() {
        // Given: two rectangles with equal dimensions.
        let outer = Rectangle::square(10);
        let equal = Rectangle::square(10);
        // When: containment is checked.
        let contains_equal = outer.can_hold(&equal);
        // Then: equal size is not treated as containment.
        assert!(!contains_equal);
    }
}
