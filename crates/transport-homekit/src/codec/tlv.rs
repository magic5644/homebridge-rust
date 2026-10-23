use bridge_core::error::BridgeError;

pub const MAX_MESSAGE_BYTES: usize = 8192;
pub const MAX_ITEMS: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tlv {
    pub tag: u8,
    pub value: Vec<u8>,
}

pub fn decode(input: &[u8]) -> Result<Vec<Tlv>, BridgeError> {
    if input.len() > MAX_MESSAGE_BYTES {
        return Err(BridgeError::Validation);
    }
    let mut out: Vec<Tlv> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut offset = 0;
    let mut previous_length = 0;
    while offset < input.len() {
        let header = input
            .get(offset..offset + 2)
            .ok_or(BridgeError::Validation)?;
        let tag = header[0];
        let length = usize::from(header[1]);
        offset += 2;
        let value = input
            .get(offset..offset + length)
            .ok_or(BridgeError::Validation)?;
        offset += length;
        if tag == 255 {
            if length != 0 {
                return Err(BridgeError::Validation);
            }
            seen.clear();
        } else if previous_length == 255 && out.last().is_some_and(|item| item.tag == tag) {
            out.last_mut()
                .ok_or(BridgeError::Validation)?
                .value
                .extend_from_slice(value);
            previous_length = length;
            continue;
        } else if !seen.insert(tag) {
            return Err(BridgeError::Validation);
        }
        if out.len() == MAX_ITEMS {
            return Err(BridgeError::Validation);
        }
        out.push(Tlv {
            tag,
            value: value.to_vec(),
        });
        previous_length = length;
    }
    Ok(out)
}

pub fn encode(items: &[Tlv]) -> Result<Vec<u8>, BridgeError> {
    if items.len() > MAX_ITEMS {
        return Err(BridgeError::Validation);
    }
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for item in items {
        if item.tag == 255 {
            if !item.value.is_empty() {
                return Err(BridgeError::Validation);
            }
            seen.clear();
        } else if !seen.insert(item.tag) {
            return Err(BridgeError::Validation);
        }
        if item.value.len() > MAX_MESSAGE_BYTES {
            return Err(BridgeError::Validation);
        }
        let encoded_length = item.value.len() + 2 * item.value.len().div_ceil(255).max(1);
        if out.len() + encoded_length > MAX_MESSAGE_BYTES {
            return Err(BridgeError::Validation);
        }
        if item.value.is_empty() {
            out.extend_from_slice(&[item.tag, 0]);
        }
        for chunk in item.value.chunks(255) {
            out.extend_from_slice(&[item.tag, chunk.len() as u8]);
            out.extend_from_slice(chunk);
        }
    }
    Ok(out)
}
