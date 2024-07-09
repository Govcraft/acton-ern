use uuid::Uuid;

pub trait IdType {
    fn generate_id(value: &str) -> Uuid;
}

// Implement the trait for each ID version with user-friendly names
#[derive(Debug, Clone, PartialEq)]
pub struct Random;

impl IdType for Random {
    fn generate_id(_: &str) -> Uuid {
        Uuid::new_v4()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SHA1Name;

impl IdType for SHA1Name {
    fn generate_id(value: &str) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_DNS, value.as_bytes())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Timestamp;

impl IdType for Timestamp {
    fn generate_id(value: &str) -> Uuid {
        Uuid::now_v6(&<[u8; 6]>::try_from(value.as_bytes()).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnixTime;

impl IdType for UnixTime {
    fn generate_id(_: &str) -> Uuid {
        Uuid::now_v7()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefined;

impl IdType for UserDefined {
    fn generate_id(value: &str) -> Uuid {
        // For v8 UUIDs, the user-defined data should be provided
        // Here, we use a simple example of generating a UUID from a fixed namespace
        // Adjust this logic based on your specific use case
        Uuid::new_v8(<[u8; 16]>::try_from(value.as_bytes()).unwrap())
    }
}