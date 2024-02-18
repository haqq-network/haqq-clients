// @generated by protoc-gen-es v1.3.1 with parameter "target=ts"
// @generated from file cosmos/auth/v1beta1/auth.proto (package cosmos.auth.v1beta1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Any, Message, proto3, protoInt64 } from "@bufbuild/protobuf";

/**
 * BaseAccount defines a base account type. It contains all the necessary fields
 * for basic account functionality. Any custom account type should extend this
 * type for additional functionality (e.g. vesting).
 *
 * @generated from message cosmos.auth.v1beta1.BaseAccount
 */
export class BaseAccount extends Message<BaseAccount> {
  /**
   * @generated from field: string address = 1;
   */
  address = "";

  /**
   * @generated from field: google.protobuf.Any pub_key = 2;
   */
  pubKey?: Any;

  /**
   * @generated from field: uint64 account_number = 3;
   */
  accountNumber = protoInt64.zero;

  /**
   * @generated from field: uint64 sequence = 4;
   */
  sequence = protoInt64.zero;

  constructor(data?: PartialMessage<BaseAccount>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "cosmos.auth.v1beta1.BaseAccount";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "address", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "pub_key", kind: "message", T: Any },
    { no: 3, name: "account_number", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 4, name: "sequence", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): BaseAccount {
    return new BaseAccount().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): BaseAccount {
    return new BaseAccount().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): BaseAccount {
    return new BaseAccount().fromJsonString(jsonString, options);
  }

  static equals(a: BaseAccount | PlainMessage<BaseAccount> | undefined, b: BaseAccount | PlainMessage<BaseAccount> | undefined): boolean {
    return proto3.util.equals(BaseAccount, a, b);
  }
}

/**
 * ModuleAccount defines an account for modules that holds coins on a pool.
 *
 * @generated from message cosmos.auth.v1beta1.ModuleAccount
 */
export class ModuleAccount extends Message<ModuleAccount> {
  /**
   * @generated from field: cosmos.auth.v1beta1.BaseAccount base_account = 1;
   */
  baseAccount?: BaseAccount;

  /**
   * @generated from field: string name = 2;
   */
  name = "";

  /**
   * @generated from field: repeated string permissions = 3;
   */
  permissions: string[] = [];

  constructor(data?: PartialMessage<ModuleAccount>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "cosmos.auth.v1beta1.ModuleAccount";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "base_account", kind: "message", T: BaseAccount },
    { no: 2, name: "name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "permissions", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ModuleAccount {
    return new ModuleAccount().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ModuleAccount {
    return new ModuleAccount().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ModuleAccount {
    return new ModuleAccount().fromJsonString(jsonString, options);
  }

  static equals(a: ModuleAccount | PlainMessage<ModuleAccount> | undefined, b: ModuleAccount | PlainMessage<ModuleAccount> | undefined): boolean {
    return proto3.util.equals(ModuleAccount, a, b);
  }
}

/**
 * ModuleCredential represents a unclaimable pubkey for base accounts controlled by modules.
 *
 * Since: cosmos-sdk 0.47
 *
 * @generated from message cosmos.auth.v1beta1.ModuleCredential
 */
export class ModuleCredential extends Message<ModuleCredential> {
  /**
   * module_name is the name of the module used for address derivation (passed into address.Module).
   *
   * @generated from field: string module_name = 1;
   */
  moduleName = "";

  /**
   * derivation_keys is for deriving a module account address (passed into address.Module)
   * adding more keys creates sub-account addresses (passed into address.Derive)
   *
   * @generated from field: repeated bytes derivation_keys = 2;
   */
  derivationKeys: Uint8Array[] = [];

  constructor(data?: PartialMessage<ModuleCredential>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "cosmos.auth.v1beta1.ModuleCredential";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "module_name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "derivation_keys", kind: "scalar", T: 12 /* ScalarType.BYTES */, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): ModuleCredential {
    return new ModuleCredential().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): ModuleCredential {
    return new ModuleCredential().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): ModuleCredential {
    return new ModuleCredential().fromJsonString(jsonString, options);
  }

  static equals(a: ModuleCredential | PlainMessage<ModuleCredential> | undefined, b: ModuleCredential | PlainMessage<ModuleCredential> | undefined): boolean {
    return proto3.util.equals(ModuleCredential, a, b);
  }
}

/**
 * Params defines the parameters for the auth module.
 *
 * @generated from message cosmos.auth.v1beta1.Params
 */
export class Params extends Message<Params> {
  /**
   * @generated from field: uint64 max_memo_characters = 1;
   */
  maxMemoCharacters = protoInt64.zero;

  /**
   * @generated from field: uint64 tx_sig_limit = 2;
   */
  txSigLimit = protoInt64.zero;

  /**
   * @generated from field: uint64 tx_size_cost_per_byte = 3;
   */
  txSizeCostPerByte = protoInt64.zero;

  /**
   * @generated from field: uint64 sig_verify_cost_ed25519 = 4;
   */
  sigVerifyCostEd25519 = protoInt64.zero;

  /**
   * @generated from field: uint64 sig_verify_cost_secp256k1 = 5;
   */
  sigVerifyCostSecp256k1 = protoInt64.zero;

  constructor(data?: PartialMessage<Params>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "cosmos.auth.v1beta1.Params";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "max_memo_characters", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 2, name: "tx_sig_limit", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 3, name: "tx_size_cost_per_byte", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 4, name: "sig_verify_cost_ed25519", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
    { no: 5, name: "sig_verify_cost_secp256k1", kind: "scalar", T: 4 /* ScalarType.UINT64 */ },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Params {
    return new Params().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Params {
    return new Params().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Params {
    return new Params().fromJsonString(jsonString, options);
  }

  static equals(a: Params | PlainMessage<Params> | undefined, b: Params | PlainMessage<Params> | undefined): boolean {
    return proto3.util.equals(Params, a, b);
  }
}

