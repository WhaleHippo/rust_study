// 구조체 자체는 공개지만, 각 필드는 별도로 공개 여부를 정한다.
pub struct Breakfast {
    // 공개 필드는 생성 뒤에 이 모듈 밖의 코드도 읽거나 수정할 수 있다.
    pub toast: String,
    // 비공개 필드는 이 모듈 밖에서 직접 읽거나 수정할 수 없어 내부 상태를 보호한다.
    seasonal_fruit: String,
}

impl Breakfast {
    // 비공개 필드가 있는 공개 구조체는 외부에서 구조체 리터럴로 만들 수 없다.
    // 이 공개 생성자가 계절 과일을 정하는 유일한 구성 경계가 된다.
    pub fn summer(toast: &str) -> Self {
        Self {
            toast: toast.into(),
            seasonal_fruit: "peaches".into(),
        }
    }

    // 읽기 전용 공개 메서드로 비공개 필드의 값을 노출한다.
    pub fn fruit(&self) -> &str {
        &self.seasonal_fruit
    }
}

// 열거형과 변형이 공개되어 라이브러리 내부의 다른 모듈은 선택값을 명시적으로 사용할 수 있다.
pub enum Appetizer {
    Soup,
    Salad,
}
