// @generated by protoc-gen-connect-es v1.1.3 with parameter "target=ts"
// @generated from file haqq/dao/v1/query.proto (package haqq.dao.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { QueryAllBalancesRequest, QueryAllBalancesResponse, QueryBalanceRequest, QueryBalanceResponse, QueryParamsRequest, QueryParamsResponse, QueryTotalBalanceRequest, QueryTotalBalanceResponse } from "./query_pb.js";
import { MethodKind } from "@bufbuild/protobuf";

/**
 * Query defines the gRPC querier service for dao module
 *
 * @generated from service haqq.dao.v1.Query
 */
export const Query = {
  typeName: "haqq.dao.v1.Query",
  methods: {
    /**
     * Balance queries the balance of a single coin for a single account.
     *
     * @generated from rpc haqq.dao.v1.Query.Balance
     */
    balance: {
      name: "Balance",
      I: QueryBalanceRequest,
      O: QueryBalanceResponse,
      kind: MethodKind.Unary,
    },
    /**
     * AllBalances queries the balance of all coins for a single account.
     *
     * When called from another module, this query might consume a high amount of
     * gas if the pagination field is incorrectly set.
     *
     * @generated from rpc haqq.dao.v1.Query.AllBalances
     */
    allBalances: {
      name: "AllBalances",
      I: QueryAllBalancesRequest,
      O: QueryAllBalancesResponse,
      kind: MethodKind.Unary,
    },
    /**
     * TotalBalance queries the total balance of all coins locked in the module.
     *
     * When called from another module, this query might consume a high amount of
     * gas if the pagination field is incorrectly set.
     *
     * @generated from rpc haqq.dao.v1.Query.TotalBalance
     */
    totalBalance: {
      name: "TotalBalance",
      I: QueryTotalBalanceRequest,
      O: QueryTotalBalanceResponse,
      kind: MethodKind.Unary,
    },
    /**
     * Params queries the parameters of x/dao module.
     *
     * @generated from rpc haqq.dao.v1.Query.Params
     */
    params: {
      name: "Params",
      I: QueryParamsRequest,
      O: QueryParamsResponse,
      kind: MethodKind.Unary,
    },
  }
} as const;

