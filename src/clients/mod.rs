use {
    crate::client_config::ClientConfig,
    strum_macros::Display,
};

pub mod bench_tps;
pub mod spam;

pub trait Client {
    fn new(config: &ClientConfig) -> Self where Self: Sized;
    fn generate_client_command_flags(&self) -> Vec<String>; // Add this method
}

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum ClientType {
    #[strum(serialize = "bench-tps")]
    BenchTps,
    #[strum(serialize = "spammer")]
    Spammer,
}

pub struct ClientFactory;

impl ClientFactory {
    pub fn create_client(client_type: ClientType, config: &ClientConfig) -> Box<dyn Client> {
        match client_type {
            ClientType::BenchTps => Box::new(bench_tps::BenchTpsClient::new(config)),
            ClientType::Spammer => Box::new(spam::SpammerClient::new(config)),
        }
    }
}
