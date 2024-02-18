// @generated by protoc-gen-connect-es v1.1.3 with parameter "target=ts"
// @generated from file cosmos/tx/v1beta1/service.proto (package cosmos.tx.v1beta1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { BroadcastTxRequest, BroadcastTxResponse, GetBlockWithTxsRequest, GetBlockWithTxsResponse, GetTxRequest, GetTxResponse, GetTxsEventRequest, GetTxsEventResponse, SimulateRequest, SimulateResponse, TxDecodeAminoRequest, TxDecodeAminoResponse, TxDecodeRequest, TxDecodeResponse, TxEncodeAminoRequest, TxEncodeAminoResponse, TxEncodeRequest, TxEncodeResponse } from "./service_pb.js";
import { MethodKind } from "@bufbuild/protobuf";

/**
 * Service defines a gRPC service for interacting with transactions.
 *
 * @generated from service cosmos.tx.v1beta1.Service
 */
export const Service = {
  typeName: "cosmos.tx.v1beta1.Service",
  methods: {
    /**
     * Simulate simulates executing a transaction for estimating gas usage.
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.Simulate
     */
    simulate: {
      name: "Simulate",
      I: SimulateRequest,
      O: SimulateResponse,
      kind: MethodKind.Unary,
    },
    /**
     * GetTx fetches a tx by hash.
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.GetTx
     */
    getTx: {
      name: "GetTx",
      I: GetTxRequest,
      O: GetTxResponse,
      kind: MethodKind.Unary,
    },
    /**
     * BroadcastTx broadcast transaction.
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.BroadcastTx
     */
    broadcastTx: {
      name: "BroadcastTx",
      I: BroadcastTxRequest,
      O: BroadcastTxResponse,
      kind: MethodKind.Unary,
    },
    /**
     * GetTxsEvent fetches txs by event.
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.GetTxsEvent
     */
    getTxsEvent: {
      name: "GetTxsEvent",
      I: GetTxsEventRequest,
      O: GetTxsEventResponse,
      kind: MethodKind.Unary,
    },
    /**
     * GetBlockWithTxs fetches a block with decoded txs.
     *
     * Since: cosmos-sdk 0.45.2
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.GetBlockWithTxs
     */
    getBlockWithTxs: {
      name: "GetBlockWithTxs",
      I: GetBlockWithTxsRequest,
      O: GetBlockWithTxsResponse,
      kind: MethodKind.Unary,
    },
    /**
     * TxDecode decodes the transaction.
     *
     * Since: cosmos-sdk 0.47
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.TxDecode
     */
    txDecode: {
      name: "TxDecode",
      I: TxDecodeRequest,
      O: TxDecodeResponse,
      kind: MethodKind.Unary,
    },
    /**
     * TxEncode encodes the transaction.
     *
     * Since: cosmos-sdk 0.47
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.TxEncode
     */
    txEncode: {
      name: "TxEncode",
      I: TxEncodeRequest,
      O: TxEncodeResponse,
      kind: MethodKind.Unary,
    },
    /**
     * TxEncodeAmino encodes an Amino transaction from JSON to encoded bytes.
     *
     * Since: cosmos-sdk 0.47
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.TxEncodeAmino
     */
    txEncodeAmino: {
      name: "TxEncodeAmino",
      I: TxEncodeAminoRequest,
      O: TxEncodeAminoResponse,
      kind: MethodKind.Unary,
    },
    /**
     * TxDecodeAmino decodes an Amino transaction from encoded bytes to JSON.
     *
     * Since: cosmos-sdk 0.47
     *
     * @generated from rpc cosmos.tx.v1beta1.Service.TxDecodeAmino
     */
    txDecodeAmino: {
      name: "TxDecodeAmino",
      I: TxDecodeAminoRequest,
      O: TxDecodeAminoResponse,
      kind: MethodKind.Unary,
    },
  }
} as const;

