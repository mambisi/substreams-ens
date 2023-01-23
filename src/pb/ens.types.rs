// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigInt {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigDecimal {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Registration {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub domains: ::prost::alloc::vec::Vec<Domain>,
    #[prost(message, optional, tag="3")]
    pub registration_date: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="4")]
    pub expiry_date: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="5")]
    pub cost: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="6")]
    pub registrant: ::core::option::Option<Account>,
    #[prost(string, tag="7")]
    pub label_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub domains: ::prost::alloc::vec::Vec<Domain>,
    #[prost(message, repeated, tag="3")]
    pub wrapped_domains: ::prost::alloc::vec::Vec<WrappedDomains>,
    #[prost(message, repeated, tag="4")]
    pub registrations: ::prost::alloc::vec::Vec<Registration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolverEvent {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub resolver: ::core::option::Option<Resolver>,
    #[prost(uint64, tag="3")]
    pub block_number: u64,
    #[prost(bytes="vec", tag="4")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resolver {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag="2")]
    pub domain: ::core::option::Option<::prost::alloc::boxed::Box<Domain>>,
    #[prost(bytes="vec", tag="3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub addr: ::core::option::Option<Account>,
    #[prost(bytes="vec", tag="5")]
    pub content_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="6")]
    pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="7")]
    pub coin_types: ::prost::alloc::vec::Vec<BigInt>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrappedDomains {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub domain: ::core::option::Option<Domain>,
    #[prost(message, optional, tag="3")]
    pub expiry_date: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="4")]
    pub fuses: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="5")]
    pub owner: ::core::option::Option<Account>,
    #[prost(string, tag="6")]
    pub label_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub label_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub label_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, boxed, tag="5")]
    pub parent: ::core::option::Option<::prost::alloc::boxed::Box<Domain>>,
    #[prost(message, repeated, tag="6")]
    pub subdomains: ::prost::alloc::vec::Vec<Domain>,
    #[prost(uint64, tag="7")]
    pub subdomain_count: u64,
    #[prost(message, optional, tag="8")]
    pub resolved_address: ::core::option::Option<Account>,
    #[prost(message, optional, boxed, tag="9")]
    pub resolver: ::core::option::Option<::prost::alloc::boxed::Box<Resolver>>,
    #[prost(message, optional, tag="10")]
    pub ttl: ::core::option::Option<BigInt>,
    #[prost(bool, tag="11")]
    pub is_migrated: bool,
    #[prost(message, optional, tag="12")]
    pub created_at: ::core::option::Option<BigInt>,
}
/// Encoded file descriptor set for the `ens.types` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x93, 0x22, 0x0a, 0x09, 0x65, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x09,
    0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x22, 0x1e, 0x0a, 0x06, 0x42, 0x69, 0x67,
    0x49, 0x6e, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x22, 0x0a, 0x0a, 0x42, 0x69, 0x67,
    0x44, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xb9, 0x02,
    0x0a, 0x0c, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0e,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x2b,
    0x0a, 0x07, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61,
    0x69, 0x6e, 0x52, 0x07, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x3e, 0x0a, 0x11, 0x72,
    0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x10, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61, 0x74, 0x65, 0x12, 0x32, 0x0a, 0x0b, 0x65,
    0x78, 0x70, 0x69, 0x72, 0x79, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67,
    0x49, 0x6e, 0x74, 0x52, 0x0a, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79, 0x44, 0x61, 0x74, 0x65, 0x12,
    0x25, 0x0a, 0x04, 0x63, 0x6f, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74,
    0x52, 0x04, 0x63, 0x6f, 0x73, 0x74, 0x12, 0x32, 0x0a, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65, 0x6e, 0x73,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x0a,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61,
    0x62, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x6c, 0x61, 0x62, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0xc9, 0x01, 0x0a, 0x07, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x2b, 0x0a, 0x07, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x52, 0x07, 0x64, 0x6f, 0x6d, 0x61, 0x69,
    0x6e, 0x73, 0x12, 0x42, 0x0a, 0x0f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x64, 0x6f,
    0x6d, 0x61, 0x69, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x65, 0x6e,
    0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x44,
    0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x52, 0x0e, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x44,
    0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x3d, 0x0a, 0x0d, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x9a, 0x01, 0x0a, 0x0d, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x2f, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x65, 0x6e, 0x73, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x52, 0x08,
    0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x0e, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x0d, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x49, 0x64, 0x22, 0xf2, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x29, 0x0a, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61,
    0x69, 0x6e, 0x52, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x12, 0x26, 0x0a, 0x04, 0x61, 0x64, 0x64, 0x72, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x41,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x04, 0x61, 0x64, 0x64, 0x72, 0x12, 0x21, 0x0a, 0x0c,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68, 0x12,
    0x14, 0x0a, 0x05, 0x74, 0x65, 0x78, 0x74, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05,
    0x74, 0x65, 0x78, 0x74, 0x73, 0x12, 0x30, 0x0a, 0x0a, 0x63, 0x6f, 0x69, 0x6e, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x09, 0x63, 0x6f,
    0x69, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x73, 0x22, 0xf1, 0x01, 0x0a, 0x0e, 0x57, 0x72, 0x61, 0x70,
    0x70, 0x65, 0x64, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x29, 0x0a, 0x06, 0x64, 0x6f,
    0x6d, 0x61, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x52, 0x06, 0x64,
    0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12, 0x32, 0x0a, 0x0b, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79, 0x5f,
    0x64, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x0a, 0x65,
    0x78, 0x70, 0x69, 0x72, 0x79, 0x44, 0x61, 0x74, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x66, 0x75, 0x73,
    0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x05, 0x66, 0x75, 0x73,
    0x65, 0x73, 0x12, 0x28, 0x0a, 0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a,
    0x6c, 0x61, 0x62, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x09, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0xd9, 0x03, 0x0a, 0x06,
    0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61,
    0x62, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x6c, 0x61, 0x62, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61, 0x62,
    0x65, 0x6c, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x6c,
    0x61, 0x62, 0x65, 0x6c, 0x48, 0x61, 0x73, 0x68, 0x12, 0x29, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x65,
    0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x52, 0x06, 0x70, 0x61, 0x72,
    0x65, 0x6e, 0x74, 0x12, 0x31, 0x0a, 0x0a, 0x73, 0x75, 0x62, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e,
    0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x52, 0x0a, 0x73, 0x75, 0x62, 0x64,
    0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x73, 0x75, 0x62, 0x64, 0x6f, 0x6d,
    0x61, 0x69, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0e, 0x73, 0x75, 0x62, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12,
    0x3d, 0x0a, 0x10, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65, 0x6e, 0x73, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x0f, 0x72,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x2f,
    0x0a, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x13, 0x2e, 0x65, 0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x72, 0x52, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x12,
    0x23, 0x0a, 0x03, 0x74, 0x74, 0x6c, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65,
    0x6e, 0x73, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52,
    0x03, 0x74, 0x74, 0x6c, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x6d, 0x69, 0x67, 0x72, 0x61,
    0x74, 0x65, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x4d, 0x69, 0x67,
    0x72, 0x61, 0x74, 0x65, 0x64, 0x12, 0x30, 0x0a, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x5f, 0x61, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x65, 0x6e, 0x73, 0x2e,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x42, 0x69, 0x67, 0x49, 0x6e, 0x74, 0x52, 0x09, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x4a, 0xc4, 0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x43, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x01, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00,
    0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x04, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x04, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07, 0x00, 0x09,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x08, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x08, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0b, 0x00, 0x13, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0c, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0c, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x0e, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x09,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x0f, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x10, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12, 0x03, 0x10, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x09, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x10, 0x10, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x06, 0x12, 0x03, 0x11, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x11, 0x0a, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x11, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x12, 0x02,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x12, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x12, 0x09, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x12, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x15, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x15, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x12,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x18, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x18, 0x1a, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x18, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x19, 0x02,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x19, 0x0b, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x19, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x19, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x1c, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x08,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x1e, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x16, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1f, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03,
    0x20, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x20, 0x02,
    0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x08, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x20, 0x19, 0x1a, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x23, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x24,
    0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x09, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x25, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x25, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x25, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x26, 0x02, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x02, 0x07, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x03, 0x12, 0x03, 0x27, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x27, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x27, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x11,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x28, 0x02, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x28, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x28, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x28, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05,
    0x12, 0x03, 0x29, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x29, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x29, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x29, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06,
    0x06, 0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x2a, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2a,
    0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2d, 0x00, 0x34, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x2e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2e, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x0e,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x02, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02,
    0x12, 0x03, 0x30, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x30, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x30, 0x09,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x30, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x31, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x31, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x31, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x31, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03,
    0x32, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x06, 0x12, 0x03, 0x32, 0x02,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x32, 0x0a, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x32, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x33, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x33, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x33, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x33, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x36, 0x00, 0x43, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x36, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x37, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x37, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x37, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x38, 0x02, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x38, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x02, 0x12, 0x03, 0x39, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x39, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x39, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x39, 0x16,
    0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x02, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3a, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x3a, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04,
    0x12, 0x03, 0x3b, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x3b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3b, 0x09,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3b, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x03, 0x3c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x05, 0x06, 0x12, 0x03, 0x3c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x3c, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x3c, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x06, 0x12, 0x03, 0x3d, 0x02,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x05, 0x12, 0x03, 0x3d, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3d, 0x09, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x07, 0x12, 0x03, 0x3e, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07,
    0x06, 0x12, 0x03, 0x3e, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x3e, 0x0a, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x03, 0x12, 0x03, 0x3e,
    0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x08, 0x12, 0x03, 0x3f, 0x02, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x06, 0x12, 0x03, 0x3f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x08, 0x01, 0x12, 0x03, 0x3f, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x08, 0x03, 0x12, 0x03, 0x3f, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x09, 0x12, 0x03, 0x40, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x06, 0x12,
    0x03, 0x40, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x01, 0x12, 0x03, 0x40,
    0x09, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x03, 0x12, 0x03, 0x40, 0x0f, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0a, 0x12, 0x03, 0x41, 0x02, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x41, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x41, 0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x0a, 0x03, 0x12, 0x03, 0x41, 0x15, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0b, 0x12,
    0x03, 0x42, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x42,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x42, 0x09, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x42, 0x16, 0x18, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)