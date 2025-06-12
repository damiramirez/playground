use tracing::{debug, info, instrument, level_filters::LevelFilter};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    handle_new_signature(42);
    handle_new_task(42);

    let len = 10;
    let i = 1;
    let to_block = 100;

    debug!("numTransactionLogs: {len}, fromBlock: {i}, toBlock: {to_block}");

    debug!(
        num_transaction_logs = len,
        from_block = i,
        to_block = to_block,
    );
}

// #[instrument] - adds context to the log
#[instrument(skip_all)]
fn handle_new_signature(task_index: u32) {
    info!("(USING INSTRUMENT SKIP ALL) Processing new signature: {task_index}");
}

fn handle_new_task(task_index: u32) {
    info!(target: "handle_new_task", "(USING TARGET) Processing new task: {task_index}");
}
