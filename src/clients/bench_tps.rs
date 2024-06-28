use crate::client_config::ClientConfig;
use super::Client;

pub struct BenchTpsClient {
    config: ClientConfig,
}

impl Client for BenchTpsClient {
    fn new(config: &ClientConfig) -> Self {
        BenchTpsClient {
            config: config.clone(),
        }
    }

    fn get_docker_image(&self) -> &str {
        "your-dockerhub-username/bench-tps:latest"
    }

    fn get_command(&self) -> Vec<String> {
        let mut command = vec![
            "bench-tps".to_string(),
            "--endpoint".to_string(),
            self.get_endpoint().to_string(),
        ];
        command.extend(self.config.bench_tps_config.as_ref().unwrap().bench_tps_args.clone());
        command
    }

    fn get_endpoint(&self) -> &str {
        self.config.client_target_node.as_ref().unwrap().to_string().as_str()
    }

    fn generate_client_command_flags(&self) -> Vec<String> {
        let mut flags = vec![];

        if let Some(bench_tps_config) = &self.config.bench_tps_config {
            flags.push(bench_tps_config.client_to_run.clone()); //client to run
            if !bench_tps_config.bench_tps_args.is_empty() {
                flags.push(bench_tps_config.bench_tps_args.join(" "));
            }

            flags.push(bench_tps_config.client_type.clone());

            if let Some(target_node) = &self.config.client_target_node {
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