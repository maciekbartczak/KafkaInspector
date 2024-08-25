#[derive(serde::Deserialize)]
pub struct ConnectToClusterParams {
    address: String,
}

pub fn connect(params: &ConnectToClusterParams) -> Result<(), String> {
    println!("connect to cluster started");

    std::thread::sleep(std::time::Duration::from_secs(5));

    Err("not implemented".to_string())
}
