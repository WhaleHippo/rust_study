// 이름 있는 필드는 관련 데이터를 한 값으로 묶으며, 각 필드의 타입이 저장 가능한 값을 제한한다.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 튜플 구조체는 필드 이름 대신 위치를 사용하지만 `Color`라는 별도 타입을 만든다.
struct Color(i32, i32, i32);

// 단위 구조체는 저장 데이터 없이 타입 자체로만 의미를 표현한다.
struct AlwaysEqual;

// 사각형의 두 치수를 함께 보관해 계산에 필요한 관계를 한 값으로 유지한다.
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 메서드는 `&self`로 사각형을 빌려 읽기만 하므로 호출 뒤에도 원본을 계속 쓸 수 있다.
    const fn area(&self) -> u32 {
        self.width * self.height
    }

    // 두 비교가 모두 참이어야 하므로, 한 변이라도 같거나 크면 포함으로 취급하지 않는다.
    const fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 연관 함수는 인스턴스 대신 타입에서 호출하며, 같은 크기의 두 변으로 정사각형을 만든다.
    const fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 매개변수의 소유권을 `User` 필드로 이동하고, 같은 이름의 필드는 축약 초기화로 채운다.
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

    // 함수가 만든 구조체 하나가 계정의 서로 관련된 상태를 일관되게 전달한다.
    let user = build_user(
        String::from("learner@example.com"),
        String::from("rustacean"),
    );
    // 구조체 갱신 문법은 `email`만 새 값으로 바꾼다. 나머지 중 `username: String`은 이동하고,
    // `Copy` 타입인 `active: bool`과 `sign_in_count: u64`는 복사한다.
    let updated_user = User {
        email: String::from("updated@example.com"),
        ..user
    };
    // 위치 필드는 `.0`, `.1`, `.2`로 접근한다. RGB 성분 순서는 타입 정의가 정한다.
    let black = Color(0, 0, 0);
    // 밑줄 접두사는 예제용 단위 구조체가 의도적으로 사용되지 않음을 컴파일러에 알린다.
    let _subject = AlwaysEqual;
    let screen = Rectangle {
        width: 30,
        height: 50,
    };
    // `square`가 반환한 새 구조체와 비교하므로, `screen`과 `icon`은 각각 독립적인 값이다.
    let icon = Rectangle::square(10);
    let too_wide_icon = Rectangle {
        width: 31,
        height: 10,
    };

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
    println!(
        "one dimension too large: holds_icon={}",
        screen.can_hold(&too_wide_icon)
    );
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn computes_area_when_rectangle_has_width_and_height() {
        // 준비: 너비와 높이가 모두 있는 사각형이다.
        let rectangle = Rectangle {
            width: 3,
            height: 10,
        };
        // 실행: 읽기 전용 빌림으로 넓이를 계산한다.
        let area = rectangle.area();
        // 검증: 넓이는 두 필드의 곱인 30이다.
        assert_eq!(area, 30);
    }

    #[test]
    fn contains_smaller_rectangle_when_both_dimensions_fit() {
        // 준비: 바깥 사각형의 두 변이 모두 더 길다.
        let outer = Rectangle::square(10);
        let inner = Rectangle::square(9);
        // 실행: 두 구조체를 이동하지 않고 빌려 포함 여부를 확인한다.
        let contains_inner = outer.can_hold(&inner);
        // 검증: 두 비교가 참이므로 포함된다.
        assert!(contains_inner);
    }

    #[test]
    fn rejects_equal_rectangle_when_no_dimension_is_larger() {
        // 준비: 두 사각형의 치수가 완전히 같다.
        let outer = Rectangle::square(10);
        let equal = Rectangle::square(10);
        // 실행: 엄격한 `>` 비교로 포함 여부를 계산한다.
        let contains_equal = outer.can_hold(&equal);
        // 검증: 같은 크기는 어느 변도 더 크지 않아 포함이 아니다.
        assert!(!contains_equal);
    }

    #[test]
    fn rejects_rectangle_when_one_dimension_is_too_large() {
        // 준비: 후보는 너비는 맞지만 높이가 바깥 사각형보다 크다.
        let outer = Rectangle {
            width: 10,
            height: 10,
        };
        let too_tall = Rectangle {
            width: 9,
            height: 11,
        };
        // 실행: 두 차원을 독립적으로 비교한다.
        let contains_too_tall = outer.can_hold(&too_tall);
        // 검증: 한 차원만 초과해도 `&&` 전체가 거짓이 되어 거부된다.
        assert!(!contains_too_tall);
    }
}
