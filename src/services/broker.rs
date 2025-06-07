use crate::repository::memory::SystemReportStore;
use crate::{Result, config::Config};
use crate::{info, status};
use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

pub struct Broker {
	client: Client,
}

impl Broker {
	pub fn new(config: Config) -> Self {
		let mut mqttoptions =
			MqttOptions::new("rumqtt-sync", config.broker_uri, config.broker_port);
		mqttoptions.set_keep_alive(Duration::from_secs(5));
		mqttoptions.set_credentials(config.broker_username, config.broker_password);

		let (client, mut connection) = Client::new(mqttoptions.clone(), 10);

		// Start the connection handler in a background thread
		thread::spawn(move || {
			Self::handle_connection(connection);
		});

		Broker {
			client,
		}
	}

	fn handle_connection(mut connection: Connection) {
		for (i, notification) in connection.iter().enumerate() {
			println!("Notification = {:?}", notification);
		}
	}

	pub fn publish(&self, topic: &str, payload: String) {
		info!("Topic = {}", topic);
		println!("Payload = {}", payload);
		// self.client.publish(topic, QoS::AtMostOnce, false, payload);
	}
}
