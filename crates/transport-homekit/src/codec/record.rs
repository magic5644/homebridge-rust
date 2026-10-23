use bridge_core::error::BridgeError;
use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit, Nonce,
    aead::{Aead, Payload},
};

pub const MAX_PLAINTEXT_BYTES: usize = 1024;
pub const MAX_RECORD_BYTES: usize = 1042;

/// One direction of one session; deliberately not Clone or Debug.
/// The caller owns key generation/erasure and must never reuse a session key.
pub struct RecordCipher {
    cipher: ChaCha20Poly1305,
    counter: u64,
    failed: bool,
}
impl RecordCipher {
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            cipher: ChaCha20Poly1305::new(key.into()),
            counter: 0,
            failed: false,
        }
    }
    fn nonce(&self) -> Nonce {
        let mut nonce = [0; 12];
        nonce[4..].copy_from_slice(&self.counter.to_le_bytes());
        Nonce::from(nonce)
    }
    pub fn seal(&mut self, plaintext: &[u8]) -> Result<Vec<u8>, BridgeError> {
        if self.failed || self.counter == u64::MAX {
            self.failed = true;
            return Err(BridgeError::Authentication);
        }
        if plaintext.len() > MAX_PLAINTEXT_BYTES {
            return Err(BridgeError::Validation);
        }
        let header = (plaintext.len() as u16).to_le_bytes();
        let ciphertext = self
            .cipher
            .encrypt(
                &self.nonce(),
                Payload {
                    msg: plaintext,
                    aad: &header,
                },
            )
            .map_err(|_| {
                self.failed = true;
                BridgeError::Authentication
            })?;
        self.counter += 1;
        let mut record = Vec::with_capacity(header.len() + ciphertext.len());
        record.extend_from_slice(&header);
        record.extend_from_slice(&ciphertext);
        Ok(record)
    }
    /// Only call with one complete frame; TCP buffering and writes belong to the adapter.
    pub fn open(&mut self, record: &[u8]) -> Result<Vec<u8>, BridgeError> {
        if self.failed || self.counter == u64::MAX {
            self.failed = true;
            return Err(BridgeError::Authentication);
        }
        self.failed = true;
        if record_length(record)? != Some(record.len()) {
            return Err(BridgeError::Validation);
        }
        let plaintext = self
            .cipher
            .decrypt(
                &self.nonce(),
                Payload {
                    msg: &record[2..],
                    aad: &record[..2],
                },
            )
            .map_err(|_| BridgeError::Authentication)?;
        self.counter += 1;
        self.failed = false;
        Ok(plaintext)
    }
}

pub fn record_length(prefix: &[u8]) -> Result<Option<usize>, BridgeError> {
    if prefix.len() < 2 {
        return Ok(None);
    }
    let length = usize::from(u16::from_le_bytes([prefix[0], prefix[1]]));
    if length > MAX_PLAINTEXT_BYTES {
        return Err(BridgeError::Validation);
    }
    Ok(Some(length + 18))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhausted_counter_never_wraps() {
        let mut encoder = RecordCipher::new(&[7; 32]);
        encoder.counter = u64::MAX;
        assert!(encoder.seal(b"x").is_err());
        let frame = RecordCipher::new(&[7; 32]).seal(b"x").unwrap();
        let mut decoder = RecordCipher::new(&[7; 32]);
        decoder.counter = u64::MAX;
        assert!(decoder.open(&frame).is_err());
        assert!(decoder.open(&frame).is_err());
    }
}
