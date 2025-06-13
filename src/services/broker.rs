use crate::config::Config;
use crate::{error, info};
use rumqttc::{Client, Connection, Event, MqttOptions, QoS};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub struct Broker {
	client: Client,
	is_connected: Arc<AtomicBool>,
}

impl Broker {
	pub fn new(config: Config) -> Self {
		info!("broker setup...");

		let mut mqttoptions =
			MqttOptions::new("rumqtt-sync", config.broker_host.clone(), config.broker_port);
		mqttoptions.set_keep_alive(Duration::from_secs(5));
		mqttoptions.set_credentials(config.broker_username.clone(), config.broker_password.clone());

		let (client, connection) = Client::new(mqttoptions.clone(), 100);

		let is_connected = Arc::new(AtomicBool::new(false));
		let is_connected_clone = is_connected.clone();

		thread::spawn(move || {
			Self::handle_connection(connection, is_connected_clone);
		});

		Broker {
			client,
			is_connected,
		}
	}

	fn handle_connection(mut connection: Connection, is_connected: Arc<AtomicBool>) {
		let mut backoff_secs = 1;

		for notification in connection.iter() {
			match notification {
				Ok(Event::Incoming(rumqttc::Packet::ConnAck(_))) => {
					info!("Broker connection established");
					is_connected.store(true, Ordering::SeqCst);
					backoff_secs = 1; // Reset backoff on successful connection
				}
				Ok(Event::Incoming(rumqttc::Packet::Disconnect)) => {
					error!("Broker disconnected");
					is_connected.store(false, Ordering::SeqCst);
				}
				Err(err) => {
					is_connected.store(false, Ordering::SeqCst);

					// Check if it's a connection abort error
					if format!("{:?}", err).contains("ConnectionAborted") {
						error!("Connection aborted, waiting {} seconds before retry", backoff_secs);
						thread::sleep(Duration::from_secs(backoff_secs));
						backoff_secs = (backoff_secs * 2).min(30); // Max 30 seconds
					} else {
						error!("Error in notification: {:?}", err);
					}
				}
				_ => {}
			}

			thread::sleep(Duration::from_millis(100));
		}
	}

	pub fn publish(&self, topic: &str, payload: String) {
		let result = self.client.publish(topic, QoS::AtLeastOnce, false, payload);

		if let Err(err) = result {
			error!("Error publishing message: {:?}", err);
		}
	}

	pub fn wait_until_ready(&self) -> Result<(), String> {
		info!("Waiting for broker connection...");

		let timeout = Duration::from_secs(10);
		let start = std::time::Instant::now();

		while !self.is_connected.load(Ordering::SeqCst) {
			if start.elapsed() > timeout {
				return Err("Broker connection timeout".to_string());
			}

			thread::sleep(Duration::from_millis(100));
		}

		info!("Broker connection ready");
		Ok(())
	}
}
