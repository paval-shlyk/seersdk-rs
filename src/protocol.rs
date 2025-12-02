use bytes::{Buf, BufMut, BytesMut};
use crate::frame::RbkFrame;

// Protocol constants
const START_MARK: u8 = 0x5A;
const PROTO_VERSION: u8 = 0x01;
const HEAD_SIZE: usize = 16;
const RESERVED: [u8; 6] = [0; 6];

/// Encode an RBK request into bytes
pub(crate) fn encode_request(api_no: u16, body_str: &str, flow_no: u16) -> BytesMut {
    let body_bytes = body_str.as_bytes();
    let body_len = body_bytes.len() as u32;
    
    let mut buf = BytesMut::with_capacity(HEAD_SIZE + body_bytes.len());
    
    // Write header
    buf.put_u8(START_MARK);
    buf.put_u8(PROTO_VERSION);
    buf.put_u16(flow_no);
    buf.put_u32(body_len);
    buf.put_u16(api_no);
    buf.put_slice(&RESERVED);
    
    // Write body
    buf.put_slice(body_bytes);
    
    buf
}

/// Decoder state for RBK protocol
pub(crate) struct RbkDecoder {
    started: bool,
    flow_no: u16,
    api_no: u16,
    body_size: i32,
}

impl RbkDecoder {
    pub fn new() -> Self {
        Self {
            started: false,
            flow_no: 0,
            api_no: 0,
            body_size: -1,
        }
    }

    /// Try to decode a frame from the buffer
    /// Returns Some(RbkFrame) if a complete frame was decoded, None otherwise
    pub fn decode(&mut self, buf: &mut BytesMut) -> Option<RbkFrame> {
        loop {
            // Look for start marker
            if !self.started {
                while buf.has_remaining() {
                    if buf.get_u8() == START_MARK {
                        self.started = true;
                        break;
                    }
                }
                if !self.started {
                    return None;
                }
            }

            // Read header
            if self.body_size < 0 {
                if buf.remaining() < 15 {
                    return None;
                }
                
                let _version = buf.get_u8();
                self.flow_no = buf.get_u16();
                self.body_size = buf.get_u32() as i32;
                self.api_no = buf.get_u16();
                buf.advance(6); // Skip reserved bytes
            }

            // Read body
            if buf.remaining() < self.body_size as usize {
                return None;
            }

            let body_str = if self.body_size == 0 {
                String::new()
            } else {
                let body_bytes = buf.split_to(self.body_size as usize);
                String::from_utf8_lossy(&body_bytes).to_string()
            };

            let frame = RbkFrame::new(self.flow_no, self.api_no, body_str);

            // Reset state for next frame
            self.started = false;
            self.flow_no = 0;
            self.api_no = 0;
            self.body_size = -1;

            return Some(frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let api_no = 1007;
        let body = r#"{"simple": true}"#;
        let flow_no = 42;

        let encoded = encode_request(api_no, body, flow_no);
        let mut buf = encoded;

        let mut decoder = RbkDecoder::new();
        let frame = decoder.decode(&mut buf).expect("Should decode frame");

        assert_eq!(frame.flow_no, flow_no);
        assert_eq!(frame.api_no, api_no);
        assert_eq!(frame.body_str, body);
    }
}
