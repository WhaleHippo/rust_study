use std::{
    fmt,
    process::ExitCode,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

#[derive(Debug, PartialEq, Eq)]
enum LessonError {
    ThreadPanicked,
    ChannelClosed,
    LockPoisoned,
}

impl fmt::Display for LessonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreadPanicked => write!(formatter, "a worker thread panicked"),
            Self::ChannelClosed => write!(formatter, "the channel receiver closed"),
            Self::LockPoisoned => write!(formatter, "the shared-state lock was poisoned"),
        }
    }
}

impl std::error::Error for LessonError {}

fn moved_thread_sum(values: Vec<i32>) -> Result<i32, LessonError> {
    // `move` 클로저는 `values`의 소유권을 새 스레드로 넘긴다. 따라서 스레드 수명 동안 원본 스택을 빌리지 않는다.
    let worker = thread::spawn(move || values.into_iter().sum());
    // `join`은 작업자가 끝날 때까지 기다리고, 정상 반환값 또는 패닉을 결과로 회수한다.
    match worker.join() {
        Ok(total) => Ok(total),
        Err(_) => Err(LessonError::ThreadPanicked),
    }
}

type Worker = JoinHandle<Result<(), LessonError>>;

fn join_workers(workers: Vec<Worker>) -> Result<(), LessonError> {
    // 오류가 난 뒤에도 모든 핸들을 join하여 작업자 종료를 보장하고, 처음 관찰한 오류를 보존한다.
    let mut outcome = Ok(());
    for worker in workers {
        let current = match worker.join() {
            Ok(result) => result,
            Err(_) => Err(LessonError::ThreadPanicked),
        };
        outcome = match outcome {
            Ok(()) => current,
            Err(error) => Err(error),
        };
    }
    outcome
}

fn collect_messages(batches: Vec<Vec<i32>>) -> Result<Vec<i32>, LessonError> {
    // 여러 생산자가 하나의 수신기로 값을 보낼 수 있는 다중 생산자 채널을 만든다.
    let (sender, receiver) = mpsc::channel();
    let mut workers = Vec::with_capacity(batches.len());
    for batch in batches {
        // 각 스레드는 자신만의 송신기 핸들을 소유한다. 같은 생산자 안의 순서는 보장되지만 생산자 간 도착 순서는 보장되지 않는다.
        let producer = sender.clone();
        workers.push(thread::spawn(move || {
            for message in batch {
                producer
                    .send(message)
                    .map_err(|_| LessonError::ChannelClosed)?;
            }
            Ok(())
        }));
    }
    // 원래 송신기를 버려야 모든 생산자 종료 뒤 채널이 닫히고 `into_iter`가 끝을 알 수 있다.
    drop(sender);
    join_workers(workers)?;
    let mut messages: Vec<_> = receiver.into_iter().collect();
    // 스케줄링에 따른 도착 순서를 정렬해, 이 예제의 관찰 가능한 결과를 결정적으로 만든다.
    messages.sort_unstable();
    Ok(messages)
}

fn increment_shared(worker_count: usize) -> Result<usize, LessonError> {
    // `Arc`는 스레드 간 소유권을 원자적으로 공유하고, `Mutex`는 한 번에 한 작업자만 내부 값에 접근하게 한다.
    let counter = Arc::new(Mutex::new(0));
    let mut workers = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        // `Arc::clone`은 카운터 값을 복사하지 않고 또 하나의 공유 소유권만 만든다.
        let shared_counter = Arc::clone(&counter);
        workers.push(thread::spawn(move || {
            // 잠금 가드는 스코프가 끝날 때 자동 해제된다. 보유 중 패닉이 나면 이후 잠금은 poison 오류를 보고한다.
            let mut count = shared_counter
                .lock()
                .map_err(|_| LessonError::LockPoisoned)?;
            *count += 1;
            Ok(())
        }));
    }
    join_workers(workers)?;
    let count = *counter.lock().map_err(|_| LessonError::LockPoisoned)?;
    Ok(count)
}

fn assert_send<T: Send>() {}

fn assert_sync<T: Sync>() {}

fn prove_send_sync() {
    // `Send`는 값을 다른 스레드로 옮길 수 있음을, `Sync`는 `&T`를 스레드 간 공유해도 안전함을 뜻한다.
    assert_send::<Vec<i32>>();
    assert_sync::<Arc<Mutex<usize>>>();
}

fn run_lesson() -> Result<(), LessonError> {
    prove_send_sync();
    println!("Moved thread sum: {}", moved_thread_sum(vec![1, 2, 3, 4])?);
    println!(
        "Sorted multi-producer messages: {:?}",
        collect_messages(vec![vec![3, 1], vec![4, 2]])?
    );
    println!("Arc<Mutex<_>> counter: {}", increment_shared(4)?);
    println!("Zero-worker counter: {}", increment_shared(0)?);
    println!("Send moves ownership; Sync permits shared references.");
    Ok(())
}

fn main() -> ExitCode {
    println!("Chapter 16: Fearless Concurrency");
    match run_lesson() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Chapter 16 failed: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawned_thread_sums_moved_values() {
        // Given: owned values that can be moved into a spawned thread.
        let values = vec![1, 2, 3, 4];

        // When: the worker consumes and sums them before being joined.
        let total = moved_thread_sum(values).expect("the worker should finish");

        // Then: the joined result contains every moved value.
        assert_eq!(total, 10);
    }

    #[test]
    fn multiple_producers_return_normalized_messages() {
        // Given: batches owned by independent channel producers.
        let batches = vec![vec![3, 1], vec![4, 2]];

        // When: every producer sends and is joined.
        let messages = collect_messages(batches).expect("all producers should finish");

        // Then: scheduling cannot affect the observable order.
        assert_eq!(messages, [1, 2, 3, 4]);
    }

    #[test]
    fn shared_counter_matches_worker_count() {
        // Given: four workers sharing one synchronized counter.

        // When: every worker increments once and is joined.
        let count = increment_shared(4).expect("all workers should finish");

        // Then: no update is lost.
        assert_eq!(count, 4);
    }

    #[test]
    fn zero_workers_complete_without_blocking() {
        // Given: the edge case of no workers.

        // When: shared-state work is requested.
        let count = increment_shared(0).expect("zero workers should be valid");

        // Then: it completes immediately with the identity value.
        assert_eq!(count, 0);
    }

    #[test]
    fn one_worker_increments_shared_counter_once() {
        // Given: one worker sharing a synchronized counter.

        // When: the worker increments once and is joined.
        let count = increment_shared(1).expect("the worker should finish");

        // Then: exactly one update is recorded.
        assert_eq!(count, 1);
    }
}
