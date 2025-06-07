use std::fmt;

#[derive(Debug, Clone)]
pub struct SystemReport {
	pub ram_total: ByteInfo,
	pub ram_usage: ByteInfo,
	pub disk_total: ByteInfo,
	pub disk_usage: ByteInfo,
}

impl fmt::Display for SystemReport {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			r#"{{
"ram_total": {:.prec_ram_total$},
"ram_usage": {:.prec_ram_usage$},
"disk_total": {:.prec_disk_total$},
"disk_usage": {:.prec_disk_usage$}
}}"#,
			self.ram_total.value,
			self.ram_usage.value,
			self.disk_total.value,
			self.disk_usage.value,
			prec_ram_total = self.ram_total.precision,
			prec_ram_usage = self.ram_usage.precision,
			prec_disk_total = self.disk_total.precision,
			prec_disk_usage = self.disk_usage.precision,
		)
	}
}

// let config = serde_json::json!({
// 	"device_class": "data_size",
// 	"state_topic": topic,
// 	"unit_of_measurement": self.config.byte_unit.to_string(),
// 	"value_template": value_template,
// 	"unique_id": sensor,
// 	"name": sensor.replace("_", " "),
// 	"state_class": "total",
// 	"device": {
// 		"name": device_name,
// 		"identifiers": [model_id],
// 		"manufacturer": self.config.program_name,
// 		"model": model_id,
// 	}
// });

#[derive(Debug, Clone)]
pub struct EntityConfig {
	pub device_class: DeviceClass,
}

#[derive(Debug, Clone)]
pub enum DeviceClass {
	DataSize,
}

#[derive(Debug, Clone)]
pub struct ByteInfo {
	pub value: f64,
	pub unit: String,
	pub precision: usize,
}

#[derive(Debug, Clone)]
pub enum ByteUnit {
	Byte,
	Kilobyte,
	Megabyte,
	Gigabyte,
	Terabyte,
	Petabyte,
}

impl ByteUnit {
	pub fn parse(unit: &str) -> ByteUnit {
		match unit {
			"KB" => ByteUnit::Kilobyte,
			"MB" => ByteUnit::Megabyte,
			"GB" => ByteUnit::Gigabyte,
			"TB" => ByteUnit::Terabyte,
			"PB" => ByteUnit::Petabyte,
			_ => ByteUnit::Byte,
		}
	}

	pub fn to_bytes(&self) -> f64 {
		match self {
			ByteUnit::Byte => 1.0,
			ByteUnit::Kilobyte => 1024.0,
			ByteUnit::Megabyte => 1024.0 * 1024.0,
			ByteUnit::Gigabyte => 1024.0 * 1024.0 * 1024.0,
			ByteUnit::Terabyte => 1024.0 * 1024.0 * 1024.0 * 1024.0,
			ByteUnit::Petabyte => 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
		}
	}
}

impl std::fmt::Display for ByteUnit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let unit_str = match self {
			ByteUnit::Byte => "B",
			ByteUnit::Kilobyte => "KB",
			ByteUnit::Megabyte => "MB",
			ByteUnit::Gigabyte => "GB",
			ByteUnit::Terabyte => "TB",
			ByteUnit::Petabyte => "PB",
		};
		write!(f, "{}", unit_str)
	}
}

#[derive(Debug, Clone)]
pub struct DeviceConfig {
	pub unique_id: String,
	pub name: String,
	pub state_topic: String,
	pub state_class: String,
	pub device_class: String,
	pub unit_of_measurement: String,
	pub device: Device,
}

#[derive(Debug, Clone)]
pub struct Device {
	pub identifiers: Vec<String>,
	pub manufacturer: String,
	pub model: String,
	pub name: String,
}
