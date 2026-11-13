use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) host: String,
    pub(crate) nats_servers: Vec<String>,
}
