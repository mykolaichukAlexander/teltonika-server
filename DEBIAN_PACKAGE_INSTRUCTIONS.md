# Building the Debian Package with Logging to /var/log

This document provides instructions for building the Teltonika Server Debian package with logging configured to write to the `/var/log/teltonika-server/` directory.

## Prerequisites

- Rust and Cargo installed
- cargo-deb installed (`cargo install cargo-deb`)
- Linux environment (for building the Debian package)

## Building the Package

1. Make sure you have the latest code with the logging configuration changes:
   - Updated Cargo.toml with log directory in assets
   - Added postinst script in debian/postinst
   - Updated systemd service file with log file paths

2. Build the release binary:
   ```bash
   cargo build --release
   ```

3. Build the Debian package:
   ```bash
   cargo deb
   ```

4. The Debian package will be created in the `target/debian/` directory with a name like `teltonika-server_0.1.0-1_amd64.deb`.

## Installing the Package

1. Install the package on your Debian/Ubuntu system:
   ```bash
   sudo dpkg -i target/debian/teltonika-server_0.1.0-1_amd64.deb
   ```

2. If there are any dependency issues, resolve them with:
   ```bash
   sudo apt-get install -f
   ```

## Verifying Logging

After installation, the service should be running and logging to the `/var/log/teltonika-server/` directory:

1. Check if the service is running:
   ```bash
   sudo systemctl status teltonika-server
   ```

2. Check the log files:
   ```bash
   sudo cat /var/log/teltonika-server/teltonika-server.log
   sudo cat /var/log/teltonika-server/teltonika-server.error.log
   ```

3. Follow the logs in real-time:
   ```bash
   sudo tail -f /var/log/teltonika-server/teltonika-server.log
   ```

## Troubleshooting

If you encounter any issues with logging:

1. Make sure the `/var/log/teltonika-server/` directory exists and has the correct permissions:
   ```bash
   sudo ls -la /var/log/teltonika-server/
   ```
   
   The directory should be owned by the teltonika-server user and group:
   ```
   drwxr-xr-x 2 teltonika-server teltonika-server 4096 May 15 12:00 /var/log/teltonika-server/
   ```

2. If the directory doesn't exist or has incorrect permissions, you can manually create it:
   ```bash
   sudo mkdir -p /var/log/teltonika-server/
   sudo chown -R teltonika-server:teltonika-server /var/log/teltonika-server/
   sudo chmod 755 /var/log/teltonika-server/
   ```

3. Restart the service:
   ```bash
   sudo systemctl restart teltonika-server
   ```
