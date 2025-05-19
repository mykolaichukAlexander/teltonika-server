use nom_teltonika::{AVLRecord, AVLEventIOValue};
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::config::ApiIntegrationConfig;

pub async fn send_to_api(record: &AVLRecord, imei: &str, config: &ApiIntegrationConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Transform io_events into a map of id -> value
    let mut io_elements = HashMap::new();
    for event in &record.io_events {
        let value = match &event.value {
            AVLEventIOValue::U8(v) => Value::from(*v),
            AVLEventIOValue::U16(v) => Value::from(*v),
            AVLEventIOValue::U32(v) => Value::from(*v),
            AVLEventIOValue::U64(v) => Value::from(*v),
            AVLEventIOValue::Variable(bytes) => {
                // Convert byte array to string for ASCII data
                match String::from_utf8(bytes.clone()) {
                    Ok(s) => Value::String(s),
                    Err(_) => Value::String(format!("{:?}", bytes)), // Fallback to debug representation
                }
            }
        };
        io_elements.insert(event.id.to_string(), value);
    }

    let payload = json!({
        "imei": imei,
        "timestamp": record.timestamp.timestamp_millis(),
        "priority": record.priority,
        "longitude": record.longitude,
        "latitude": record.latitude,
        "altitude": record.altitude,
        "angle": record.angle,
        "satellites": record.satellites,
        "speed": record.speed,
        "generation_type": record.generation_type,
        "io_elements": io_elements
    });

    let resp = client
        .post(&config.http_endpoint_url)
        .header("Content-Type", "application/json")
        .header(&config.auth_header_name, &config.auth_header_value)
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let error_msg = format!("Error sending data to API: {}", resp.status());
        // println!("{}", error_msg);
        // println!("HTTP request failed, status code:");
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)));
    }
    Ok(())
} 
