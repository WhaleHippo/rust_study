use day009_move_copy_clone::{append_owned, copy_pair};
use std::rc::Rc;

fn main() {
    let moved = append_owned(String::from("rust"), "acean");
    println!("move 뒤 새 소유자: {moved}");

    let number = 21;
    let copied = copy_pair(number);
    println!("Copy 뒤 원본 {number}, 두 값: {copied:?}");

    let original = String::from("clone");
    let mut independent = original.clone();
    independent.push('!');
    println!("독립 String: 원본={original}, 복제본={independent}");

    let shared = Rc::new(String::from("shared"));
    let shared_clone = Rc::clone(&shared);
    println!(
        "Rc Clone은 같은 할당을 공유: {}, 강한 참조={}",
        Rc::ptr_eq(&shared, &shared_clone),
        Rc::strong_count(&shared)
    );
}
