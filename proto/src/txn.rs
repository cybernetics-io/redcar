#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(enumeration = "event::EventType", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub kv: ::core::option::Option<KeyValue>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        Put = 0,
        Delete = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    pub create_revision: i64,
    #[prost(int64, tag = "3")]
    pub mod_revision: i64,
    #[prost(int64, tag = "4")]
    pub version: i64,
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
