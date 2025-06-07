use crate::{Error, Result, models::ByteUnit};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub broker_password: String,
	pub broker_port: u16,
	pub broker_uri: String,
	pub broker_username: String,
	pub byte_unit: ByteUnit,
	pub client_id: String,
	pub program_name: String,
	pub report_interval: u64,
}

impl Config {
	pub fn new() -> Result<Config> {
		dotenv::dotenv().ok();

		let client_id = load_env("CLIENT_ID")?;
		let report_interval =
			load_env("REPORT_INTERVAL")?.parse::<u64>().map_err(|_| Error::EnvParseError)?;
		let byte_unit = load_env("BYTE_UNIT")?;
		let broker_uri = load_env("BROKER_URI")?;
		let broker_port =
			load_env("BROKER_PORT")?.parse::<u16>().map_err(|_| Error::EnvParseError)?;
		let broker_username = load_env("BROKER_USERNAME")?;
		let broker_password = load_env("BROKER_PASSWORD")?;

		Ok(Config {
			broker_password,
			broker_port,
			broker_uri,
			broker_username,
			byte_unit: ByteUnit::parse(&byte_unit),
			client_id,
			program_name: "system_monitor".to_string(),
			report_interval,
		})
	}
}

fn load_env(key: &str) -> Result<String> {
	env::var(key).map_err(|_| Error::Env(key.to_string()))
}
