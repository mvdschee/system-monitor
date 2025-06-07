# System Monitor MQTT

A lightweight Rust-based system resource monitor that reports metrics to Home Assistant via MQTT.

## Overview

This project monitors system resources (CPU, memory, disk usage, network usage) and publishes the data over MQTT for integration with Home Assistant. Written in Rust for efficiency and reliability, it provides real-time system metrics for your home automation setup.

## Features

- Real-time system resource monitoring
- MQTT integration for Home Assistant
- Low resource footprint
- Systemd service support for automatic startup

## Prerequisites

- MQTT broker accessible from your system
- Home Assistant instance with MQTT integration configured
- Remote access to your Home Assistant (Tailscale recommended for secure connectivity)

### Remote Access Setup

To ensure reliable communication with your Home Assistant instance, you'll need a way to connect remotely. I recommend using [Tailscale](https://tailscale.com) for secure, easy-to-configure remote access.

## Installation

### Quick Deploy

1. Download the latest release from the [GitHub releases page](https://github.com/mvdschee/system-monitor/releases)

2. Extract the release archive:

   ```bash
   tar -xzf system-monitor-mqtt-*.tar.gz
   ```

3. Copy the systemd service file:

   ```bash
   sudo cp system_monitor_mqtt.service /etc/systemd/system/
   ```

4. Configure the service (edit the service file with your MQTT broker details):

   ```bash
   sudo nano /etc/systemd/system/system_monitor_mqtt.service
   ```

5. Enable and start the service:
   ```bash
   sudo systemctl enable system_monitor_mqtt.service
   sudo systemctl start system_monitor_mqtt.service
   ```

## Configuration

The system monitor can be configured through environment variables in a `.env` file. Key settings include:

```bash
CLIENT_ID=your_system_unique_name
STORAGE_UNIT=GB # set by default
MEMORY_UNIT=GB # set by default
NETWORK_UNIT=MB # set by default
MQTT_HOST=127.0.0.1
MQTT_PORT=1883 # set by default
MQTT_USERNAME=home_assistant_username
MQTT_PASSWORD=home_assistant_password
REPORT_INTERVAL=5 # set by default
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
