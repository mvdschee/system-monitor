use crate::{
	config::Config,
	models::{Device, DeviceConfig},
	repository::memory::SystemReportStore,
	services::broker::Broker,
	status,
};
use std::{thread, time::Duration};

pub mod models;

const SUPPORTED_SENSORS: &[&str] = &["ram_total", "ram_usage", "disk_total", "disk_usage"];

pub struct SystemReporter {
	config: Config,
	store: SystemReportStore,
	broker: Broker,
	sensors: Vec<String>,
}

impl SystemReporter {
	pub fn new(config: Config, store: SystemReportStore, broker: Broker) -> Self {
		Self {
			config,
			store,
			broker,
			sensors: SUPPORTED_SENSORS.iter().map(|s| s.to_string()).collect(),
		}
	}

	pub fn run(&self) {
		// Implementation to report system information
		let topic = format!("{}/{}/state", self.config.program_name, self.config.client_id);

		loop {
			if let Some(entry) = self.store.get_latest() {
				self.broker.publish(&topic, entry.report.to_string());
			}

			thread::sleep(Duration::from_secs(self.config.report_interval));
		}
	}

	pub fn register(&self) {
		// Implementation to register system information
		let topic = format!("{}/{}/state", self.config.program_name, self.config.client_id);
		let device_name = format!(
			"{} {}",
			self.config.program_name.replace("_", " "),
			self.config.client_id.replace("_", " ")
		);
		let model_id = format!("{}_{}", self.config.program_name, self.config.client_id);

		for sensor in &self.sensors {
			let config_topic =
				format!("homeassistant/sensor/{}_{}/config", self.config.client_id, sensor);

			let value_template = format!("{{{{ value_json.{} }}}}", sensor);
			let config = serde_json::json!({
				"device_class": "data_size",
				"state_topic": topic,
				"unit_of_measurement": self.config.byte_unit.to_string(),
				"value_template": value_template,
				"unique_id": sensor,
				"name": sensor.replace("_", " "),
				"state_class": "total",
				"device": {
					"name": device_name,
					"identifiers": [model_id],
					"manufacturer": self.config.program_name,
					"model": model_id,
				}
			});

			self.broker.publish(&config_topic, config.to_string());
		}
	}
}
