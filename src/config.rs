use serde::Deserialize;
use serde_json::from_str;
use tokio::io::AsyncReadExt;
use tokio::sync::OnceCell;
use tokio::{fs::OpenOptions, io};

use crate::constants::{DEFAULT_CONFIG_PATH, DEFAULT_STORAGE_PATH};

fn default_storage_dir() -> String {
  DEFAULT_STORAGE_PATH.to_string()
}

#[derive(Deserialize, Default, Debug)]
pub struct Config {
  #[serde(default = "default_storage_dir")]
  pub storage_dir: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
  #[error(transparent)]
  IO(#[from] io::Error),

  #[error(transparent)]
  Json(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, ConfigError>;

static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn get_config_from_path(config_path: &str) -> Result<Config> {
  let expanded_path = shellexpand::tilde(config_path);
  let mut file = OpenOptions::new()
    .read(true)
    .open(expanded_path.as_ref())
    .await?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).await?;

  let config: Config = from_str(&contents)?;

  Ok(config)
}

pub async fn get_config(matches: &clap::ArgMatches) -> &'static Config {
  CONFIG
    .get_or_init(|| async {
      let config_path = match matches.get_one::<String>("config") {
        Some(path) => path,
        None => &DEFAULT_CONFIG_PATH.to_string(),
      };

      match get_config_from_path(config_path).await {
        Ok(config) => config,
        Err(_) => {
          eprintln!(
            "Info: No config found at {}, using default config",
            config_path
          );
          Config::default()
        }
      }
    })
    .await
}
