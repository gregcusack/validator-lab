use {
    crate::{new_spinner_progress_bar, release::DeployMethod, ValidatorType, BUILD},
    log::*,
    std::{
        env,
        error::Error,
        fs,
        path::{Path, PathBuf},
        process::{Command, Output, Stdio},
    },
};

pub struct DockerConfig {
    pub base_image: String,
    pub image_name: String,
    pub tag: String,
    pub registry: String,
    deploy_method: DeployMethod,
}

impl DockerConfig {
    pub fn new(
        base_image: String,
        image_name: String,
        tag: String,
        registry: String,
        deploy_method: DeployMethod,
    ) -> Self {
        DockerConfig {
            base_image,
            image_name,
            tag,
            registry,
            deploy_method,
        }
    }

    pub fn build_image(
        &self,
        solana_root_path: &Path,
        validator_type: &ValidatorType,
    ) -> Result<(), Box<dyn Error>> {
        match validator_type {
            ValidatorType::Bootstrap => (),
            ValidatorType::Standard | ValidatorType::RPC | ValidatorType::Client => {
                return Err(format!(
                    "Build docker image for validator type: {validator_type} not supported yet"
                )
                .into());
            }
        }
        let image_name = format!("{validator_type}-{}", self.image_name);
        let docker_path = solana_root_path.join(format!("docker-build/{validator_type}"));
        match self.create_base_image(solana_root_path, image_name, &docker_path, validator_type) {
            Ok(res) => {
                if res.status.success() {
                    info!("Successfully created base Image");
                    Ok(())
                } else {
                    error!("Failed to build base image");
                    Err(String::from_utf8_lossy(&res.stderr).into())
                }
            }
            Err(err) => Err(err),
        }
    }

    fn create_base_image(
        &self,
        solana_root_path: &Path,
        image_name: String,
        docker_path: &PathBuf,
        validator_type: &ValidatorType,
    ) -> Result<Output, Box<dyn Error>> {
        self.create_dockerfile(validator_type, docker_path, None)?;

        trace!("Tmp: {}", docker_path.as_path().display());
        trace!("Exists: {}", docker_path.as_path().exists());

        // We use std::process::Command here because Docker-rs is very slow building dockerfiles
        // when they are in large repos. Docker-rs doesn't seem to support the `--file` flag natively.
        // so we result to using std::process::Command
        let dockerfile = docker_path.join("Dockerfile");
        let context_path = solana_root_path.display().to_string();

        let progress_bar = new_spinner_progress_bar();
        progress_bar.set_message(format!("{BUILD}Building {validator_type} docker image...",));

        let command = format!(
            "docker build -t {}/{image_name}:{} -f {dockerfile:?} {context_path}",
            self.registry, self.tag,
        );

        let output = match Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to execute command")
            .wait_with_output()
        {
            Ok(res) => Ok(res),
            Err(err) => Err(Box::new(err) as Box<dyn Error>),
        };
        progress_bar.finish_and_clear();
        info!("{validator_type} image build complete");

        output
    }

    fn copy_file_to_docker(
        source_dir: &Path,
        docker_dir: &Path,
        file_name: &str,
    ) -> std::io::Result<()> {
        let source_path = source_dir.join("src/scripts").join(file_name);
        let destination_path = docker_dir.join(file_name);
        fs::copy(source_path, destination_path)?;
        Ok(())
    }

    fn create_dockerfile(
        &self,
        validator_type: &ValidatorType,
        docker_path: &PathBuf,
        content: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        if docker_path.exists() {
            fs::remove_dir_all(docker_path)?;
        }
        fs::create_dir_all(docker_path)?;

        if let DeployMethod::Local(_) = self.deploy_method {
            if validator_type == &ValidatorType::Bootstrap {
                let manifest_path =
                    PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR"));
                let files_to_copy = ["bootstrap-startup-script.sh", "common.sh"];
                for file_name in files_to_copy.iter() {
                    Self::copy_file_to_docker(&manifest_path, docker_path, file_name)?;
                }
            }
        }

        let (solana_build_directory, startup_script_directory) =
            if let DeployMethod::ReleaseChannel(_) = self.deploy_method {
                ("solana-release", "./src/scripts".to_string())
            } else {
                ("farf", format!("./docker-build/{validator_type}"))
            };

        let dockerfile = format!(
            r#"
FROM {}
RUN apt-get update
RUN apt-get install -y iputils-ping curl vim bzip2

RUN useradd -ms /bin/bash solana
RUN adduser solana sudo
USER solana

RUN mkdir -p /home/solana/k8s-cluster-scripts
# TODO: this needs to be changed for non bootstrap, this should be ./src/scripts/<validator-type>-startup-scripts.sh
COPY {startup_script_directory}/bootstrap-startup-script.sh /home/solana/k8s-cluster-scripts
 
RUN mkdir -p /home/solana/ledger
COPY --chown=solana:solana ./config-k8s/bootstrap-validator  /home/solana/ledger

RUN mkdir -p /home/solana/.cargo/bin

COPY ./{solana_build_directory}/bin/ /home/solana/.cargo/bin/
COPY ./{solana_build_directory}/version.yml /home/solana/

RUN mkdir -p /home/solana/config
ENV PATH="/home/solana/.cargo/bin:${{PATH}}"

WORKDIR /home/solana

"#,
            self.base_image
        );

        debug!("dockerfile: {dockerfile:?}");
        std::fs::write(
            docker_path.join("Dockerfile"),
            content.unwrap_or(dockerfile.as_str()),
        )?;
        Ok(())
    }
}
