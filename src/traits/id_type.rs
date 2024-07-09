use uuid::Uuid;

pub trait IdType {
    fn generate_id(value: &str) -> Uuid;
}