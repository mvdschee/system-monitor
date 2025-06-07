use crate::core::models::SystemReport;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SystemReportEntry {
	pub report: SystemReport,
	pub timestamp: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SystemReportStore {
	storage: Arc<DashMap<String, SystemReportEntry>>,
}

impl SystemReportStore {
	pub fn new() -> Self {
		Self {
			storage: Arc::new(DashMap::new()),
		}
	}

	/// Update the system report with a new value
	pub fn update(&self, report: SystemReport) {
		let entry = SystemReportEntry {
			report,
			timestamp: Utc::now(),
		};
		self.storage.insert("latest".to_string(), entry);
	}

	/// Get the latest system report
	pub fn get_latest(&self) -> Option<SystemReportEntry> {
		self.storage.get("latest").map(|entry| entry.clone())
	}

	/// Check if there's any report stored
	pub fn has_report(&self) -> bool {
		self.storage.contains_key("latest")
	}

	/// Clear all stored reports
	pub fn clear(&self) {
		self.storage.clear();
	}
}

impl Default for SystemReportStore {
	fn default() -> Self {
		Self::new()
	}
}
