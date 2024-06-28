use {
    crate::{
        clients::ClientType,
        startup_scripts::StartupScripts,
    },
    strum_macros::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum ValidatorType {
    #[strum(serialize = "bootstrap-validator")]
    Bootstrap,
    #[strum(serialize = "validator")]
    Standard,
    #[strum(serialize = "rpc-node")]
    RPC,
    #[strum(serialize = "client")]
    Client(ClientType, /* client index */ usize),
}

impl ValidatorType {
    pub fn script(&self) -> &'static str {
        match self {
            ValidatorType::Bootstrap => StartupScripts::bootstrap(),
            ValidatorType::Standard => StartupScripts::validator(),
            ValidatorType::RPC => StartupScripts::rpc(),
            ValidatorType::Client(_,_) => StartupScripts::bench_tps_client(),
        }
    }
}