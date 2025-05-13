# Teltonika Server

A server for Teltonika GPS trackers that receives data from devices and forwards it to ThingsBoard IoT platform.

## Description

Teltonika Server is a lightweight, high-performance TCP server written in Rust that handles connections from Teltonika GPS tracking devices. It processes the binary data sent by these devices, parses it according to the Teltonika protocols (primarily Codec8), and forwards the structured data to a ThingsBoard IoT platform instance.

## Features

- TCP server listening on port 5027 (standard Teltonika port)
- IMEI-based device authentication
- Support for Teltonika Codec8 protocol
- Real-time data forwarding to ThingsBoard
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
# Run with default settings
./teltonika-server

# Set custom log level
RUST_LOG=debug ./teltonika-server
```

The server looks for a `config.yaml` file in the current directory. If not found, it uses the default configuration (listening on `0.0.0.0:5027` and using the default ThingsBoard integration settings).

## Configuration

The server can be configured using a YAML configuration file (`config.yaml`). The configuration file should be placed in the same directory as the executable.

### Configuration Options

```yaml
# Server configuration
server:
  # Host to bind the server to
  host: "0.0.0.0"
  # Port to listen on
  port: 5027

# ThingsBoard integration configuration
thingsboard:
  # ThingsBoard HTTP integration URL
  http_integration_url: "https://thingsboard.cloud/api/v1/integrations/http/your-integration-token"
  # Authentication header value
  auth_header_name: "your-auth-token",
  auth_header_value: "your-auth-token-value"
```

If the configuration file is not found or cannot be parsed, the server will use default values.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author

Alexander Mykolaichuk (mykolaichukalexander@gmail.com)
