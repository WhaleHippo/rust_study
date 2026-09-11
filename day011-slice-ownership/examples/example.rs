use day011_slice_ownership::{first_word, tail_from};

fn main() {
    let owner = String::from("소유권 배우기");
    let word = first_word(&owner);
    println!("문자열 소유자={owner}, 빌린 첫 단어={word}");

    let numbers = [10, 20, 30, 40];
    match tail_from(&numbers, 2) {
        Some(tail) => println!("배열의 빌린 꼬리={tail:?}"),
        None => println!("시작 위치가 범위를 벗어났습니다"),
    }
}
