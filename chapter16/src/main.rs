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
    let worker = thread::spawn(move || values.into_iter().sum());
    match worker.join() {
        Ok(total) => Ok(total),
        Err(_) => Err(LessonError::ThreadPanicked),
    }
}

type Worker = JoinHandle<Result<(), LessonError>>;

fn join_workers(workers: Vec<Worker>) -> Result<(), LessonError> {
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
    let (sender, receiver) = mpsc::channel();
    let mut workers = Vec::with_capacity(batches.len());
    for batch in batches {
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
    drop(sender);
    join_workers(workers)?;
    let mut messages: Vec<_> = receiver.into_iter().collect();
    messages.sort_unstable();
    Ok(messages)
}

fn increment_shared(worker_count: usize) -> Result<usize, LessonError> {
    let counter = Arc::new(Mutex::new(0));
    let mut workers = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        let shared_counter = Arc::clone(&counter);
        workers.push(thread::spawn(move || {
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
