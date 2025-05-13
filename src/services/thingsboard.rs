use serde_json::json;
use teltonika_rs::protocol::AVLData;
use crate::config::ThingsboardConfig;

pub async fn send_to_thingsboard(record: &AVLData, imei: &str, config: &ThingsboardConfig) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Create a map for IO elements
    let mut io_elements = serde_json::Map::new();

    // Add 1-byte IO elements
    if let Some(elements) = &record.io.io_1_byte_elements {
        for element in elements {
            io_elements.insert(format!("io_1b_{}", element.id), json!(element.value));
        }
    }

    // Add 2-byte IO elements
    if let Some(elements) = &record.io.io_2_byte_elements {
        for element in elements {
            io_elements.insert(format!("io_2b_{}", element.id), json!(element.value));
        }
    }

    // Add 4-byte IO elements
    if let Some(elements) = &record.io.io_4_byte_elements {
        for element in elements {
            io_elements.insert(format!("io_4b_{}", element.id), json!(element.value));
        }
    }

    // Add 8-byte IO elements
    if let Some(elements) = &record.io.io_8_byte_elements {
        for element in elements {
            io_elements.insert(format!("io_8b_{}", element.id), json!(element.value));
        }
    }

    let payload = json!({
        "imei": imei,
        "timestamp": record.timestamp,
        "priority": record.priority,
        "longitude": record.gps.longitude,
        "latitude": record.gps.latitude,
        "altitude": record.gps.altitude,
        "angle": record.gps.angle,
        "satellites": record.gps.visible_satellites,
        "speed": record.gps.speed,
        "event_io_id": record.io.event_io_id,
        "total_io": record.io.number_of_total_io,
        "io_elements": io_elements
    });

    client
        .post(&config.http_integration_url)
        .header("Content-Type", "application/json")
        .header(&config.auth_header_name, &config.auth_header_value)
        .json(&payload)
        .send()
        .await?;

    Ok(())
} 
