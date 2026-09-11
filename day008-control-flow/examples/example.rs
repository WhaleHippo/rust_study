use day008_control_flow::{
    Signal, countdown_steps, labeled_pair, loop_result, signal_action, sum_for,
};

fn main() {
    println!("if + while 단계={}", countdown_steps(3));
    println!("exhaustive match={}", signal_action(Signal::Green));
    println!("loop break 값={}", loop_result(21));
    println!("for 합={}", sum_for(&[10, 20, 12]));
    println!("label 탐색={:?}", labeled_pair(4, 3));
}
