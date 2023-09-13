use subxt::{OnlineClient, SubstrateConfig};

pub async fn connect() -> OnlineClient<SubstrateConfig> {
    let uri = "ws://127.0.0.1:9944".to_string();
    let api = OnlineClient::<SubstrateConfig>::from_url(uri)
        .await
        .expect("Connection to Rococo failed");
    api
}