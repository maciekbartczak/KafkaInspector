use std::time::Duration;

use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct ConsumerParams {
    pub address: String,
}

pub struct MetadataFetcher {
    consumer: BaseConsumer,
}

#[derive(Serialize, Clone, PartialEq, Default)]
pub struct Metadata {
    topics: Vec<Topic>,
}

#[derive(Serialize, Clone, PartialEq)]
pub struct Topic {
    name: String,
    partitions_count: usize,
}

impl MetadataFetcher {
    pub fn new(params: &ConsumerParams) -> Result<Self, String> {
        let consumer = ClientConfig::new()
            .set("bootstrap.servers", &params.address)
            .create()
            .map_err(|e| format!("failed to create consumer: {}", e))?;

        Ok(Self { consumer })
    }

    pub fn fetch_metadata(&self) -> Result<Metadata, String> {
        let rdkafka_metadata = self
            .consumer
            .fetch_metadata(None, Duration::from_secs(5))
            .map_err(|e| format!("failed to fetch metadata: {}", e))?;

        Ok(Metadata {
            topics: rdkafka_metadata
                .topics()
                .into_iter()
                .map(|t| Topic {
                    name: t.name().to_string(),
                    partitions_count: t.partitions().len(),
                })
                .collect(),
        })
    }
}
