use tracing::{info, instrument};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    handle_new_signature(42).await;
}

// #[instrument] - adds context to the log
#[instrument]
async fn handle_new_signature(task_index: u32) {
    info!("Using instrument in handle_new_signature. Skipping all");
    info!("Processing new signature: {task_index}");
}
