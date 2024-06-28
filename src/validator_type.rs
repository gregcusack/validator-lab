use {
    crate::{
        client_config::Client,
        startup_scripts::StartupScripts,
    },
    strum_macros::Display,
};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ValidatorType {
    #[strum(serialize = "bootstrap-validator")]
    Bootstrap,
    #[strum(serialize = "validator")]
    Standard,
    #[strum(serialize = "rpc-node")]
    RPC,
    #[strum(serialize = "client")]
    ClientWrapper(Client, /* client index */ usize),
}

impl ValidatorType {
    pub fn script(&self) -> &'static str {
        match self {
            ValidatorType::Bootstrap => StartupScripts::bootstrap(),
            ValidatorType::Standard => StartupScripts::validator(),
            ValidatorType::RPC => StartupScripts::rpc(),
            ValidatorType::ClientWrapper(_,_) => StartupScripts::bench_tps_client(),
        }
    }
}