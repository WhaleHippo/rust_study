use std::collections::HashMap;

fn vector_lesson() -> Vec<i32> {
    // Vec는 힙 버퍼의 소유자다. 여기서는 지역 변수 values가 버퍼를 소유하고,
    // mut 바인딩이므로 push로 길이와 (필요하면 재할당된) 버퍼를 바꿀 수 있다.
    let mut values = vec![1, 2, 3];
    values.push(4);
    // 반환 시 Vec 자체가 이동하여 호출자가 소유권을 받는다. 따라서 복사나 clone이 필요 없다.
    values
}

fn character_count(text: &str) -> usize {
    // &str은 UTF-8 바이트열이다. len()은 바이트 수를 세지만 chars()는 유니코드 스칼라 값을
    // 순회하므로, 여러 바이트로 저장되는 🌍도 여기서는 문자 하나로 센다. 다만 chars()도
    // 사용자에게 보이는 조합 문자 단위(그래핌 클러스터)와 항상 같지는 않다.
    text.chars().count()
}

fn word_frequencies(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    // split_whitespace가 만든 word는 입력 text를 가리키는 빌린 슬라이스다. HashMap은 키 문자열을
    // 복제하지 않고 그 참조를 저장하므로, 반환된 맵은 원본 text보다 오래 살 수 없다는 관계가 타입에 담긴다.
    for word in text.split_whitespace() {
        // entry는 키를 한 번만 조회한다. 이미 있으면 and_modify가 먼저 기존 횟수를 늘리고,
        // 없으면 이어지는 or_insert가 1을 넣어 삽입과 갱신을 한 흐름으로 연결한다.
        counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    counts
}

fn main() {
    println!("Chapter 08: Common Collections");
    println!("vector: {:?}", vector_lesson());
    println!("characters in Rust: {}", character_count("Rust"));
    // HashMap의 해시 시드와 버킷 배치는 정렬 순서를 보장하지 않는다. 아래 Debug 출력의 항목 순서는
    // 실행마다 달라질 수 있으므로, 표시 순서가 필요하면 키를 정렬해 별도로 출력해야 한다.
    println!("frequencies: {:?}", word_frequencies("rust rust book"));
}

#[cfg(test)]
mod tests {
    use super::word_frequencies;

    #[test]
    fn word_frequencies_when_words_repeat() {
        // Given: repeated words in borrowed text.
        // When: frequencies are collected.
        let counts = word_frequencies("rust rust book");
        // Then: each occurrence contributes to its entry.
        assert_eq!(counts.get("rust"), Some(&2));
    }

    #[test]
    fn vector_lesson_when_a_value_is_pushed() {
        // Given: a vector with three values.
        // When: the lesson pushes another value.
        let values = super::vector_lesson();
        // Then: Vec retains the appended value in insertion order.
        assert_eq!(values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn character_count_when_text_has_utf8_characters() {
        // Given: a string slice containing a multi-byte Unicode character.
        // When: its characters are counted.
        let count = super::character_count("hi🌍");
        // Then: the count is characters rather than UTF-8 bytes.
        assert_eq!(count, 3);
    }

    #[test]
    fn word_frequencies_when_text_is_empty() {
        // Given: borrowed text without words.
        // When: entries are counted.
        let counts = word_frequencies("");
        // Then: no HashMap entries are created.
        assert!(counts.is_empty());
    }

    // [학습 실습: C08-01]
    // 목표: 같은 단어가 세 번 나오면 HashMap 빈도가 3으로 누적되는 계약을 고정한다.
    // 학습자 행동: 로컬에서 `and_modify`를 임시 제거한 뒤 기존 키의 증가 처리를 직접 복구한다.
    // RED: `cargo test -p chapter08-common-collections word_frequencies_when_one_word_occurs_three_times`가 증가하지 않은 빈도와 기대값 3의 차이로 실패한다.
    // GREEN: `and_modify(|count| *count += 1)`를 `or_insert` 앞에 복구하면 이 테스트와 장 전체 테스트가 통과한다.
    // 힌트: `and_modify`는 기존 키를 증가시키고 `or_insert`는 새 키에만 1을 넣는다.
    #[test]
    fn word_frequencies_when_one_word_occurs_three_times() {
        // 준비: 하나의 빌린 단어가 세 번 반복된다.
        // 실행: entry 흐름으로 빈도를 수집한다.
        let counts = word_frequencies("rust rust rust book");
        // 검증: 기존 키는 삽입 전에 증가되어 세 번의 발생을 모두 반영한다.
        assert_eq!(counts.get("rust"), Some(&3));
    }
}
