use crate::{
	config::Config,
	core::SystemReporter,
	services::{broker::Broker, system_monitor::SystemMonitor},
};
pub use core::models;
pub use error::{Error, Result};
use repository::memory::SystemReportStore;
use std::{thread, time::Duration};

mod config;
mod core;
mod error;
mod repository;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
	tagged_status!("[MAIN]", "booting...");

	let config = Config::new()?;
	let store = SystemReportStore::new();

	let producer_store = store.clone();
	let mut monitor = SystemMonitor::new(config.clone(), producer_store);

	if !monitor.check_support() {
		return Err(Error::UnsupportedOS);
	}

	let broker = Broker::new(config.clone());
	let reporter = SystemReporter::new(config, store, broker);

	// start monitoring system resources
	monitor.run();

	// run the reporter
	reporter.register();
	reporter.run();

	Err(Error::MainLoopClosed)
}
