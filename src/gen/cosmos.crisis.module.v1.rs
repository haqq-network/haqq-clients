// @generated
/// Module is the config object of the crisis module.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// fee_collector_name is the name of the FeeCollector ModuleAccount.
    #[prost(string, tag="1")]
    pub fee_collector_name: ::prost::alloc::string::String,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
}
include!("cosmos.crisis.module.v1.serde.rs");
// @@protoc_insertion_point(module)