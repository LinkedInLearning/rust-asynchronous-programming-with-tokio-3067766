use std::sync::Arc;

use tokio::sync::{Barrier, BarrierWaitResult, Notify};
use tokio::time::{sleep, Duration};

async fn barrier_example(barrier: Arc<Barrier>, notify: Arc<Notify>) -> BarrierWaitResult {
    println!("Waiting at barrier");

    let wait_result = barrier.wait().await;
    println!("Passed through the barrier");

    if wait_result.is_leader() {
        notify.notify_one();
    }

    wait_result
}

#[tokio::main]
async fn main() {
    let total_cans_needed = 12;
    let barrier = Arc::new(Barrier::new(total_cans_needed));

    let notify = Arc::new(Notify::new());

    // To send the first batch of cans to the barrier
    notify.notify_one();

    let mut task_handles = Vec::new();
    for can_count in 0..60 {
        if can_count % 12 == 0 {
            notify.notified().await;

            // Give the barrier some time to close
            sleep(Duration::from_millis(1)).await;
        }

        task_handles.push(tokio::spawn(barrier_example(
            barrier.clone(),
            notify.clone(),
        )));
    }

    let mut num_of_leaders = 0;
    for handle in task_handles {
        let wait_result = handle.await.unwrap();

        if wait_result.is_leader() {
            num_of_leaders += 1;
        }
    }

    println!("total num of leaders: {}", num_of_leaders);
}
