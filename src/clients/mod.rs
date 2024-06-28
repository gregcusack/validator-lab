use crate::client_config::ClientConfig;

pub mod bench_tps;
pub mod generic;

pub trait ClientTrait {
    fn new(config: &ClientConfig) -> Self where Self: Sized;
    fn generate_client_command_flags(&self) -> Vec<String>; // Add this method
}
