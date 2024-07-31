// @generated by protoc-gen-connect-es v1.1.3 with parameter "target=ts"
// @generated from file haqq/dao/v1/tx.proto (package haqq.dao.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { MsgFund, MsgFundResponse, MsgTransferOwnership, MsgTransferOwnershipResponse } from "./tx_pb.js";
import { MethodKind } from "@bufbuild/protobuf";

/**
 * Msg defines the dao Msg service.
 *
 * @generated from service haqq.dao.v1.Msg
 */
export const Msg = {
  typeName: "haqq.dao.v1.Msg",
  methods: {
    /**
     * Fund defines a method to allow an account to directly fund the dao.
     *
     * @generated from rpc haqq.dao.v1.Msg.Fund
     */
    fund: {
      name: "Fund",
      I: MsgFund,
      O: MsgFundResponse,
      kind: MethodKind.Unary,
    },
    /**
     * TransferOwnership defines a method to allow an account to transfer the ownership of shares to another account.
     *
     * @generated from rpc haqq.dao.v1.Msg.TransferOwnership
     */
    transferOwnership: {
      name: "TransferOwnership",
      I: MsgTransferOwnership,
      O: MsgTransferOwnershipResponse,
      kind: MethodKind.Unary,
    },
  }
} as const;

