// @generated
/// CommitInfo defines commit information used by the multi-store when committing
/// a version/height.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int64, tag="1")]
    pub version: i64,
    #[prost(message, repeated, tag="2")]
    pub store_infos: ::prost::alloc::vec::Vec<StoreInfo>,
}
/// StoreInfo defines store-specific commit information. It contains a reference
/// between a store name and the commit ID.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub commit_id: ::core::option::Option<CommitId>,
}
/// CommitID defines the commitment information when a specific store is
/// committed.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitId {
    #[prost(int64, tag="1")]
    pub version: i64,
    #[prost(bytes="bytes", tag="2")]
    pub hash: ::prost::bytes::Bytes,
}
/// StoreKVPair is a KVStore KVPair used for listening to state changes (Sets and Deletes)
/// It optionally includes the StoreKey for the originating KVStore and a Boolean flag to distinguish between Sets and
/// Deletes
///
/// Since: cosmos-sdk 0.43
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreKvPair {
    /// the store key for the KVStore this pair originates from
    #[prost(string, tag="1")]
    pub store_key: ::prost::alloc::string::String,
    /// true indicates a delete operation, false indicates a set operation
    #[prost(bool, tag="2")]
    pub delete: bool,
    #[prost(bytes="bytes", tag="3")]
    pub key: ::prost::bytes::Bytes,
    #[prost(bytes="bytes", tag="4")]
    pub value: ::prost::bytes::Bytes,
}
/// BlockMetadata contains all the abci event data of a block
/// the file streamer dump them into files together with the state changes.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    #[prost(message, optional, tag="1")]
    pub request_begin_block: ::core::option::Option<super::super::super::super::tendermint::abci::RequestBeginBlock>,
    #[prost(message, optional, tag="2")]
    pub response_begin_block: ::core::option::Option<super::super::super::super::tendermint::abci::ResponseBeginBlock>,
    #[prost(message, repeated, tag="3")]
    pub deliver_txs: ::prost::alloc::vec::Vec<block_metadata::DeliverTx>,
    #[prost(message, optional, tag="4")]
    pub request_end_block: ::core::option::Option<super::super::super::super::tendermint::abci::RequestEndBlock>,
    #[prost(message, optional, tag="5")]
    pub response_end_block: ::core::option::Option<super::super::super::super::tendermint::abci::ResponseEndBlock>,
    #[prost(message, optional, tag="6")]
    pub response_commit: ::core::option::Option<super::super::super::super::tendermint::abci::ResponseCommit>,
}
/// Nested message and enum types in `BlockMetadata`.
pub mod block_metadata {
    /// DeliverTx encapulate deliver tx request and response.
    #[derive(::derive_builder::Builder)]
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeliverTx {
        #[prost(message, optional, tag="1")]
        pub request: ::core::option::Option<super::super::super::super::super::tendermint::abci::RequestDeliverTx>,
        #[prost(message, optional, tag="2")]
        pub response: ::core::option::Option<super::super::super::super::super::tendermint::abci::ResponseDeliverTx>,
    }
}
include!("cosmos.base.store.v1beta1.serde.rs");
// @@protoc_insertion_point(module)