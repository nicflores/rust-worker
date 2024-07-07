mod health;

use rust_worker::worker::dowork::perform_work;
use tokio::sync::oneshot;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let task_duration = std::env::var("TASK_DURATION").unwrap_or_else(|_| "10".to_string());
    let task_duration: u64 = task_duration.parse().unwrap_or(10);

    // Create the health router
    let health_router = health::api::router();
    let app = health_router;

    // Channel to signal the server to shut down
    let (shutdown_tx, shutdown_rx): (oneshot::Sender<()>, oneshot::Receiver<()>) =
        oneshot::channel();

    // Create a TCP listener and serve the app on the listener.
    // This is the main event loop that listens for incoming requests.
    // Spawn a task to run the web server
    let server_task = tokio::spawn(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        println!("Listening on {}", listener.local_addr().unwrap());

        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
            })
            .await
            .unwrap()
    });

    // Perform the work
    perform_work(task_duration).await;

    // Signal the server to shut down
    let _ = shutdown_tx.send(());

    // Wait for the server task to complete
    let _ = server_task.await;
}
