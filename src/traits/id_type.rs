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
        Uuid::now_v6(&<[u8; 6]>::try_from(value.as_bytes()).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct UnixTime;

impl IdType for UnixTime {
    fn generate_id(_: &str) -> Uuid {
        Uuid::now_v7()
    }
}
