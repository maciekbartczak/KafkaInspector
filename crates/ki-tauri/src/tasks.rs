use crate::AppMessage;
use ki_core::{ConsumerParams, Metadata, MetadataFetcher};
use log::debug;
use tauri::ipc::Channel;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn metadata_fetcher_task(
    params: ConsumerParams,
    metadata_updated_channel: Channel<Metadata>,
    app_message_sender: Sender<AppMessage>,
    result_sender: oneshot::Sender<Result<(), String>>,
) {
    let metadata_fetcher = MetadataFetcher::new(&params).unwrap();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

    // test the connection first
    match metadata_fetcher.fetch_metadata() {
        Ok(_) => {
            result_sender.send(Ok(())).unwrap();
        }
        Err(e) => {
            app_message_sender
                .send(AppMessage::RemoveMetadataFetcher)
                .await
                .unwrap();
            result_sender.send(Err(e.to_string())).unwrap();
            return;
        }
    };

    loop {
        interval.tick().await;

        let (result_sender, result_receiver) = oneshot::channel();
        app_message_sender
            .send(AppMessage::GetLastMetadata(result_sender))
            .await
            .unwrap();
        let last_metadata = result_receiver.await.unwrap();

        let metadata = metadata_fetcher.fetch_metadata().unwrap();
        if metadata != last_metadata {
            debug!("Updating metadata - it differs from the previous update");
            app_message_sender
                .send(AppMessage::SetLastMetadata(metadata.clone()))
                .await
                .unwrap();
            metadata_updated_channel.send(metadata).unwrap()
        } else {
            debug!("Metadata update skipped - it does not differ from the previous update")
        }
    }
}
