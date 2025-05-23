use futures::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tarpc::{
    client, context,
    server::{self, Channel},
};

#[derive(Debug)]
struct Task {
    id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenericTask<T> {
    id: u32,
    name: String,
    description: String,
    data: T,
}

// This is the service definition. It looks a lot like a trait definition.
// It defines one RPC
#[tarpc::service]
trait TaskProcessor {
    async fn process_task(task: Task) -> String;

    async fn process_generic_task(task: String) -> String;
}

// This is the type that implements the generated TaskProcessor trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct TaskProcessorServer;

impl TaskProcessor for TaskProcessorServer {
    // Each defined rpc generates an async fn that serves the RPC
    async fn process_task(self, _: context::Context, task: Task) -> String {
        // Simulate a long running task
        tokio::time::sleep(Duration::from_secs(5)).await;
        format!("Processed task: {}", task.id)
    }

    async fn process_generic_task(self, _: context::Context, task: String) -> String {
        // Simulate a long running task
        tokio::time::sleep(Duration::from_secs(5)).await;
        let task = serde_json::from_str::<GenericTask<String>>(&task).unwrap();
        format!("Processed generic task: {}", task.data)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (client_transport, server_transport) = tarpc::transport::channel::unbounded();

    let server = server::BaseChannel::with_defaults(server_transport);
    println!("Spawning server");
    tokio::spawn(
        server
            .execute(TaskProcessorServer.serve())
            .for_each(|response| async move {
                tokio::spawn(response);
            }),
    );

    // TaskProcessorClient is generated by the #[tarpc::service] attribute. It has a constructor `new`
    // that takes a config and any Transport as input.
    println!("Spawning client");
    let client = TaskProcessorClient::new(client::Config::default(), client_transport).spawn();

    // The client has an RPC method for each RPC defined in the annotated trait. It takes the same
    // args as defined, with the addition of a Context, which is always the first arg. The Context
    // specifies a deadline and trace information which can be helpful in debugging requests.
    println!("Sending request");
    let task = Task { id: 1 };
    let response = client.process_task(context::current(), task).await?;
    println!("Received response: {:?}", response);

    let generic_task = GenericTask {
        id: 1,
        name: "Task 1".to_string(),
        description: "Testing tarpc crate".to_string(),
        data: "Convert with serde_json::to_string".to_string(),
    };
    let generic_task_string = serde_json::to_string(&generic_task).unwrap();
    let response = client
        .process_generic_task(context::current(), generic_task_string)
        .await?;
    println!("Received generic task: {:?}", response);
    Ok(())
}
