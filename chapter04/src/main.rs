// 반환된 `&str`은 새 문자열을 만들지 않고 원본 `text`의 일부를 빌려, 원본보다 오래 살 수 없게 한다.
fn first_word(text: &str) -> &str {
    // 바이트 인덱스는 UTF-8 경계에서 슬라이스를 만들기 위해 사용하며, 여기서는 ASCII 공백을 경계로 삼는다.
    for (index, byte) in text.bytes().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }
    // 공백이 없으면 입력 전체를 같은 빌림으로 반환한다.
    text
}

// 가변 빌림은 소유권을 옮기지 않지만, 빌림이 살아 있는 동안 이 문자열의 단독 변경 권한을 요구한다.
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    println!("Chapter 04: Understanding Ownership");

    // `String`은 힙 데이터를 소유한다. 대입은 깊은 복사가 아니라 소유권 이동이므로 `greeting`은 이후 사용할 수 없다.
    let greeting = String::from("hello world");
    let moved_greeting = greeting;
    // `clone`만이 별도의 힙 데이터를 복제하므로 두 문자열을 독립적으로 사용할 수 있다.
    let cloned_greeting = moved_greeting.clone();
    // 불변 빌림은 읽기 전용 접근이며 소유권은 계속 `moved_greeting`에 남는다.
    let borrowed_length = moved_greeting.len();
    // 두 결과 모두 원본을 빌린 `&str`이므로, 이들이 사용되는 동안 원본을 변경할 수 없다.
    let first = first_word(&moved_greeting);
    let greeting_slice = &moved_greeting[..5];

    // 가변 바인딩에서만 `&mut`를 만들 수 있으며, 함수 호출 뒤 값은 `!`가 붙은 문자열이 된다.
    let mut editable = String::from("borrowed mutably");
    add_exclamation(&mut editable);

    // 문자열 리터럴은 이미 빌린 `&str`이므로 복사나 소유권 이동 없이 슬라이스 함수에 전달된다.
    let spaced_phrase = "borrowed   slice";
    let spaced_first = first_word(spaced_phrase);

    println!("move: {moved_greeting}; clone: {cloned_greeting}");
    println!("borrow: length={borrowed_length}; slice={greeting_slice}; first={first}");
    println!("mutable borrow: {editable}");
    println!("multiple spaces: first_word({spaced_phrase:?})={spaced_first:?}");
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn returns_prefix_when_text_contains_space() {
        // 준비: 첫 단어 뒤에 일반적인 공백 경계가 있다.
        let text = "hello world";
        // 실행: 첫 공백 전까지의 빌린 슬라이스를 요청한다.
        let word = first_word(text);
        // 검증: 인덱스가 공백 바로 앞에서 끝나므로 "hello"만 보인다.
        assert_eq!(word, "hello");
    }

    #[test]
    fn returns_entire_slice_when_text_has_one_word() {
        // 준비: 공백이 없는 한 단어 입력이다.
        let text = "ownership";
        // 실행: 탐색이 끝날 때까지 공백을 찾지 못한다.
        let word = first_word(text);
        // 검증: 대체 할당 없이 원본 전체 `&str`이 반환된다.
        assert_eq!(word, "ownership");
    }

    #[test]
    fn returns_empty_slice_when_text_starts_with_space() {
        // 준비: 첫 바이트가 곧 단어 경계인 입력이다.
        let text = " leading";
        // 실행: 경계를 인덱스 0에서 발견한다.
        let word = first_word(text);
        // 검증: `&text[..0]`은 유효한 빈 슬라이스다.
        assert_eq!(word, "");
    }

    #[test]
    fn returns_prefix_when_words_have_multiple_spaces_between_them() {
        // 준비: 단어 사이에 공백이 여러 개 있지만 첫 공백이 경계다.
        let text = "borrowed   slice";
        // 실행: 바이트 순회는 첫 공백에서 즉시 반환한다.
        let word = first_word(text);
        // 검증: 연속 공백 수와 무관하게 첫 단어만 빌려 반환한다.
        assert_eq!(word, "borrowed");
    }
}
