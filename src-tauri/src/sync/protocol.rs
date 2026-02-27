//! Sync Protocol
//!
//! Binary protocol for P2P communication

use super::SyncMessage;
use serde::{Deserialize, Serialize};

/// Message type constants
pub const MSG_TYPE_TEXT: u8 = 0x01;
pub const MSG_TYPE_IMAGE: u8 = 0x02;
pub const MSG_TYPE_FILE: u8 = 0x03;
pub const MSG_TYPE_DELETE: u8 = 0x04;
pub const MSG_TYPE_ACK: u8 = 0x05;
pub const MSG_TYPE_REQUEST: u8 = 0x10;
pub const MSG_TYPE_RESPONSE: u8 = 0x11;

/// Protocol header size (type + length)
pub const HEADER_SIZE: usize = 5;

/// Maximum message size (10MB)
pub const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;

/// Sync protocol for encoding/decoding messages
pub struct SyncProtocol;

impl SyncProtocol {
    /// Encode a sync message to binary
    pub fn encode(message: &SyncMessage) -> Result<Vec<u8>, String> {
        let json = serde_json::to_vec(message)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;

        if json.len() > MAX_MESSAGE_SIZE {
            return Err("Message too large".to_string());
        }

        let msg_type = match message {
            SyncMessage::Text { .. } => MSG_TYPE_TEXT,
            SyncMessage::Image { .. } => MSG_TYPE_IMAGE,
            SyncMessage::File { .. } => MSG_TYPE_FILE,
            SyncMessage::Delete { .. } => MSG_TYPE_DELETE,
            SyncMessage::Ack { .. } => MSG_TYPE_ACK,
            SyncMessage::RequestContent { .. } => MSG_TYPE_REQUEST,
            SyncMessage::ContentResponse { .. } => MSG_TYPE_RESPONSE,
        };

        let mut buffer = Vec::with_capacity(HEADER_SIZE + json.len());
        buffer.push(msg_type);
        buffer.extend_from_slice(&(json.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&json);

        Ok(buffer)
    }

    /// Decode a binary message
    pub fn decode(data: &[u8]) -> Result<SyncMessage, String> {
        if data.len() < HEADER_SIZE {
            return Err("Invalid message: too short".to_string());
        }

        let msg_type = data[0];
        let length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;

        if data.len() != HEADER_SIZE + length {
            return Err("Invalid message: length mismatch".to_string());
        }

        let json = &data[HEADER_SIZE..];
        let message: SyncMessage = serde_json::from_slice(json)
            .map_err(|e| format!("Failed to deserialize message: {}", e))?;

        // Verify message type matches
        let expected_type = match &message {
            SyncMessage::Text { .. } => MSG_TYPE_TEXT,
            SyncMessage::Image { .. } => MSG_TYPE_IMAGE,
            SyncMessage::File { .. } => MSG_TYPE_FILE,
            SyncMessage::Delete { .. } => MSG_TYPE_DELETE,
            SyncMessage::Ack { .. } => MSG_TYPE_ACK,
            SyncMessage::RequestContent { .. } => MSG_TYPE_REQUEST,
            SyncMessage::ContentResponse { .. } => MSG_TYPE_RESPONSE,
        };

        if msg_type != expected_type {
            return Err("Message type mismatch".to_string());
        }

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let message = SyncMessage::Text {
            content_hash: "abc123".to_string(),
            content: "Hello, World!".to_string(),
        };

        let encoded = SyncProtocol::encode(&message).unwrap();
        let decoded = SyncProtocol::decode(&encoded).unwrap();

        match decoded {
            SyncMessage::Text { content, .. } => {
                assert_eq!(content, "Hello, World!");
            }
            _ => panic!("Wrong message type"),
        }
    }
}
