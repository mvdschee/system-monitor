use crate::{
	config::Config,
	error,
	models::{ByteInfo, SystemReport},
	repository::memory::SystemReportStore,
	utils::format_bytes,
};
use std::{
	collections::HashMap,
	thread::{self, sleep},
	time::Duration,
};
use sysinfo::{Components, Disks, Networks, System};

pub struct SystemMonitor {
	system: System,
	config: Config,
	store: SystemReportStore,
}

impl SystemMonitor {
	pub fn new(config: Config, store: SystemReportStore) -> Self {
		let mut system = System::new_all();

		system.refresh_all();

		Self {
			system,
			config,
			store,
		}
	}

	pub fn check_support(&self) -> bool {
		sysinfo::IS_SUPPORTED_SYSTEM
	}

	pub fn ram_usage(&mut self) -> u64 {
		self.system.refresh_memory();
		self.system.used_memory()
	}

	pub fn disk_usage(&mut self) -> DiskInfo {
		let disks = Disks::new_with_refreshed_list();

		let mut unique_disks = std::collections::HashMap::new();
		for disk in &disks {
			unique_disks.insert(disk.total_space(), disk);
		}

		let total_space: u64 = unique_disks.values().map(|disk| disk.total_space()).sum();

		let used_space: u64 =
			unique_disks.values().map(|disk| disk.total_space() - disk.available_space()).sum();

		DiskInfo {
			total: total_space,
			used: used_space,
		}
	}

	pub fn cpu_usage(&mut self) -> {


	}

	pub fn system_info(&self) -> SystemInfo {
		let total_memory = self.system.total_memory();

		SystemInfo {
			total_memory: format_bytes(total_memory, self.config.byte_unit.clone()),
		}
	}

	pub fn run(mut self) -> thread::JoinHandle<()> {
		let info = self.system_info();

		thread::spawn(move || {
			loop {
				let ram_usage = self.ram_usage();
				let disk_info = self.disk_usage();

				let report = SystemReport {
					ram_total: info.total_memory.clone(),
					ram_usage: format_bytes(ram_usage, self.config.byte_unit.clone()),
					disk_total: format_bytes(disk_info.total, self.config.byte_unit.clone()),
					disk_usage: format_bytes(disk_info.used, self.config.byte_unit.clone()),
				};

				self.store.update(report);

				sleep(Duration::from_secs(self.config.report_interval));
			}
		})
	}
}

#[derive(Debug)]
pub struct SystemInfo {
	pub total_memory: ByteInfo,
}

#[derive(Debug)]
pub struct DiskInfo {
	pub total: u64,
	pub used: u64,
}
