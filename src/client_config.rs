use {
    solana_sdk::pubkey::Pubkey,
    strum_macros::Display,
};

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub num_clients: usize,
    pub client_duration_seconds: u64,
    pub client: Client,
}

#[derive(Clone, PartialEq, Debug)]
pub struct BenchTpsConfig {
    pub client_type: String,
    pub bench_tps_args: Vec<String>,
    pub client_wait_for_n_nodes: Option<usize>,
    pub client_to_run: String,
    pub client_target_node: Option<Pubkey>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct GenericClientConfig {
    pub args: Vec<String>,
    pub image: String,
    pub executable_name: String,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Client {
    #[strum(serialize = "bench-tps")]
    BenchTps(BenchTpsConfig),
    #[strum(serialize = "generic")]
    Generic(GenericClientConfig),
}