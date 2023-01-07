use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

async fn person(semaphore: Arc<Semaphore>, name: String) {
    println!("{} is waiting in line", name);

    teller(semaphore, name).await;
}

async fn teller(semaphore: Arc<Semaphore>, customer: String) {
    let permit = semaphore.acquire().await.unwrap();

    sleep(Duration::from_secs(2)).await;
    println!("\n{} is being served by the teller", customer);
    sleep(Duration::from_secs(5)).await;
    println!("{} is now leaving the teller", customer);

    drop(permit);
}

#[tokio::main]
async fn main() {
    let mut people_handles = Vec::new();

    let num_of_tellers = 4;
    let semaphore = Semaphore::new(num_of_tellers);
    let semaphore_arc = Arc::new(semaphore);

    for num in 0..10 {
        people_handles.push(tokio::spawn(person(
            semaphore_arc.clone(),
            format!("Person_{num}"),
        )));
    }

    for handle in people_handles {
        let _ = handle.await.unwrap();
    }
}
