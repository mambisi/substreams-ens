// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigDecimal {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resolver {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub content_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "5")]
    pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub coin_types: ::prost::alloc::vec::Vec<BigInt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub label_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub label_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub parent: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub resolved_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub ttl: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "9")]
    pub is_migrated: bool,
    #[prost(uint64, tag = "10")]
    pub created_at_timestamp: u64,
    #[prost(uint64, tag = "11")]
    pub created_at_block: u64,
    /// internals
    #[prost(uint64, tag = "30")]
    pub log_ordinal: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domains {
    #[prost(message, repeated, tag = "1")]
    pub domains: ::prost::alloc::vec::Vec<Domain>,
}
/// Encoded file descriptor set for the `ens.types` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9b, 0x10, 0x0a, 0x09, 0x65, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x09,
    0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x22, 0x1e, 0x0a, 0x06, 0x42, 0x69, 0x67,
    0x49, 0x6e, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x22, 0x0a, 0x0a, 0x42, 0x69, 0x67,
    0x44, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xb7, 0x01,
    0x0a, 0x08, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x6f,
    0x6d, 0x61, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x64, 0x6f, 0x6d, 0x61,
    0x69, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x21, 0x0a, 0x0c,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68, 0x12,
    0x14, 0x0a, 0x05, 0x74, 0x65, 0x78, 0x74, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05,
    0x74, 0x65, 0x78, 0x74, 0x73, 0x12, 0x30, 0x0a, 0x0a, 0x63, 0x6f, 0x69, 0x6e, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x09, 0x63, 0x6f,
    0x69, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x73, 0x22, 0xf3, 0x02, 0x0a, 0x06, 0x44, 0x6f, 0x6d, 0x61,
    0x69, 0x6e, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02,
    0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6c, 0x61, 0x62, 0x65,
    0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x5f, 0x68,
    0x61, 0x73, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x6c, 0x61, 0x62, 0x65, 0x6c,
    0x48, 0x61, 0x73, 0x68, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x12, 0x14, 0x0a, 0x05,
    0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x12, 0x29, 0x0a, 0x10, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x5f, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0f, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x10, 0x0a,
    0x03, 0x74, 0x74, 0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x74, 0x74, 0x6c, 0x12,
    0x1f, 0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x64, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x4d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x64,
    0x12, 0x30, 0x0a, 0x14, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x04, 0x52, 0x12,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x12, 0x28, 0x0a, 0x10, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74,
    0x5f, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0e, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x1f, 0x0a, 0x0b,
    0x6c, 0x6f, 0x67, 0x5f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0x1e, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x0a, 0x6c, 0x6f, 0x67, 0x4f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x22, 0x36, 0x0a,
    0x07, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x2b, 0x0a, 0x07, 0x64, 0x6f, 0x6d, 0x61,
    0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x52, 0x07, 0x64, 0x6f,
    0x6d, 0x61, 0x69, 0x6e, 0x73, 0x4a, 0xce, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x27, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x01, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x05, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x04, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x04, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x04, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07, 0x00, 0x09, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x08, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08,
    0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0b, 0x00, 0x12, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0c, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x0e,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x0e, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x0f, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x10, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x10, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x10, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x12, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x11,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x11, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x14, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x15, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x15, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x09,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x0e, 0x0f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x16, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x16, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03,
    0x17, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x09, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x16, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x18, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x18, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x18, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x18, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x19, 0x02,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x19, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x19, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x1a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x1a, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1a,
    0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1b, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x07, 0x12, 0x03, 0x1c, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12,
    0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1c,
    0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1c, 0x0e, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x03, 0x1d, 0x02, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x1d, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12,
    0x03, 0x1e, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x05, 0x12, 0x03, 0x1e,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1e, 0x09, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1e, 0x20, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x0a, 0x12, 0x03, 0x1f, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x1f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x1f, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x1f, 0x1c, 0x1e, 0x0a, 0x18, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0b, 0x12, 0x03, 0x22,
    0x02, 0x1a, 0x1a, 0x0b, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x73, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x22, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x22, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x22, 0x17, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x25, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x25, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x26, 0x02, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x26, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x26, 0x1c, 0x1d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
