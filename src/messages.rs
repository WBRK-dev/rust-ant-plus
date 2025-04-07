// Constants module to define all constants used in Messages
mod constants {
    pub const MESSAGE_SYSTEM_RESET: u8 = 0x4A; // Example value
    pub const MESSAGE_CHANNEL_REQUEST: u8 = 0x4B; // Example value
    pub const MESSAGE_NETWORK_KEY: u8 = 0x4C; // Example value
    pub const DEFAULT_NETWORK_NUMBER: u8 = 0x00;
    // Add other constants as needed
}

use constants::*;
use std::convert::TryInto;

pub struct Messages;

impl Messages {
    pub const BUFFER_INDEX_MSG_LEN: usize = 1;
    pub const BUFFER_INDEX_MSG_TYPE: usize = 2;
    pub const BUFFER_INDEX_CHANNEL_NUM: usize = 3;
    pub const BUFFER_INDEX_MSG_DATA: usize = 4;
    pub const BUFFER_INDEX_EXT_MSG_BEGIN: usize = 12;

    pub fn reset_system() -> Vec<u8> {
        let payload = vec![0x00];
        Self::build_message(&payload, MESSAGE_SYSTEM_RESET)
    }

    pub fn request_message(channel: u8, message_id: u8) -> Vec<u8> {
        let mut payload = vec![channel];
        payload.push(message_id);
        Self::build_message(&payload, MESSAGE_CHANNEL_REQUEST)
    }

    pub fn set_network_key() -> Vec<u8> {
        let payload = vec![
            DEFAULT_NETWORK_NUMBER,
            0xB9, 0xA5, 0x21, 0xFB,
            0xBD, 0x72, 0xC3, 0x45,
        ];
        Self::build_message(&payload, MESSAGE_NETWORK_KEY)
    }

    // Additional methods translated similarly...

    fn build_message(payload: &[u8], msg_id: u8) -> Vec<u8> {
        let mut message = Vec::with_capacity(payload.len() + 4);
        message.push(0xA4); // MESSAGE_TX_SYNC
        message.push(payload.len() as u8);
        message.push(msg_id);
        message.extend_from_slice(payload);
        let checksum = Self::calculate_checksum(&message);
        message.push(checksum);
        message
    }

    fn calculate_checksum(message: &[u8]) -> u8 {
        message.iter().fold(0u8, |acc, &x| acc ^ x)
    }
}
