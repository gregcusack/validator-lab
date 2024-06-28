use crate::client_config::ClientConfig;
use super::Client;

pub struct SpammerClient {
    config: ClientConfig,
}

impl Client for SpammerClient {
    fn new(config: &ClientConfig) -> Self {
        SpammerClient {
            config: config.clone(),
        }
    }

    fn generate_client_command_flags(&self) -> Vec<String> {
        let mut flags = vec![];

        flags

    }

}