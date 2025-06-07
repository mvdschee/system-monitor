use crate::{Error, Result, models::ByteUnit};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
	pub broker_password: String,
	pub broker_port: u16,
	pub broker_host: String,
	pub broker_username: String,
	pub storage_unit: ByteUnit,
	pub network_unit: ByteUnit,
	pub memory_unit: ByteUnit,
	pub client_id: String,
	pub program_name: String,
	pub report_interval: u64,
}

impl Config {
	pub fn new() -> Result<Config> {
		dotenv::dotenv().ok();

		let client_id = load_env("CLIENT_ID")?;
		let broker_username = load_env("MQTT_USERNAME")?;
		let broker_password = load_env("MQTT_PASSWORD")?;

		let report_interval = load_env_with_default("REPORT_INTERVAL", "5")
			.parse::<u64>()
			.map_err(|_| Error::EnvParseError)?;
		let storage_unit = load_env_with_default("STORAGE_UNIT", "GB");
		let network_unit = load_env_with_default("NETWORK_UNIT", "MB");
		let memory_unit = load_env_with_default("MEMORY_UNIT", "GB");
		let broker_host = load_env_with_default("MQTT_HOST", "localhost");
		let broker_port = load_env_with_default("MQTT_PORT", "1883")
			.parse::<u16>()
			.map_err(|_| Error::EnvParseError)?;

		Ok(Config {
			broker_password,
			broker_port,
			broker_host,
			broker_username,
			storage_unit: ByteUnit::parse(&storage_unit),
			network_unit: ByteUnit::parse(&network_unit),
			memory_unit: ByteUnit::parse(&memory_unit),
			client_id,
			program_name: "system_monitor".to_string(),
			report_interval,
		})
	}
}

fn load_env(key: &str) -> Result<String> {
	env::var(key).map_err(|_| Error::Env(key.to_string()))
}

fn load_env_with_default(key: &str, default: &str) -> String {
	env::var(key).unwrap_or_else(|_| default.to_string())
}
