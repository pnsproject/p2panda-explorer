
use std::time::Duration;

use aquadoggo::{Configuration, Node};
use p2panda_rs::identity::KeyPair;
use p2panda_sdk::p2panda;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key_pair = KeyPair::new();

    let client = p2panda::Client::new(
        KeyPair::from_private_key(key_pair.private_key())?,
        "http://localhost:2020/graphql",
    );

    let node = tokio::spawn(async {
        let config = Configuration::default();
        let node = Node::start(key_pair, config).await;

        node.on_exit().await;
        node.shutdown().await;
    });

    let definition_name = json!([
        1,
        0,
        "schema_field_definition_v1",
        {
            "name": "name",
            "type": "str"
        }
    ]);

    let operation = serde_json::from_value(definition_name)?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    let resp = client.publish(operation).await?;

    println!("{resp:?}");

    node.await?;

    Ok(())
}
