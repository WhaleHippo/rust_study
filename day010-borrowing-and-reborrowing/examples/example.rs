use day010_borrowing_and_reborrowing::{increment_twice, read_then_append, shared_length};

fn main() {
    let text = String::from("rust");
    let first_length = shared_length(&text);
    let second_length = shared_length(&text);
    println!("공유 빌림 둘: {first_length}, {second_length}; 소유자={text}");

    let mut number = 40;
    increment_twice(&mut number);
    println!("짧은 재빌림 두 번 뒤: {number}");

    let mut label = String::from("NLL");
    let old_length = read_then_append(&mut label);
    println!("공유 참조의 마지막 사용 뒤 변경: 이전 길이={old_length}, 값={label}");
}
