use {
    clap::ArgMatches,
    solana_sdk::pubkey::Pubkey,
    strum_macros::{Display, EnumString},
};

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub num_clients: usize,
    pub client_target_node: Option<Pubkey>,
    pub client_duration_seconds: u64,
    pub bench_tps_config: Option<BenchTpsConfig>,
    pub spammer_config: Option<SpammerConfig>, // Added new client config
}

#[derive(Clone, Debug)]
pub struct BenchTpsConfig {
    pub client_type: String,
    pub bench_tps_args: Vec<String>,
    pub client_wait_for_n_nodes: Option<usize>,
    pub client_to_run: String,
}

#[derive(Clone, Debug)]
pub struct SpammerConfig {
    pub thread_sleep_ms: Option<u64>,
    pub spam_type: SpamType,
}
