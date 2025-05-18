# Teltonika Server

A server for Teltonika GPS trackers that receives data from devices and forwards it to a configurable HTTP API endpoint.

## Description

Teltonika Server is a lightweight, high-performance TCP server written in Rust that handles connections from Teltonika GPS tracking devices. It processes the binary data sent by these devices, parses it according to the Teltonika protocols (primarily Codec8), and forwards the structured data to a configurable HTTP API endpoint.

## Features

- TCP server listening on port 5027 (standard Teltonika port)
- IMEI-based device authentication
- Support for Teltonika Codec8 protocol
- Real-time data forwarding to configurable HTTP API endpoints
- Proper acknowledgment handling for device messages
- Asynchronous processing using Tokio runtime

## Installation

### Prerequisites

- Rust 1.56 or later
- Cargo package manager

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/teltonika-server.git
cd teltonika-server

# Build the project
cargo build --release

# The binary will be available at target/release/teltonika-server
```


## Usage

```bash
# Run with default settings (looks for /etc/teltonika-server/config.json)
./teltonika-server

# Specify a custom configuration file path
./teltonika-server --config /path/to/your/config.json
# or using the short option
./teltonika-server -c /path/to/your/config.json

# Set configuration path using environment variable
TELTONIKA_CONFIG_PATH=/path/to/your/config.json ./teltonika-server

# Set custom log level
RUST_LOG=debug ./teltonika-server
```

### Logging

When running the server manually, logs are output to the console (stderr). You can control the log level using the `RUST_LOG` environment variable.

When installed as a Debian package and running as a systemd service, logs are written to:
- Standard output: `/var/log/teltonika-server/teltonika-server.log`
- Standard error: `/var/log/teltonika-server/teltonika-server.error.log`

You can view the logs using:
```bash
# View standard output logs
sudo cat /var/log/teltonika-server/teltonika-server.log
# or follow the logs in real-time
sudo tail -f /var/log/teltonika-server/teltonika-server.log

# View error logs
sudo cat /var/log/teltonika-server/teltonika-server.error.log
```

The server determines the configuration file path in the following order of priority:
1. Command-line argument (`--config` or `-c`)
2. Environment variable (`TELTONIKA_CONFIG_PATH`)
3. Default path (`/etc/teltonika-server/config.json`)

## Configuration

The server can be configured using a JSON configuration file. You can specify the path to this file using command-line arguments or environment variables as shown in the Usage section.

### Configuration Options

```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 5027
  },
  "api_integration": {
    "http_endpoint_url": "https://your-api-endpoint-url",
    "auth_header_name": "your-auth-header-name",
    "auth_header_value": "your-auth-header-value"
  }
}
```

If the configuration file is not found or cannot be parsed, the server will use default values.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author

Alexander Mykolaichuk (mykolaichukalexander@gmail.com)
