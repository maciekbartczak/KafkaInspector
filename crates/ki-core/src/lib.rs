use std::time::Duration;

use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig,
};

#[derive(serde::Deserialize)]
pub struct ConnectToClusterParams {
    address: String,
}

pub fn connect(params: &ConnectToClusterParams) -> Result<(), String> {
    println!("connect to cluster started");

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &params.address)
        .create()
        .map_err(|e| format!("failed to create consumer: {}", e))?;

    let metadata = consumer
        .fetch_metadata(None, Duration::from_secs(5))
        .map_err(|e| format!("failed to fetch metadata: {}", e))?;

    println!("Topics:");
    for topic in metadata.topics() {
        println!(
            "Name: {}, partitions: {}",
            topic.name(),
            topic.partitions().len()
        );
    }

    Ok(())
}
