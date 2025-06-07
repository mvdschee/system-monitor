use crate::config::Config;
use crate::{error, info};
use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

pub struct Broker {
	client: Client,
}

impl Broker {
	pub fn new(config: Config) -> Self {
		info!("broker setup...");

		let mut mqttoptions =
			MqttOptions::new("rumqtt-sync", config.broker_uri.clone(), config.broker_port);
		mqttoptions.set_keep_alive(Duration::from_secs(5));
		mqttoptions.set_credentials(config.broker_username.clone(), config.broker_password.clone());

		let (client, connection) = Client::new(mqttoptions.clone(), 10);

		thread::spawn(move || {
			Self::handle_connection(connection);
		});

		Broker {
			client,
		}
	}

	fn handle_connection(mut connection: Connection) {
		for notification in connection.iter() {
			if notification.is_err() {
				error!("Error in notification: {:?}", notification);
				continue;
			}

			thread::sleep(Duration::from_millis(100));
		}
	}

	pub fn publish(&self, topic: &str, payload: String) {
		let _ = self.client.publish(topic, QoS::AtMostOnce, false, payload);
	}
}
