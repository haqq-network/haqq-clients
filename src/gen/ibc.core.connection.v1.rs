// @generated
// ICS03 - Connection Data Structures as defined in
// <https://github.com/cosmos/ibc/blob/master/spec/core/ics-003-connection-semantics#data-structures>

/// ConnectionEnd defines a stateful object on a chain connected to another
/// separate one.
/// NOTE: there must only be 2 defined ConnectionEnds to establish
/// a connection between two chains.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionEnd {
    /// client associated with this connection.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection.
    #[prost(message, repeated, tag="2")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration="State", tag="3")]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag="4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period that must pass before a consensus state can be used for
    /// packet-verification NOTE: delay period logic is only implemented by some
    /// clients.
    #[prost(uint64, tag="5")]
    pub delay_period: u64,
}
/// IdentifiedConnection defines a connection with additional connection
/// identifier field.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedConnection {
    /// connection identifier.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// client associated with this connection.
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    /// IBC version which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection
    #[prost(message, repeated, tag="3")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// current state of the connection end.
    #[prost(enumeration="State", tag="4")]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag="5")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// delay period associated with this connection.
    #[prost(uint64, tag="6")]
    pub delay_period: u64,
}
/// Counterparty defines the counterparty chain associated with a connection end.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counterparty {
    /// identifies the client on the counterparty chain associated with a given
    /// connection.
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// identifies the connection end on the counterparty chain associated with a
    /// given connection.
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// commitment merkle prefix of the counterparty chain.
    #[prost(message, optional, tag="3")]
    pub prefix: ::core::option::Option<super::super::commitment::v1::MerklePrefix>,
}
/// ClientPaths define all the connection paths for a client state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientPaths {
    /// list of connection paths
    #[prost(string, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConnectionPaths define all the connection paths for a given client state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionPaths {
    /// client state unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// list of connection paths
    #[prost(string, repeated, tag="2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Version defines the versioning scheme used to negotiate the IBC verison in
/// the connection handshake.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// unique version identifier
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
    /// list of features compatible with the specified identifier
    #[prost(string, repeated, tag="2")]
    pub features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of Connection parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// maximum expected time per block (in nanoseconds), used to enforce block delay. This parameter should reflect the
    /// largest amount of time that the chain might reasonably take to produce the next block under normal operating
    /// conditions. A safe choice is 3-5x the expected time per block.
    #[prost(uint64, tag="1")]
    pub max_expected_time_per_block: u64,
}
/// State defines if a connection is in one of the following states:
/// INIT, TRYOPEN, OPEN or UNINITIALIZED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A connection end has just started the opening handshake.
    Init = 1,
    /// A connection end has acknowledged the handshake step on the counterparty
    /// chain.
    Tryopen = 2,
    /// A connection end has completed the handshake.
    Open = 3,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::UninitializedUnspecified => "STATE_UNINITIALIZED_UNSPECIFIED",
            State::Init => "STATE_INIT",
            State::Tryopen => "STATE_TRYOPEN",
            State::Open => "STATE_OPEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNINITIALIZED_UNSPECIFIED" => Some(Self::UninitializedUnspecified),
            "STATE_INIT" => Some(Self::Init),
            "STATE_TRYOPEN" => Some(Self::Tryopen),
            "STATE_OPEN" => Some(Self::Open),
            _ => None,
        }
    }
}
/// GenesisState defines the ibc connection submodule's genesis state.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub connections: ::prost::alloc::vec::Vec<IdentifiedConnection>,
    #[prost(message, repeated, tag="2")]
    pub client_connection_paths: ::prost::alloc::vec::Vec<ConnectionPaths>,
    /// the sequence for the next generated connection identifier
    #[prost(uint64, tag="3")]
    pub next_connection_sequence: u64,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<Params>,
}
/// QueryConnectionRequest is the request type for the Query/Connection RPC
/// method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionRequest {
    /// connection unique identifier
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryConnectionResponse is the response type for the Query/Connection RPC
/// method. Besides the connection end, it includes a proof and the height from
/// which the proof was retrieved.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionResponse {
    /// connection associated with the request identifier
    #[prost(message, optional, tag="1")]
    pub connection: ::core::option::Option<ConnectionEnd>,
    /// merkle proof of existence
    #[prost(bytes="bytes", tag="2")]
    pub proof: ::prost::bytes::Bytes,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionsRequest is the request type for the Query/Connections RPC
/// method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionsRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryConnectionsResponse is the response type for the Query/Connections RPC
/// method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionsResponse {
    /// list of stored connections of the chain.
    #[prost(message, repeated, tag="1")]
    pub connections: ::prost::alloc::vec::Vec<IdentifiedConnection>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
    /// query block height
    #[prost(message, optional, tag="3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryClientConnectionsRequest is the request type for the
/// Query/ClientConnections RPC method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientConnectionsRequest {
    /// client identifier associated with a connection
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientConnectionsResponse is the response type for the
/// Query/ClientConnections RPC method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientConnectionsResponse {
    /// slice of all the connection paths associated with a client.
    #[prost(string, repeated, tag="1")]
    pub connection_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// merkle proof of existence
    #[prost(bytes="bytes", tag="2")]
    pub proof: ::prost::bytes::Bytes,
    /// height at which the proof was generated
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionClientStateRequest is the request type for the
/// Query/ConnectionClientState RPC method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionClientStateRequest {
    /// connection identifier
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryConnectionClientStateResponse is the response type for the
/// Query/ConnectionClientState RPC method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionClientStateResponse {
    /// client state associated with the channel
    #[prost(message, optional, tag="1")]
    pub identified_client_state: ::core::option::Option<super::super::client::v1::IdentifiedClientState>,
    /// merkle proof of existence
    #[prost(bytes="bytes", tag="2")]
    pub proof: ::prost::bytes::Bytes,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionConsensusStateRequest is the request type for the
/// Query/ConnectionConsensusState RPC method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionConsensusStateRequest {
    /// connection identifier
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub revision_number: u64,
    #[prost(uint64, tag="3")]
    pub revision_height: u64,
}
/// QueryConnectionConsensusStateResponse is the response type for the
/// Query/ConnectionConsensusState RPC method
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionConsensusStateResponse {
    /// consensus state associated with the channel
    #[prost(message, optional, tag="1")]
    pub consensus_state: ::core::option::Option<::pbjson_types::Any>,
    /// client ID associated with the consensus state
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    /// merkle proof of existence
    #[prost(bytes="bytes", tag="3")]
    pub proof: ::prost::bytes::Bytes,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionParamsRequest is the request type for the Query/ConnectionParams RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionParamsRequest {
}
/// QueryConnectionParamsResponse is the response type for the Query/ConnectionParams RPC method.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgConnectionOpenInit defines the msg sent by an account on Chain A to
/// initialize a connection with Chain B.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenInit {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub counterparty: ::core::option::Option<Counterparty>,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<Version>,
    #[prost(uint64, tag="4")]
    pub delay_period: u64,
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenInitResponse defines the Msg/ConnectionOpenInit response
/// type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenInitResponse {
}
/// MsgConnectionOpenTry defines a msg sent by a Relayer to try to open a
/// connection on Chain B.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenTry {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// Deprecated: this field is unused. Crossing hellos are no longer supported in core IBC.
    #[deprecated]
    #[prost(string, tag="2")]
    pub previous_connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub client_state: ::core::option::Option<::pbjson_types::Any>,
    #[prost(message, optional, tag="4")]
    pub counterparty: ::core::option::Option<Counterparty>,
    #[prost(uint64, tag="5")]
    pub delay_period: u64,
    #[prost(message, repeated, tag="6")]
    pub counterparty_versions: ::prost::alloc::vec::Vec<Version>,
    #[prost(message, optional, tag="7")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    /// proof of the initialization the connection on Chain A: `UNITIALIZED ->
    /// INIT`
    #[prost(bytes="bytes", tag="8")]
    pub proof_init: ::prost::bytes::Bytes,
    /// proof of client state included in message
    #[prost(bytes="bytes", tag="9")]
    pub proof_client: ::prost::bytes::Bytes,
    /// proof of client consensus state
    #[prost(bytes="bytes", tag="10")]
    pub proof_consensus: ::prost::bytes::Bytes,
    #[prost(message, optional, tag="11")]
    pub consensus_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="12")]
    pub signer: ::prost::alloc::string::String,
    /// optional proof data for host state machines that are unable to introspect their own consensus state
    #[prost(bytes="bytes", tag="13")]
    pub host_consensus_state_proof: ::prost::bytes::Bytes,
}
/// MsgConnectionOpenTryResponse defines the Msg/ConnectionOpenTry response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenTryResponse {
}
/// MsgConnectionOpenAck defines a msg sent by a Relayer to Chain A to
/// acknowledge the change of connection state to TRYOPEN on Chain B.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenAck {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub counterparty_connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<Version>,
    #[prost(message, optional, tag="4")]
    pub client_state: ::core::option::Option<::pbjson_types::Any>,
    #[prost(message, optional, tag="5")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    /// proof of the initialization the connection on Chain B: `UNITIALIZED ->
    /// TRYOPEN`
    #[prost(bytes="bytes", tag="6")]
    pub proof_try: ::prost::bytes::Bytes,
    /// proof of client state included in message
    #[prost(bytes="bytes", tag="7")]
    pub proof_client: ::prost::bytes::Bytes,
    /// proof of client consensus state
    #[prost(bytes="bytes", tag="8")]
    pub proof_consensus: ::prost::bytes::Bytes,
    #[prost(message, optional, tag="9")]
    pub consensus_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="10")]
    pub signer: ::prost::alloc::string::String,
    /// optional proof data for host state machines that are unable to introspect their own consensus state
    #[prost(bytes="bytes", tag="11")]
    pub host_consensus_state_proof: ::prost::bytes::Bytes,
}
/// MsgConnectionOpenAckResponse defines the Msg/ConnectionOpenAck response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenAckResponse {
}
/// MsgConnectionOpenConfirm defines a msg sent by a Relayer to Chain B to
/// acknowledge the change of connection state to OPEN on Chain A.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenConfirm {
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
    /// proof for the change of the connection state on Chain A: `INIT -> OPEN`
    #[prost(bytes="bytes", tag="2")]
    pub proof_ack: ::prost::bytes::Bytes,
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgConnectionOpenConfirmResponse defines the Msg/ConnectionOpenConfirm
/// response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenConfirmResponse {
}
/// MsgUpdateParams defines the sdk.Msg type to update the connection parameters.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// signer address (it may be the the address that controls the module, which defaults to x/gov unless overwritten).
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    /// params defines the connection parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the MsgUpdateParams response type.
#[derive(::derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("ibc.core.connection.v1.serde.rs");
include!("ibc.core.connection.v1.tonic.rs");
// @@protoc_insertion_point(module)