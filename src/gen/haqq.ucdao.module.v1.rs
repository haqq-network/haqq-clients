// @generated
/// Module is the config object of the distribution module.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// max_metadata_len defines the maximum proposal metadata length.
    /// Defaults to 5000 if not explicitly set.
    #[prost(uint64, tag="1")]
    pub max_metadata_len: u64,
    /// authority defines the custom module authority. If not set, defaults to the ucdao module.
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
}
include!("haqq.ucdao.module.v1.serde.rs");
// @@protoc_insertion_point(module)