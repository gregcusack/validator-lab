use crate::client_config::ClientConfig;
use super::ClientTrait;

pub struct BenchTpsClient {
    config: ClientConfig,
}

impl ClientTrait for BenchTpsClient {
    fn new(config: &ClientConfig) -> Self {
        BenchTpsClient {
            config: config.clone(),
        }
    }

    fn generate_client_command_flags(&self) -> Vec<String> {
        let mut flags = vec![];

        if let Some(bench_tps_config) = &self.config.bench_tps_config {
            flags.push(bench_tps_config.client_to_run.clone()); //client to run
            if !bench_tps_config.bench_tps_args.is_empty() {
                flags.push(bench_tps_config.bench_tps_args.join(" "));
            }

            flags.push(bench_tps_config.client_type.clone());

            if let Some(target_node) = bench_tps_config.client_target_node {
                flags.push("--target-node".to_string());
                flags.push(target_node.to_string().clone());
            }

            flags.push("--duration".to_string());
            flags.push(self.config.client_duration_seconds.to_string());

            if let Some(num_nodes) = bench_tps_config.client_wait_for_n_nodes {
                flags.push("--num-nodes".to_string());
                flags.push(num_nodes.to_string());
            }
        }

        flags
    }
}