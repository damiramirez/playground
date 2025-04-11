use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open connection to redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    print!("got value from the server; result = {:?}", result);

    Ok(())
}
