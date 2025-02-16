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
    messages_count: usize,
    last_message_timestamp: Option<i64>,
    size: usize,
}

impl Metadata {
    pub fn topics(&self) -> &Vec<Topic> {
        &self.topics
    }
}

impl Topic {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn partitions(&self) -> &usize {
        &self.partitions_count
    }

    pub fn messages_count(&self) -> &usize {
        &self.messages_count
    }

    pub fn last_message_timestamp(&self) -> &Option<i64> {
        &self.last_message_timestamp
    }

    pub fn size(&self) -> &usize {
        &self.size
    }
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
                    messages_count: 0,
                    last_message_timestamp: None,
                    size: 0,
                })
                .collect(),
        })
    }
}
