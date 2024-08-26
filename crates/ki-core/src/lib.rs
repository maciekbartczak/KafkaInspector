use std::time::Duration;

use log::info;
use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig,
};

#[derive(serde::Deserialize)]
pub struct ConnectToClusterParams {
    pub address: String,
}

pub fn connect(params: &ConnectToClusterParams) -> Result<(), String> {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &params.address)
        .create()
        .map_err(|e| format!("failed to create consumer: {}", e))?;

    let metadata = consumer
        .fetch_metadata(None, Duration::from_secs(5))
        .map_err(|e| format!("failed to fetch metadata: {}", e))?;

    info!("Topics:");
    for topic in metadata.topics() {
        info!(
            "Name: {}, partitions: {}",
            topic.name(),
            topic.partitions().len()
        );
    }

    Ok(())
}
