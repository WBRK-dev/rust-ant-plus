use rusb::{DeviceHandle, GlobalContext, UsbContext};
use std::sync::{Arc, Mutex};

use crate::constants::Constants;
use crate::messages::Messages;

pub struct AntPlusSensor {
    stick: Arc<Mutex<DeviceHandle<GlobalContext>>>,
    channel: Option<u8>,
    device_id: u16,
    transmission_type: u8,
    msg_queue: Vec<Vec<u8>>,
}

impl AntPlusSensor {
    fn new(stick: Arc<Mutex<DeviceHandle<GlobalContext>>>) -> Self {
        AntPlusSensor {
            stick,
            channel: None,
            device_id: 0,
            transmission_type: 0,
            msg_queue: vec![],
        }
    }

    fn scan(&self) -> Result<(), &str> {
        Err("scanning unsupported")
    }

    fn attach(
        &mut self,
        channel: u8,
        type_: &str,
        device_id: u16,
        device_type: u8,
        transmission_type: u8,
        timeout: u8,
        period: u16,
    ) -> Result<(), &str> {
        self.channel = Some(channel);
        self.device_id = device_id;
        self.transmission_type = transmission_type;

        // Call the base attach method (implementation needed)
        // super.attach(channel, type_, device_id, device_type, transmission_type, timeout, period);

        Ok(())
    }

    fn decode_data(&mut self, data: &[u8]) {
        match data[Messages::BUFFER_INDEX_MSG_TYPE] {
            Constants::MESSAGE_CHANNEL_BROADCAST_DATA
            | Constants::MESSAGE_CHANNEL_ACKNOWLEDGED_DATA
            | Constants::MESSAGE_CHANNEL_BURST_DATA => {
                if self.device_id == 0 {
                    self.write(Messages::request_message(self.channel.unwrap(), Constants::MESSAGE_CHANNEL_ID));
                }
                self.update_state(self.device_id, data);
            }
            Constants::MESSAGE_CHANNEL_ID => {
                self.device_id = u16::from_le_bytes([data[Messages::BUFFER_INDEX_MSG_DATA], data[Messages::BUFFER_INDEX_MSG_DATA + 1]]);
                self.transmission_type = data[Messages::BUFFER_INDEX_MSG_DATA + 3];
            }
            _ => {}
        }
    }

    fn write(&self, data: Vec<u8>) {
        let stick = self.stick.lock().unwrap();
        stick.write_bulk(0x01, &data, std::time::Duration::from_secs(1)).unwrap();
    }

    fn update_state(&self, device_id: u16, data: &[u8]) {
        // Implement state update logic
    }
}