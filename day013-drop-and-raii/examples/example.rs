#![forbid(unsafe_code)]

use day013_drop_and_raii::{early_drop_order, field_drop_order, local_drop_order, partial_move};

fn main() {
    let summary = partial_move(
        String::from("Ferris"),
        vec![String::from("ownership"), String::from("borrowing")],
    );

    println!("부분 이동: {}, 기술 {}개", summary.0, summary.1);
    println!("지역 변수: {:?}", local_drop_order());
    println!("구조체 필드: {:?}", field_drop_order());
    println!("조기 해제: {:?}", early_drop_order());
}
