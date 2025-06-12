use crate::config::Config;
use crate::{error, info, warn};
use rumqttc::{Client, Connection, Event, MqttOptions, QoS};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Broker {
	client: Client,
	connected: Arc<AtomicBool>,
}

impl Broker {
	pub fn new(config: Config) -> Self {
		info!("broker setup...");

		// Generate unique client ID to avoid conflicts
		let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
		let client_id = format!("system-monitor-{}", timestamp);

		info!(
			"Connecting to MQTT broker at {}:{} with client ID: {}",
			config.broker_host, config.broker_port, client_id
		);

		let mut mqttoptions =
			MqttOptions::new(client_id, config.broker_host.clone(), config.broker_port);
		// Increase keep-alive to reduce connection drops
		mqttoptions.set_keep_alive(Duration::from_secs(30));
		mqttoptions.set_credentials(config.broker_username.clone(), config.broker_password.clone());
		// Clean session to avoid stale state
		mqttoptions.set_clean_session(true);

		let (client, connection) = Client::new(mqttoptions.clone(), 100);
		let connected = Arc::new(AtomicBool::new(false));
		let connected_clone = connected.clone();

		thread::spawn(move || {
			Self::handle_connection(connection, config, connected_clone);
		});

		Broker {
			client,
			connected,
		}
	}

	fn handle_connection(mut connection: Connection, config: Config, connected: Arc<AtomicBool>) {
		loop {
			match connection.iter().next() {
				Some(Ok(Event::Incoming(rumqttc::Packet::ConnAck(connack)))) => {
					info!("Connected to MQTT broker successfully: {:?}", connack);
					connected.store(true, Ordering::Relaxed);
				}
				Some(Ok(event)) => {
					// Log other events at debug level
					info!("MQTT event: {:?}", event);
				}
				Some(Err(e)) => {
					connected.store(false, Ordering::Relaxed);
					match &e {
						rumqttc::ConnectionError::MqttState(rumqttc::StateError::Io(io_err)) => {
							if io_err.kind() == std::io::ErrorKind::ConnectionAborted {
								error!(
									"MQTT connection closed by broker - possible authentication failure or broker issue"
								);
							} else {
								error!("MQTT IO error: {}", io_err);
							}
						}
						_ => {
							error!("MQTT connection error: {:?}", e);
						}
					}

					// Wait before attempting reconnection
					warn!("Waiting 5 seconds before reconnection attempt...");
					thread::sleep(Duration::from_secs(5));

					// Recreate connection with new client ID
					let timestamp =
						SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
					let client_id = format!("system-monitor-{}", timestamp);

					info!("Attempting to reconnect with new client ID: {}", client_id);

					let mut mqttoptions =
						MqttOptions::new(client_id, config.broker_host.clone(), config.broker_port);
					mqttoptions.set_keep_alive(Duration::from_secs(30));
					mqttoptions.set_credentials(
						config.broker_username.clone(),
						config.broker_password.clone(),
					);
					mqttoptions.set_clean_session(true);

					// Create new connection
					let (_, new_connection) = Client::new(mqttoptions, 100);
					{
						connection = new_connection;
						info!("Reconnection attempt initiated");
					}
				}
				None => {
					warn!("MQTT connection iterator ended unexpectedly");
					thread::sleep(Duration::from_secs(1));
				}
			}
		}
	}

	pub fn publish(&self, topic: &str, payload: String) {
		if !self.connected.load(Ordering::Relaxed) {
			warn!("Attempting to publish while disconnected, message may be queued");
		}

		let result = self.client.publish(topic, QoS::AtLeastOnce, false, payload);

		if let Err(err) = result {
			error!("Error publishing to topic '{}': {:?}", topic, err);
		} else {
			info!("Successfully published to topic '{}'", topic);
		}
	}
}
