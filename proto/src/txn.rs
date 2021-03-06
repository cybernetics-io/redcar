#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(enumeration = "event::EventAction", tag = "1")]
    pub action: i32,
    #[prost(message, optional, tag = "2")]
    pub kv: ::core::option::Option<KeyValue>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventAction {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration = "message::MessageType", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub kv: ::core::option::Option<KeyValue>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        Heartbeat = 0,
        State = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudEvent {
    // -- CloudEvent Context Attributes
    /// Required Attributes
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// URI-reference
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub spec_version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional & Extension Attributes
    #[prost(map = "string, message", tag = "5")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        cloud_event::CloudEventAttributeValue,
    >,
    /// -- CloudEvent Data (Bytes, Text, or Proto)
    #[prost(oneof = "cloud_event::Data", tags = "6, 7, 8")]
    pub data: ::core::option::Option<cloud_event::Data>,
}
/// Nested message and enum types in `CloudEvent`.
pub mod cloud_event {
    //*
    // The CloudEvent specification defines
    // seven attribute value types...

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudEventAttributeValue {
        #[prost(
            oneof = "cloud_event_attribute_value::Attr",
            tags = "1, 2, 3, 4, 5, 6, 7"
        )]
        pub attr: ::core::option::Option<cloud_event_attribute_value::Attr>,
    }
    /// Nested message and enum types in `CloudEventAttributeValue`.
    pub mod cloud_event_attribute_value {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Attr {
            #[prost(bool, tag = "1")]
            CeBoolean(bool),
            #[prost(int32, tag = "2")]
            CeInteger(i32),
            #[prost(string, tag = "3")]
            CeString(::prost::alloc::string::String),
            #[prost(bytes, tag = "4")]
            CeBytes(::prost::alloc::vec::Vec<u8>),
            #[prost(string, tag = "5")]
            CeUri(::prost::alloc::string::String),
            #[prost(string, tag = "6")]
            CeUriRef(::prost::alloc::string::String),
            #[prost(message, tag = "7")]
            CeTimestamp(::prost_types::Timestamp),
        }
    }
    /// -- CloudEvent Data (Bytes, Text, or Proto)
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(bytes, tag = "6")]
        BinaryData(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "7")]
        TextData(::prost::alloc::string::String),
        #[prost(message, tag = "8")]
        ProtoData(::prost_types::Any),
    }
}
//*
// CloudEvent Protobuf Batch Format
//

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudEventBatch {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<CloudEvent>,
}
