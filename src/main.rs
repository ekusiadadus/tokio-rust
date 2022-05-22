#[allow(dead_code)]
use mini_redis::{client, Result};
use std::env;

fn get_env(name: &str) -> String {
    env::var(name).expect(name)
}

#[tokio::main]
async fn main() -> Result<()> {
    // let redis_server = get_env("REDIS_SERVER");
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
