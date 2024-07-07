use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

pub async fn perform_work(task_duration: u64) {
    info!("Starting task with duration: {} seconds", task_duration);
    for i in 1..=task_duration {
        info!("Working... {} seconds", i);
        sleep(Duration::from_secs(1)).await;
    }
    info!("Task completed!");
}
