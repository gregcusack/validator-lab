use crate::client_config::ClientConfig;
use super::ClientTrait;

pub struct GenericClient {
    config: ClientConfig,
}

impl ClientTrait for GenericClient {
    fn new(config: &ClientConfig) -> Self {
        GenericClient {
            config: config.clone(),
        }
    }

    fn generate_client_command_flags(&self) -> Vec<String> {
        let mut flags = vec![];

        if let Some(generic_config) = &self.config.generic_config {
            if !generic_config.args.is_empty() {
                flags.push(generic_config.args.join(" "));
            }
        }
        flags

    }

}