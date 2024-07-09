use uuid::Uuid;

pub trait IdType {
    fn generate_id(value: &str) -> Uuid;
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct SHA1Name;

impl IdType for SHA1Name {
    fn generate_id(value: &str) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_DNS, value.as_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Timestamp;

impl IdType for Timestamp {
    fn generate_id(value: &str) -> Uuid {
        let bytes = value.as_bytes();
        let mut id_bytes = [0u8; 6];

        // Copy the first six bytes or as many bytes as available
        for (i, &byte) in bytes.iter().take(6).enumerate() {
            id_bytes[i] = byte;
        }

        Uuid::now_v6(&id_bytes)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct UnixTime;

impl IdType for UnixTime {
    fn generate_id(_: &str) -> Uuid {
        Uuid::now_v7()
    }
}
