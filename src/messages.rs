use crate::constants::*;

pub struct Messages;

impl Messages {
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

    fn build_message(payload: &[u8], msg_id: u8) -> Vec<u8> {
        let mut message = Vec::with_capacity(payload.len() + 4);
        message.push(MESSAGE_TX_SYNC);
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