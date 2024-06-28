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

    fn get_docker_image(&self) -> &str {
        "your-dockerhub-username/spammer:latest"
    }

    fn get_command(&self) -> Vec<String> {
        vec![
            "spammer".to_string(),
            "--endpoint".to_string(),
            self.get_endpoint().to_string(),
        ]
    }

    fn get_endpoint(&self) -> &str {
        self.config.client_target_node.as_ref().unwrap().to_string().as_str()
    }
}