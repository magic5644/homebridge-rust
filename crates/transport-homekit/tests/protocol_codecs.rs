use bridge_core::error::BridgeError;
use transport_homekit::codec::{
    record::*,
    tlv::{self, Tlv},
};

fn item(tag: u8, value: Vec<u8>) -> Tlv {
    Tlv { tag, value }
}

#[test]
fn tlv_fragmentation_and_separators_are_exact() {
    let items = vec![
        item(3, vec![7; 384]),
        item(6, vec![2]),
        item(255, vec![]),
        item(6, vec![3]),
    ];
    let encoded = tlv::encode(&items).unwrap();
    assert_eq!(&encoded[..2], &[3, 255]);
    assert_eq!(&encoded[257..259], &[3, 129]);
    assert_eq!(tlv::decode(&encoded).unwrap(), items);
    for len in [0, 1, 254, 255, 256, 510, 511] {
        let items = vec![item(1, vec![42; len])];
        assert_eq!(tlv::decode(&tlv::encode(&items).unwrap()).unwrap(), items);
    }
}

#[test]
fn malformed_tlv_is_rejected_without_panic() {
    for bytes in [
        vec![1],
        vec![1, 2, 3],
        vec![1, 1, 0, 1, 1, 0],
        vec![1, 1, 0, 2, 0, 1, 0],
        vec![255, 1, 0],
        vec![0; 8193],
    ] {
        assert_eq!(tlv::decode(&bytes), Err(BridgeError::Validation));
    }
    assert!(tlv::encode(&[item(255, vec![1])]).is_err());
    assert!(tlv::encode(&[item(1, vec![0; 8192])]).is_err());
    assert!(tlv::encode(&vec![item(255, vec![]); 257]).is_err());
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../fixtures/homekit/record-vector.json")).unwrap()
}
fn unhex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}

#[test]
fn records_match_independent_cryptography_vectors() {
    let fixture = fixture();
    let key: [u8; 32] = unhex(fixture["key_hex"].as_str().unwrap())
        .try_into()
        .unwrap();
    let mut encoder = RecordCipher::new(&key);
    let mut decoder = RecordCipher::new(&key);
    for vector in fixture["records"].as_array().unwrap() {
        let plaintext = unhex(vector["plaintext_hex"].as_str().unwrap());
        let record = unhex(vector["record_hex"].as_str().unwrap());
        assert_eq!(encoder.seal(&plaintext).unwrap(), record);
        assert_eq!(decoder.open(&record).unwrap(), plaintext);
    }
}

#[test]
fn frame_prefix_bounds_and_partial_input() {
    assert_eq!(record_length(&[]), Ok(None));
    assert_eq!(record_length(&[1]), Ok(None));
    assert_eq!(record_length(&[1, 0]), Ok(Some(19)));
    assert_eq!(record_length(&[0, 4]), Ok(Some(MAX_RECORD_BYTES)));
    assert!(record_length(&[1, 4]).is_err());
    assert_eq!(record_length(&[0, 0]), Ok(Some(18)));
    let mut encoder = RecordCipher::new(&[7; 32]);
    assert_eq!(RecordCipher::new(&[7; 32]).seal(&[]).unwrap().len(), 18);
    assert!(encoder.seal(&[0; 1025]).is_err());
    let record = encoder.seal(&[42; 1024]).unwrap();
    assert_eq!(record.len(), MAX_RECORD_BYTES);
    for cut in 0..record.len() {
        assert!(RecordCipher::new(&[7; 32]).open(&record[..cut]).is_err());
    }
    let mut extra = record.clone();
    extra.push(0);
    assert!(RecordCipher::new(&[7; 32]).open(&extra).is_err());
}

#[test]
fn tamper_replay_and_wrong_direction_fail_closed() {
    let key = [7; 32];
    let record = RecordCipher::new(&key).seal(b"authenticated").unwrap();
    for index in 0..record.len() {
        let mut bad = record.clone();
        bad[index] ^= 1;
        let mut decoder = RecordCipher::new(&key);
        assert!(decoder.open(&bad).is_err());
        assert!(
            decoder.open(&record).is_err(),
            "a failed session must remain closed"
        );
    }
    let mut decoder = RecordCipher::new(&key);
    assert_eq!(decoder.open(&record).unwrap(), b"authenticated");
    assert!(decoder.open(&record).is_err());
    assert!(RecordCipher::new(&[8; 32]).open(&record).is_err());
}
