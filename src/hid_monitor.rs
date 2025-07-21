use anyhow::{Context, Result};
use hidapi::{HidApi, HidDevice, DeviceInfo}; // Fixed: HidDeviceInfo -> DeviceInfo
use log::{debug, info};
use std::time::{Duration, Instant};

#[derive(Debug)] // Added Debug derive
pub struct HidMonitor {
    vendor_id: u16,
    product_ids: Vec<u16>,
    last_scan: Instant,
    scan_interval: Duration,
}

impl HidMonitor {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Self> {
        Ok(HidMonitor {
            vendor_id,
            product_ids: vec![product_id],
            last_scan: Instant::now(),
            scan_interval: Duration::from_secs(5),
        })
    }

    pub fn new_multi_pid(vendor_id: u16, product_ids: Vec<u16>) -> Result<Self> {
        Ok(HidMonitor {
            vendor_id,
            product_ids,
            last_scan: Instant::now(),
            scan_interval: Duration::from_secs(5),
        })
    }

    pub fn find_device(&mut self) -> Result<Option<DeviceInfo>> { // Fixed return type
        // Rate limit device scanning
        if self.last_scan.elapsed() < self.scan_interval {
            return Ok(None);
        }

        self.last_scan = Instant::now();

        let api = HidApi::new()
            .context("Failed to initialize HID API")?;

        for device_info in api.device_list() {
            if device_info.vendor_id() == self.vendor_id && 
               self.product_ids.contains(&device_info.product_id()) {
                
                info!("Found SVBONY device: {:?} (PID: {:04x})", 
                      device_info.product_string(), device_info.product_id());
                debug!("Device path: {:?}", device_info.path());
                debug!("Manufacturer: {:?}", device_info.manufacturer_string());
                debug!("Serial: {:?}", device_info.serial_number());
                
                return Ok(Some(device_info.clone()));
            }
        }

        debug!("SVBONY device not found (VID: {:04x}, PIDs: {:04x?})", 
               self.vendor_id, self.product_ids);
        Ok(None)
    }

    pub fn connect_device(&self, device_info: &DeviceInfo) -> Result<HidDevice> { // Fixed parameter type
        let api = HidApi::new()
            .context("Failed to initialize HID API")?;

        let device = api.open_path(device_info.path())
            .context("Failed to open HID device")?;

        // Configure device for optimal performance
        device.set_blocking_mode(false)
            .context("Failed to set non-blocking mode")?;

        info!("Successfully connected to SVBONY device");
        Ok(device)
    }

    pub fn validate_hid_data(&self, data: &[u8], expected_pattern: &[u8]) -> bool {
        if data.len() < expected_pattern.len() {
            return false;
        }

        // Check if the beginning of the data matches our expected pattern
        data[..expected_pattern.len()] == *expected_pattern
    }

    pub fn parse_hid_report(&self, data: &[u8]) -> Option<HidReport> {
        if data.is_empty() {
            return None;
        }

        let report_id = data[0];
        let payload = &data[1..];

        Some(HidReport {
            report_id,
            payload: payload.to_vec(),
            timestamp: Instant::now(),
        })
    }

    pub fn is_ai_button_press(&self, report: &HidReport) -> bool {
        // Check for AI button pattern: 04b2000001000100...
        if report.report_id != 0x04 {
            return false;
        }

        if report.payload.len() < 7 {
            return false;
        }

        // Check for the specific pattern: b2 00 00 01 00 01 00
        let expected_payload = [0xb2, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00];
        report.payload[..expected_payload.len()] == expected_payload
    }

    pub fn log_hid_data(&self, data: &[u8], size: usize) {
        if log::log_enabled!(log::Level::Debug) {
            debug!("HID Data received ({} bytes):", size);
            
            // Log data in hex format, 16 bytes per line
            for (i, chunk) in data[..size].chunks(16).enumerate() {
                let hex_str: String = chunk.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                
                let ascii_str: String = chunk.iter()
                    .map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' })
                    .collect();

                debug!("{:04x}: {:48} |{}|", i * 16, hex_str, ascii_str);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HidReport {
    pub report_id: u8,
    pub payload: Vec<u8>,
    pub timestamp: Instant,
}

impl HidReport {
    pub fn age(&self) -> Duration {
        self.timestamp.elapsed()
    }

    pub fn to_hex_string(&self) -> String {
        let mut hex_data = vec![self.report_id];
        hex_data.extend_from_slice(&self.payload);
        
        hex_data.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_button_pattern_recognition() {
        // let monitor = HidMonitor::new(0xe2b7, 0x364d).unwrap();
        let monitor = HidMonitor::new(0xe2b7, 0x5053).unwrap();
        
        // Test valid AI button report
        let valid_report = HidReport {
            report_id: 0x04,
            payload: vec![0xb2, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00],
            timestamp: Instant::now(),
        };
        
        assert!(monitor.is_ai_button_press(&valid_report));
        
        // Test invalid report ID
        let invalid_id_report = HidReport {
            report_id: 0x03,
            payload: vec![0xb2, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00],
            timestamp: Instant::now(),
        };
        
        assert!(!monitor.is_ai_button_press(&invalid_id_report));
        
        // Test invalid payload
        let invalid_payload_report = HidReport {
            report_id: 0x04,
            payload: vec![0xb3, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00],
            timestamp: Instant::now(),
        };
        
        assert!(!monitor.is_ai_button_press(&invalid_payload_report));
    }

    #[test]
    fn test_hex_string_conversion() {
        let report = HidReport {
            report_id: 0x04,
            payload: vec![0xb2, 0x00, 0x00, 0x01],
            timestamp: Instant::now(),
        };
        
        let hex_string = report.to_hex_string();
        assert_eq!(hex_string, "04b2000001");
    }

    #[test]
    fn test_validate_hid_data() {
        // let monitor = HidMonitor::new(0xe2b7, 0x364d).unwrap();
        let monitor = HidMonitor::new(0xe2b7, 0x5053).unwrap();
        
        let data = [0x04, 0xb2, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00];
        let pattern = [0x04, 0xb2, 0x00, 0x00];
        
        assert!(monitor.validate_hid_data(&data, &pattern));
        
        let invalid_data = [0x04, 0xb3, 0x00, 0x00];
        assert!(!monitor.validate_hid_data(&invalid_data, &pattern));
    }

    #[test]
    fn test_parse_hid_report() {
        // let monitor = HidMonitor::new(0xe2b7, 0x364d).unwrap();
        let monitor = HidMonitor::new(0xe2b7, 0x5053).unwrap();
        
        let data = [0x04, 0xb2, 0x00, 0x00, 0x01];
        let report = monitor.parse_hid_report(&data).unwrap();
        
        assert_eq!(report.report_id, 0x04);
        assert_eq!(report.payload, vec![0xb2, 0x00, 0x00, 0x01]);
        
        // Test empty data
        let empty_data = [];
        assert!(monitor.parse_hid_report(&empty_data).is_none());
    }
}