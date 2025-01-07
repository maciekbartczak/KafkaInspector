use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig,
};
use std::time::Duration;

pub mod rt;

#[derive(Clone)]
pub struct ConsumerParams {
    pub address: String,
}

pub struct MetadataFetcher {
    consumer: BaseConsumer,
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Metadata {
    topics: Vec<Topic>,
}

#[derive(Clone, PartialEq, Debug)]
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
