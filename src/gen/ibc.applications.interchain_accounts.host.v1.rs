// @generated
/// Params defines the set of on-chain interchain accounts parameters.
/// The following parameters may be used to disable the host submodule.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// host_enabled enables or disables the host submodule.
    #[prost(bool, tag="1")]
    pub host_enabled: bool,
    /// allow_messages defines a list of sdk message typeURLs allowed to be executed on a host chain.
    #[prost(string, repeated, tag="2")]
    pub allow_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParams defines the payload for Msg/UpdateParams
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// signer address (it may be the the address that controls the module, which defaults to x/gov unless overwritten).
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    /// params defines the 27-interchain-accounts/host parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response for Msg/UpdateParams
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("ibc.applications.interchain_accounts.host.v1.serde.rs");
include!("ibc.applications.interchain_accounts.host.v1.tonic.rs");
// @@protoc_insertion_point(module)