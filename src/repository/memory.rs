use crate::core::models::SystemReport;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SystemReportEntry {
	pub report: SystemReport,
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
		};
		self.storage.insert("latest".to_string(), entry);
	}

	/// Get the latest system report
	pub fn get_latest(&self) -> Option<SystemReportEntry> {
		self.storage.get("latest").map(|entry| entry.clone())
	}
}

impl Default for SystemReportStore {
	fn default() -> Self {
		Self::new()
	}
}
