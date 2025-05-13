use log::{error, info};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use teltonika_rs::parser::parse_teltonika_codec_8;
use teltonika_rs::parser::parse_teltonika_imei;
use crate::models::ConnectionState;
use crate::services::thingsboard::send_to_thingsboard;
use crate::config::Config;

// First, add this new function to detect if a message is an IMEI message
fn is_imei_message(data: &[u8]) -> bool {
    // IMEI messages are typically 15-17 digits in ASCII format
    // A simple check would be to verify if the message consists of ASCII digits
    // and has a reasonable length for an IMEI

    // Check if length is reasonable for an IMEI message (with protocol overhead)
    if data.len() < 15 || data.len() > 25 {
        return false;
    }

    // Try to parse - if it succeeds, it's an IMEI message
    match parse_teltonika_imei(data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Determines whether the server should accept data from a device with the given IMEI
fn is_imei_authorized(_imei: &str) -> bool {
    // TODO: Implement your authorization logic here
    // For example:
    // 1. Check against a whitelist of IMEIs
    // 2. Verify against a database
    // 3. Make an API call to an external service

    // For now, return a simple implementation (e.g., accept all IMEIs)
    // You should replace this with your actual authorization logic
    true // or some actual validation logic
}





// Modified function to detect different device message types
fn identify_message_type(data: &[u8], state: &ConnectionState) -> MessageType {
    // Check for single-byte partial ACK first byte
    if data.len() == 1 && data[0] == 0xfe {
        return MessageType::PartialAck;
    }

    // Check for single-byte ACK second byte when we have a partial_ack stored
    if data.len() == 1 && state.partial_ack.is_some() {
        return MessageType::AckCount(data[0]);
    }

    // Check for complete ACK message in a single packet
    if data.len() == 2 && data[0] == 0xfe {
        return MessageType::AckCount(data[1]);
    }

    // Check for IMEI message
    if is_imei_message(data) {
        return MessageType::Imei;
    }

    // Otherwise it's a data message
    MessageType::Data
}

// Define message types for better readability
enum MessageType {
    Imei,
    PartialAck,
    AckCount(u8),
    Data,
}

// In the handle_connection function:
pub async fn handle_connection(mut socket: TcpStream, addr: SocketAddr, config: Config) {
    info!("New connection from: {}", addr);

    let mut buffer = [0; 4096];
    let mut state = ConnectionState::new();

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                info!("Connection closed by client: {}", addr);
                break;
            }
            Ok(n) => {
                let data = &buffer[..n];
                info!("Received binary data ({} bytes):", n);
                info!("IMEI: {:?}", state.imei);
                info!("Hex: {}", data.iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join(" "));

                let message_type = identify_message_type(data, &state);

                match message_type {
                    MessageType::Imei => {
                        match parse_teltonika_imei(data) {
                            Ok(imei) => {
                                let imei_str = imei.1.to_string();
                                info!("Received IMEI: {}", imei_str);

                                // Check if the IMEI is authorized
                                if is_imei_authorized(&imei_str) {
                                    info!("IMEI {} is authorized", imei_str);
                                    state.imei = Some(imei_str);
                                    // Send acceptance (0x01)
                                    if let Err(e) = socket.write_all(&[0x01]).await {
                                        error!("Failed to send IMEI acceptance: {}", e);
                                    }
                                } else {
                                    info!("IMEI {} is not authorized", imei_str);
                                    // Send rejection (0x00)
                                    if let Err(e) = socket.write_all(&[0x00]).await {
                                        error!("Failed to send IMEI rejection: {}", e);
                                    }
                                    // Close the connection after rejection
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("Unexpected error parsing IMEI: {}", e);
                            }
                        }
                    },
                    MessageType::PartialAck => {
                        // Store the first byte of a split ACK message
                        info!("Received first byte of ACK message (0xfe)");
                        state.partial_ack = Some(0xfe);
                    },
                    MessageType::AckCount(count) => {
                        // Handle device acknowledgment message
                        info!("Received device acknowledgment for {} messages", count);
                        // Reset partial ACK state
                        state.partial_ack = None;

                        // You might want to add specific handling for device acknowledgments here
                        // For example, update a counter or trigger some action
                    },
                    MessageType::Data => {
                        // Reset partial ACK state if we get a data message
                        state.partial_ack = None;

                        if let Some(imei) = &state.imei {
                            // We have an IMEI, so process as data
                            match parse_teltonika_codec_8(data) {
                                Ok(teltonika_data) => {
                                    let records_len = teltonika_data.1.avl_data.len();
                                    info!("Parsed {} records for IMEI {}", records_len, imei);

                                    for record in teltonika_data.1.avl_data {
                                        info!("Record: {:?}", record);
                                        if let Err(e) = send_to_thingsboard(&record, imei, &config.thingsboard).await {
                                            error!("Failed to send to ThingsBoard: {}", e);
                                        }
                                    }

                                    // Send acknowledgment with the count of processed records
                                    // The format is a 4-byte number in network byte order (big-endian)
                                    let count_bytes = (records_len as u32).to_be_bytes();
                                    if let Err(e) = socket.write_all(&count_bytes).await {
                                        error!("Failed to send data acknowledgment: {}", e);
                                    } else {
                                        info!("Sent acknowledgment for {} records", records_len);
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse Codec8 data: {}", e);
                                }
                            }
                        } else {
                            // We don't have an IMEI yet and the message isn't an IMEI message
                            error!("Received non-IMEI data before IMEI was established");
                            // You might want to close the connection here or ignore the data
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}
