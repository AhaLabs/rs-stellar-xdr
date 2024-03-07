#![cfg(feature = "curr")]
#![cfg(all(feature = "schemars", feature = "serde", feature = "alloc"))]

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::TransactionEnvelope;

#[allow(clippy::too_many_lines)]
#[test]
fn test_serde_tx_schema() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schemars::schema_for!(TransactionEnvelope);
    let s = serde_json::to_string_pretty(&schema)?;
    println!("{s}");
    assert_eq!(
        s,
        r##"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "TransactionEnvelope",
  "description": "TransactionEnvelope is an XDR Union defines as:\n\n```text union TransactionEnvelope switch (EnvelopeType type) { case ENVELOPE_TYPE_TX_V0: TransactionV0Envelope v0; case ENVELOPE_TYPE_TX: TransactionV1Envelope v1; case ENVELOPE_TYPE_TX_FEE_BUMP: FeeBumpTransactionEnvelope feeBump; }; ```",
  "oneOf": [
    {
      "type": "object",
      "required": [
        "tx_v0"
      ],
      "properties": {
        "tx_v0": {
          "$ref": "#/definitions/TransactionV0Envelope"
        }
      },
      "additionalProperties": false
    },
    {
      "type": "object",
      "required": [
        "tx"
      ],
      "properties": {
        "tx": {
          "$ref": "#/definitions/TransactionV1Envelope"
        }
      },
      "additionalProperties": false
    },
    {
      "type": "object",
      "required": [
        "tx_fee_bump"
      ],
      "properties": {
        "tx_fee_bump": {
          "$ref": "#/definitions/FeeBumpTransactionEnvelope"
        }
      },
      "additionalProperties": false
    }
  ],
  "definitions": {
    "AccountId": {
      "description": "AccountId is an XDR Typedef defines as:\n\n```text typedef PublicKey AccountID; ```",
      "allOf": [
        {
          "$ref": "#/definitions/PublicKey"
        }
      ]
    },
    "AllowTrustOp": {
      "description": "AllowTrustOp is an XDR Struct defines as:\n\n```text struct AllowTrustOp { AccountID trustor; AssetCode asset;\n\n// One of 0, AUTHORIZED_FLAG, or AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG uint32 authorize; }; ```",
      "type": "object",
      "required": [
        "asset",
        "authorize",
        "trustor"
      ],
      "properties": {
        "asset": {
          "$ref": "#/definitions/AssetCode"
        },
        "authorize": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "trustor": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "AlphaNum12": {
      "description": "AlphaNum12 is an XDR Struct defines as:\n\n```text struct AlphaNum12 { AssetCode12 assetCode; AccountID issuer; }; ```",
      "type": "object",
      "required": [
        "asset_code",
        "issuer"
      ],
      "properties": {
        "asset_code": {
          "$ref": "#/definitions/AssetCode12"
        },
        "issuer": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "AlphaNum4": {
      "description": "AlphaNum4 is an XDR Struct defines as:\n\n```text struct AlphaNum4 { AssetCode4 assetCode; AccountID issuer; }; ```",
      "type": "object",
      "required": [
        "asset_code",
        "issuer"
      ],
      "properties": {
        "asset_code": {
          "$ref": "#/definitions/AssetCode4"
        },
        "issuer": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "Asset": {
      "description": "Asset is an XDR Union defines as:\n\n```text union Asset switch (AssetType type) { case ASSET_TYPE_NATIVE: // Not credit void;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM4: AlphaNum4 alphaNum4;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM12: AlphaNum12 alphaNum12;\n\n// add other asset types here in the future }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "native"
          ]
        },
        {
          "type": "object",
          "required": [
            "credit_alphanum4"
          ],
          "properties": {
            "credit_alphanum4": {
              "$ref": "#/definitions/AlphaNum4"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "credit_alphanum12"
          ],
          "properties": {
            "credit_alphanum12": {
              "$ref": "#/definitions/AlphaNum12"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "AssetCode": {
      "description": "AssetCode is an XDR Union defines as:\n\n```text union AssetCode switch (AssetType type) { case ASSET_TYPE_CREDIT_ALPHANUM4: AssetCode4 assetCode4;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM12: AssetCode12 assetCode12;\n\n// add other asset types here in the future }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "CreditAlphanum4"
          ],
          "properties": {
            "CreditAlphanum4": {
              "$ref": "#/definitions/AssetCode4"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "CreditAlphanum12"
          ],
          "properties": {
            "CreditAlphanum12": {
              "$ref": "#/definitions/AssetCode12"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "AssetCode12": {
      "description": "AssetCode12 is an XDR Typedef defines as:\n\n```text typedef opaque AssetCode12[12]; ```",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      },
      "maxItems": 12,
      "minItems": 12
    },
    "AssetCode4": {
      "description": "AssetCode4 is an XDR Typedef defines as:\n\n```text typedef opaque AssetCode4[4]; ```",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      },
      "maxItems": 4,
      "minItems": 4
    },
    "BeginSponsoringFutureReservesOp": {
      "description": "BeginSponsoringFutureReservesOp is an XDR Struct defines as:\n\n```text struct BeginSponsoringFutureReservesOp { AccountID sponsoredID; }; ```",
      "type": "object",
      "required": [
        "sponsored_id"
      ],
      "properties": {
        "sponsored_id": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "BumpSequenceOp": {
      "description": "BumpSequenceOp is an XDR Struct defines as:\n\n```text struct BumpSequenceOp { SequenceNumber bumpTo; }; ```",
      "type": "object",
      "required": [
        "bump_to"
      ],
      "properties": {
        "bump_to": {
          "$ref": "#/definitions/SequenceNumber"
        }
      }
    },
    "BytesM_for_4294967295": {
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      }
    },
    "BytesM_for_64": {
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      }
    },
    "ChangeTrustAsset": {
      "description": "ChangeTrustAsset is an XDR Union defines as:\n\n```text union ChangeTrustAsset switch (AssetType type) { case ASSET_TYPE_NATIVE: // Not credit void;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM4: AlphaNum4 alphaNum4;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM12: AlphaNum12 alphaNum12;\n\ncase ASSET_TYPE_POOL_SHARE: LiquidityPoolParameters liquidityPool;\n\n// add other asset types here in the future }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "native"
          ]
        },
        {
          "type": "object",
          "required": [
            "credit_alphanum4"
          ],
          "properties": {
            "credit_alphanum4": {
              "$ref": "#/definitions/AlphaNum4"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "credit_alphanum12"
          ],
          "properties": {
            "credit_alphanum12": {
              "$ref": "#/definitions/AlphaNum12"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "pool_share"
          ],
          "properties": {
            "pool_share": {
              "$ref": "#/definitions/LiquidityPoolParameters"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ChangeTrustOp": {
      "description": "ChangeTrustOp is an XDR Struct defines as:\n\n```text struct ChangeTrustOp { ChangeTrustAsset line;\n\n// if limit is set to 0, deletes the trust line int64 limit; }; ```",
      "type": "object",
      "required": [
        "limit",
        "line"
      ],
      "properties": {
        "limit": {
          "type": "integer",
          "format": "int64"
        },
        "line": {
          "$ref": "#/definitions/ChangeTrustAsset"
        }
      }
    },
    "ClaimClaimableBalanceOp": {
      "description": "ClaimClaimableBalanceOp is an XDR Struct defines as:\n\n```text struct ClaimClaimableBalanceOp { ClaimableBalanceID balanceID; }; ```",
      "type": "object",
      "required": [
        "balance_id"
      ],
      "properties": {
        "balance_id": {
          "$ref": "#/definitions/ClaimableBalanceId"
        }
      }
    },
    "ClaimPredicate": {
      "description": "ClaimPredicate is an XDR Union defines as:\n\n```text union ClaimPredicate switch (ClaimPredicateType type) { case CLAIM_PREDICATE_UNCONDITIONAL: void; case CLAIM_PREDICATE_AND: ClaimPredicate andPredicates<2>; case CLAIM_PREDICATE_OR: ClaimPredicate orPredicates<2>; case CLAIM_PREDICATE_NOT: ClaimPredicate* notPredicate; case CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME: int64 absBefore; // Predicate will be true if closeTime < absBefore case CLAIM_PREDICATE_BEFORE_RELATIVE_TIME: int64 relBefore; // Seconds since closeTime of the ledger in which the // ClaimableBalanceEntry was created }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "unconditional"
          ]
        },
        {
          "type": "object",
          "required": [
            "and"
          ],
          "properties": {
            "and": {
              "$ref": "#/definitions/VecM_for_ClaimPredicate_and_2"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "or"
          ],
          "properties": {
            "or": {
              "$ref": "#/definitions/VecM_for_ClaimPredicate_and_2"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "not"
          ],
          "properties": {
            "not": {
              "anyOf": [
                {
                  "$ref": "#/definitions/ClaimPredicate"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "before_absolute_time"
          ],
          "properties": {
            "before_absolute_time": {
              "type": "integer",
              "format": "int64"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "before_relative_time"
          ],
          "properties": {
            "before_relative_time": {
              "type": "integer",
              "format": "int64"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ClaimableBalanceId": {
      "description": "ClaimableBalanceId is an XDR Union defines as:\n\n```text union ClaimableBalanceID switch (ClaimableBalanceIDType type) { case CLAIMABLE_BALANCE_ID_TYPE_V0: Hash v0; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "claimable_balance_id_type_v0"
          ],
          "properties": {
            "claimable_balance_id_type_v0": {
              "$ref": "#/definitions/Hash"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "Claimant": {
      "description": "Claimant is an XDR Union defines as:\n\n```text union Claimant switch (ClaimantType type) { case CLAIMANT_TYPE_V0: struct { AccountID destination;    // The account that can use this condition ClaimPredicate predicate; // Claimable if predicate is true } v0; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "claimant_type_v0"
          ],
          "properties": {
            "claimant_type_v0": {
              "$ref": "#/definitions/ClaimantV0"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ClaimantV0": {
      "description": "ClaimantV0 is an XDR NestedStruct defines as:\n\n```text struct { AccountID destination;    // The account that can use this condition ClaimPredicate predicate; // Claimable if predicate is true } ```",
      "type": "object",
      "required": [
        "destination",
        "predicate"
      ],
      "properties": {
        "destination": {
          "$ref": "#/definitions/AccountId"
        },
        "predicate": {
          "$ref": "#/definitions/ClaimPredicate"
        }
      }
    },
    "ClawbackClaimableBalanceOp": {
      "description": "ClawbackClaimableBalanceOp is an XDR Struct defines as:\n\n```text struct ClawbackClaimableBalanceOp { ClaimableBalanceID balanceID; }; ```",
      "type": "object",
      "required": [
        "balance_id"
      ],
      "properties": {
        "balance_id": {
          "$ref": "#/definitions/ClaimableBalanceId"
        }
      }
    },
    "ClawbackOp": {
      "description": "ClawbackOp is an XDR Struct defines as:\n\n```text struct ClawbackOp { Asset asset; MuxedAccount from; int64 amount; }; ```",
      "type": "object",
      "required": [
        "amount",
        "asset",
        "from"
      ],
      "properties": {
        "amount": {
          "type": "integer",
          "format": "int64"
        },
        "asset": {
          "$ref": "#/definitions/Asset"
        },
        "from": {
          "$ref": "#/definitions/MuxedAccount"
        }
      }
    },
    "ConfigSettingId": {
      "description": "ConfigSettingId is an XDR Enum defines as:\n\n```text enum ConfigSettingID { CONFIG_SETTING_CONTRACT_MAX_SIZE_BYTES = 0, CONFIG_SETTING_CONTRACT_COMPUTE_V0 = 1, CONFIG_SETTING_CONTRACT_LEDGER_COST_V0 = 2, CONFIG_SETTING_CONTRACT_HISTORICAL_DATA_V0 = 3, CONFIG_SETTING_CONTRACT_EVENTS_V0 = 4, CONFIG_SETTING_CONTRACT_BANDWIDTH_V0 = 5, CONFIG_SETTING_CONTRACT_COST_PARAMS_CPU_INSTRUCTIONS = 6, CONFIG_SETTING_CONTRACT_COST_PARAMS_MEMORY_BYTES = 7, CONFIG_SETTING_CONTRACT_DATA_KEY_SIZE_BYTES = 8, CONFIG_SETTING_CONTRACT_DATA_ENTRY_SIZE_BYTES = 9, CONFIG_SETTING_STATE_ARCHIVAL = 10, CONFIG_SETTING_CONTRACT_EXECUTION_LANES = 11, CONFIG_SETTING_BUCKETLIST_SIZE_WINDOW = 12, CONFIG_SETTING_EVICTION_ITERATOR = 13 }; ```",
      "type": "string",
      "enum": [
        "contract_max_size_bytes",
        "contract_compute_v0",
        "contract_ledger_cost_v0",
        "contract_historical_data_v0",
        "contract_events_v0",
        "contract_bandwidth_v0",
        "contract_cost_params_cpu_instructions",
        "contract_cost_params_memory_bytes",
        "contract_data_key_size_bytes",
        "contract_data_entry_size_bytes",
        "state_archival",
        "contract_execution_lanes",
        "bucketlist_size_window",
        "eviction_iterator"
      ]
    },
    "ContractDataDurability": {
      "description": "ContractDataDurability is an XDR Enum defines as:\n\n```text enum ContractDataDurability { TEMPORARY = 0, PERSISTENT = 1 }; ```",
      "type": "string",
      "enum": [
        "temporary",
        "persistent"
      ]
    },
    "ContractExecutable": {
      "description": "ContractExecutable is an XDR Union defines as:\n\n```text union ContractExecutable switch (ContractExecutableType type) { case CONTRACT_EXECUTABLE_WASM: Hash wasm_hash; case CONTRACT_EXECUTABLE_STELLAR_ASSET: void; }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "stellar_asset"
          ]
        },
        {
          "type": "object",
          "required": [
            "wasm"
          ],
          "properties": {
            "wasm": {
              "$ref": "#/definitions/Hash"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ContractIdPreimage": {
      "description": "ContractIdPreimage is an XDR Union defines as:\n\n```text union ContractIDPreimage switch (ContractIDPreimageType type) { case CONTRACT_ID_PREIMAGE_FROM_ADDRESS: struct { SCAddress address; uint256 salt; } fromAddress; case CONTRACT_ID_PREIMAGE_FROM_ASSET: Asset fromAsset; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "address"
          ],
          "properties": {
            "address": {
              "$ref": "#/definitions/ContractIdPreimageFromAddress"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "asset"
          ],
          "properties": {
            "asset": {
              "$ref": "#/definitions/Asset"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ContractIdPreimageFromAddress": {
      "description": "ContractIdPreimageFromAddress is an XDR NestedStruct defines as:\n\n```text struct { SCAddress address; uint256 salt; } ```",
      "type": "object",
      "required": [
        "address",
        "salt"
      ],
      "properties": {
        "address": {
          "$ref": "#/definitions/ScAddress"
        },
        "salt": {
          "$ref": "#/definitions/Uint256"
        }
      }
    },
    "CreateAccountOp": {
      "description": "CreateAccountOp is an XDR Struct defines as:\n\n```text struct CreateAccountOp { AccountID destination; // account to create int64 startingBalance; // amount they end up with }; ```",
      "type": "object",
      "required": [
        "destination",
        "starting_balance"
      ],
      "properties": {
        "destination": {
          "$ref": "#/definitions/AccountId"
        },
        "starting_balance": {
          "type": "integer",
          "format": "int64"
        }
      }
    },
    "CreateClaimableBalanceOp": {
      "description": "CreateClaimableBalanceOp is an XDR Struct defines as:\n\n```text struct CreateClaimableBalanceOp { Asset asset; int64 amount; Claimant claimants<10>; }; ```",
      "type": "object",
      "required": [
        "amount",
        "asset",
        "claimants"
      ],
      "properties": {
        "amount": {
          "type": "integer",
          "format": "int64"
        },
        "asset": {
          "$ref": "#/definitions/Asset"
        },
        "claimants": {
          "$ref": "#/definitions/VecM_for_Claimant_and_10"
        }
      }
    },
    "CreateContractArgs": {
      "description": "CreateContractArgs is an XDR Struct defines as:\n\n```text struct CreateContractArgs { ContractIDPreimage contractIDPreimage; ContractExecutable executable; }; ```",
      "type": "object",
      "required": [
        "contract_id_preimage",
        "executable"
      ],
      "properties": {
        "contract_id_preimage": {
          "$ref": "#/definitions/ContractIdPreimage"
        },
        "executable": {
          "$ref": "#/definitions/ContractExecutable"
        }
      }
    },
    "CreatePassiveSellOfferOp": {
      "description": "CreatePassiveSellOfferOp is an XDR Struct defines as:\n\n```text struct CreatePassiveSellOfferOp { Asset selling; // A Asset buying;  // B int64 amount;  // amount taker gets Price price;   // cost of A in terms of B }; ```",
      "type": "object",
      "required": [
        "amount",
        "buying",
        "price",
        "selling"
      ],
      "properties": {
        "amount": {
          "type": "integer",
          "format": "int64"
        },
        "buying": {
          "$ref": "#/definitions/Asset"
        },
        "price": {
          "$ref": "#/definitions/Price"
        },
        "selling": {
          "$ref": "#/definitions/Asset"
        }
      }
    },
    "DataValue": {
      "description": "DataValue is an XDR Typedef defines as:\n\n```text typedef opaque DataValue<64>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/BytesM_for_64"
        }
      ]
    },
    "DecoratedSignature": {
      "description": "DecoratedSignature is an XDR Struct defines as:\n\n```text struct DecoratedSignature { SignatureHint hint;  // last 4 bytes of the public key, used as a hint Signature signature; // actual signature }; ```",
      "type": "object",
      "required": [
        "hint",
        "signature"
      ],
      "properties": {
        "hint": {
          "$ref": "#/definitions/SignatureHint"
        },
        "signature": {
          "$ref": "#/definitions/Signature"
        }
      }
    },
    "Duration": {
      "description": "Duration is an XDR Typedef defines as:\n\n```text typedef uint64 Duration; ```",
      "type": "integer",
      "format": "uint64",
      "minimum": 0.0
    },
    "ExtendFootprintTtlOp": {
      "description": "ExtendFootprintTtlOp is an XDR Struct defines as:\n\n```text struct ExtendFootprintTTLOp { ExtensionPoint ext; uint32 extendTo; }; ```",
      "type": "object",
      "required": [
        "ext",
        "extend_to"
      ],
      "properties": {
        "ext": {
          "$ref": "#/definitions/ExtensionPoint"
        },
        "extend_to": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        }
      }
    },
    "ExtensionPoint": {
      "description": "ExtensionPoint is an XDR Union defines as:\n\n```text union ExtensionPoint switch (int v) { case 0: void; }; ```",
      "type": "string",
      "enum": [
        "v0"
      ]
    },
    "FeeBumpTransaction": {
      "description": "FeeBumpTransaction is an XDR Struct defines as:\n\n```text struct FeeBumpTransaction { MuxedAccount feeSource; int64 fee; union switch (EnvelopeType type) { case ENVELOPE_TYPE_TX: TransactionV1Envelope v1; } innerTx; union switch (int v) { case 0: void; } ext; }; ```",
      "type": "object",
      "required": [
        "ext",
        "fee",
        "fee_source",
        "inner_tx"
      ],
      "properties": {
        "ext": {
          "$ref": "#/definitions/FeeBumpTransactionExt"
        },
        "fee": {
          "type": "integer",
          "format": "int64"
        },
        "fee_source": {
          "$ref": "#/definitions/MuxedAccount"
        },
        "inner_tx": {
          "$ref": "#/definitions/FeeBumpTransactionInnerTx"
        }
      }
    },
    "FeeBumpTransactionEnvelope": {
      "description": "FeeBumpTransactionEnvelope is an XDR Struct defines as:\n\n```text struct FeeBumpTransactionEnvelope { FeeBumpTransaction tx; /* Each decorated signature is a signature over the SHA256 hash of * a TransactionSignaturePayload */ DecoratedSignature signatures<20>; }; ```",
      "type": "object",
      "required": [
        "signatures",
        "tx"
      ],
      "properties": {
        "signatures": {
          "$ref": "#/definitions/VecM_for_DecoratedSignature_and_20"
        },
        "tx": {
          "$ref": "#/definitions/FeeBumpTransaction"
        }
      }
    },
    "FeeBumpTransactionExt": {
      "description": "FeeBumpTransactionExt is an XDR NestedUnion defines as:\n\n```text union switch (int v) { case 0: void; } ```",
      "type": "string",
      "enum": [
        "v0"
      ]
    },
    "FeeBumpTransactionInnerTx": {
      "description": "FeeBumpTransactionInnerTx is an XDR NestedUnion defines as:\n\n```text union switch (EnvelopeType type) { case ENVELOPE_TYPE_TX: TransactionV1Envelope v1; } ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "tx"
          ],
          "properties": {
            "tx": {
              "$ref": "#/definitions/TransactionV1Envelope"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "Hash": {
      "description": "Hash is an XDR Typedef defines as:\n\n```text typedef opaque Hash[32]; ```",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      },
      "maxItems": 32,
      "minItems": 32
    },
    "HostFunction": {
      "description": "HostFunction is an XDR Union defines as:\n\n```text union HostFunction switch (HostFunctionType type) { case HOST_FUNCTION_TYPE_INVOKE_CONTRACT: InvokeContractArgs invokeContract; case HOST_FUNCTION_TYPE_CREATE_CONTRACT: CreateContractArgs createContract; case HOST_FUNCTION_TYPE_UPLOAD_CONTRACT_WASM: opaque wasm<>; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "invoke_contract"
          ],
          "properties": {
            "invoke_contract": {
              "$ref": "#/definitions/InvokeContractArgs"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "create_contract"
          ],
          "properties": {
            "create_contract": {
              "$ref": "#/definitions/CreateContractArgs"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "upload_contract_wasm"
          ],
          "properties": {
            "upload_contract_wasm": {
              "$ref": "#/definitions/BytesM_for_4294967295"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "Int128Parts": {
      "description": "Int128Parts is an XDR Struct defines as:\n\n```text struct Int128Parts { int64 hi; uint64 lo; }; ```",
      "type": "object",
      "required": [
        "hi",
        "lo"
      ],
      "properties": {
        "hi": {
          "type": "integer",
          "format": "int64"
        },
        "lo": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        }
      }
    },
    "Int256Parts": {
      "description": "Int256Parts is an XDR Struct defines as:\n\n```text struct Int256Parts { int64 hi_hi; uint64 hi_lo; uint64 lo_hi; uint64 lo_lo; }; ```",
      "type": "object",
      "required": [
        "hi_hi",
        "hi_lo",
        "lo_hi",
        "lo_lo"
      ],
      "properties": {
        "hi_hi": {
          "type": "integer",
          "format": "int64"
        },
        "hi_lo": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "lo_hi": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "lo_lo": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        }
      }
    },
    "InvokeContractArgs": {
      "description": "InvokeContractArgs is an XDR Struct defines as:\n\n```text struct InvokeContractArgs { SCAddress contractAddress; SCSymbol functionName; SCVal args<>; }; ```",
      "type": "object",
      "required": [
        "args",
        "contract_address",
        "function_name"
      ],
      "properties": {
        "args": {
          "$ref": "#/definitions/VecM_for_ScVal_and_4294967295"
        },
        "contract_address": {
          "$ref": "#/definitions/ScAddress"
        },
        "function_name": {
          "$ref": "#/definitions/ScSymbol"
        }
      }
    },
    "InvokeHostFunctionOp": {
      "description": "InvokeHostFunctionOp is an XDR Struct defines as:\n\n```text struct InvokeHostFunctionOp { // Host function to invoke. HostFunction hostFunction; // Per-address authorizations for this host function. SorobanAuthorizationEntry auth<>; }; ```",
      "type": "object",
      "required": [
        "auth",
        "host_function"
      ],
      "properties": {
        "auth": {
          "$ref": "#/definitions/VecM_for_SorobanAuthorizationEntry_and_4294967295"
        },
        "host_function": {
          "$ref": "#/definitions/HostFunction"
        }
      }
    },
    "LedgerBounds": {
      "description": "LedgerBounds is an XDR Struct defines as:\n\n```text struct LedgerBounds { uint32 minLedger; uint32 maxLedger; // 0 here means no maxLedger }; ```",
      "type": "object",
      "required": [
        "max_ledger",
        "min_ledger"
      ],
      "properties": {
        "max_ledger": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "min_ledger": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        }
      }
    },
    "LedgerFootprint": {
      "description": "LedgerFootprint is an XDR Struct defines as:\n\n```text struct LedgerFootprint { LedgerKey readOnly<>; LedgerKey readWrite<>; }; ```",
      "type": "object",
      "required": [
        "read_only",
        "read_write"
      ],
      "properties": {
        "read_only": {
          "$ref": "#/definitions/VecM_for_LedgerKey_and_4294967295"
        },
        "read_write": {
          "$ref": "#/definitions/VecM_for_LedgerKey_and_4294967295"
        }
      }
    },
    "LedgerKey": {
      "description": "LedgerKey is an XDR Union defines as:\n\n```text union LedgerKey switch (LedgerEntryType type) { case ACCOUNT: struct { AccountID accountID; } account;\n\ncase TRUSTLINE: struct { AccountID accountID; TrustLineAsset asset; } trustLine;\n\ncase OFFER: struct { AccountID sellerID; int64 offerID; } offer;\n\ncase DATA: struct { AccountID accountID; string64 dataName; } data;\n\ncase CLAIMABLE_BALANCE: struct { ClaimableBalanceID balanceID; } claimableBalance;\n\ncase LIQUIDITY_POOL: struct { PoolID liquidityPoolID; } liquidityPool; case CONTRACT_DATA: struct { SCAddress contract; SCVal key; ContractDataDurability durability; } contractData; case CONTRACT_CODE: struct { Hash hash; } contractCode; case CONFIG_SETTING: struct { ConfigSettingID configSettingID; } configSetting; case TTL: struct { // Hash of the LedgerKey that is associated with this TTLEntry Hash keyHash; } ttl; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "account"
          ],
          "properties": {
            "account": {
              "$ref": "#/definitions/LedgerKeyAccount"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "trustline"
          ],
          "properties": {
            "trustline": {
              "$ref": "#/definitions/LedgerKeyTrustLine"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "offer"
          ],
          "properties": {
            "offer": {
              "$ref": "#/definitions/LedgerKeyOffer"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "data"
          ],
          "properties": {
            "data": {
              "$ref": "#/definitions/LedgerKeyData"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "claimable_balance"
          ],
          "properties": {
            "claimable_balance": {
              "$ref": "#/definitions/LedgerKeyClaimableBalance"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "liquidity_pool"
          ],
          "properties": {
            "liquidity_pool": {
              "$ref": "#/definitions/LedgerKeyLiquidityPool"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "contract_data"
          ],
          "properties": {
            "contract_data": {
              "$ref": "#/definitions/LedgerKeyContractData"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "contract_code"
          ],
          "properties": {
            "contract_code": {
              "$ref": "#/definitions/LedgerKeyContractCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "config_setting"
          ],
          "properties": {
            "config_setting": {
              "$ref": "#/definitions/LedgerKeyConfigSetting"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "ttl"
          ],
          "properties": {
            "ttl": {
              "$ref": "#/definitions/LedgerKeyTtl"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "LedgerKeyAccount": {
      "description": "LedgerKeyAccount is an XDR NestedStruct defines as:\n\n```text struct { AccountID accountID; } ```",
      "type": "object",
      "required": [
        "account_id"
      ],
      "properties": {
        "account_id": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "LedgerKeyClaimableBalance": {
      "description": "LedgerKeyClaimableBalance is an XDR NestedStruct defines as:\n\n```text struct { ClaimableBalanceID balanceID; } ```",
      "type": "object",
      "required": [
        "balance_id"
      ],
      "properties": {
        "balance_id": {
          "$ref": "#/definitions/ClaimableBalanceId"
        }
      }
    },
    "LedgerKeyConfigSetting": {
      "description": "LedgerKeyConfigSetting is an XDR NestedStruct defines as:\n\n```text struct { ConfigSettingID configSettingID; } ```",
      "type": "object",
      "required": [
        "config_setting_id"
      ],
      "properties": {
        "config_setting_id": {
          "$ref": "#/definitions/ConfigSettingId"
        }
      }
    },
    "LedgerKeyContractCode": {
      "description": "LedgerKeyContractCode is an XDR NestedStruct defines as:\n\n```text struct { Hash hash; } ```",
      "type": "object",
      "required": [
        "hash"
      ],
      "properties": {
        "hash": {
          "$ref": "#/definitions/Hash"
        }
      }
    },
    "LedgerKeyContractData": {
      "description": "LedgerKeyContractData is an XDR NestedStruct defines as:\n\n```text struct { SCAddress contract; SCVal key; ContractDataDurability durability; } ```",
      "type": "object",
      "required": [
        "contract",
        "durability",
        "key"
      ],
      "properties": {
        "contract": {
          "$ref": "#/definitions/ScAddress"
        },
        "durability": {
          "$ref": "#/definitions/ContractDataDurability"
        },
        "key": {
          "$ref": "#/definitions/ScVal"
        }
      }
    },
    "LedgerKeyData": {
      "description": "LedgerKeyData is an XDR NestedStruct defines as:\n\n```text struct { AccountID accountID; string64 dataName; } ```",
      "type": "object",
      "required": [
        "account_id",
        "data_name"
      ],
      "properties": {
        "account_id": {
          "$ref": "#/definitions/AccountId"
        },
        "data_name": {
          "$ref": "#/definitions/String64"
        }
      }
    },
    "LedgerKeyLiquidityPool": {
      "description": "LedgerKeyLiquidityPool is an XDR NestedStruct defines as:\n\n```text struct { PoolID liquidityPoolID; } ```",
      "type": "object",
      "required": [
        "liquidity_pool_id"
      ],
      "properties": {
        "liquidity_pool_id": {
          "$ref": "#/definitions/PoolId"
        }
      }
    },
    "LedgerKeyOffer": {
      "description": "LedgerKeyOffer is an XDR NestedStruct defines as:\n\n```text struct { AccountID sellerID; int64 offerID; } ```",
      "type": "object",
      "required": [
        "offer_id",
        "seller_id"
      ],
      "properties": {
        "offer_id": {
          "type": "integer",
          "format": "int64"
        },
        "seller_id": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "LedgerKeyTrustLine": {
      "description": "LedgerKeyTrustLine is an XDR NestedStruct defines as:\n\n```text struct { AccountID accountID; TrustLineAsset asset; } ```",
      "type": "object",
      "required": [
        "account_id",
        "asset"
      ],
      "properties": {
        "account_id": {
          "$ref": "#/definitions/AccountId"
        },
        "asset": {
          "$ref": "#/definitions/TrustLineAsset"
        }
      }
    },
    "LedgerKeyTtl": {
      "description": "LedgerKeyTtl is an XDR NestedStruct defines as:\n\n```text struct { // Hash of the LedgerKey that is associated with this TTLEntry Hash keyHash; } ```",
      "type": "object",
      "required": [
        "key_hash"
      ],
      "properties": {
        "key_hash": {
          "$ref": "#/definitions/Hash"
        }
      }
    },
    "LiquidityPoolConstantProductParameters": {
      "description": "LiquidityPoolConstantProductParameters is an XDR Struct defines as:\n\n```text struct LiquidityPoolConstantProductParameters { Asset assetA; // assetA < assetB Asset assetB; int32 fee; // Fee is in basis points, so the actual rate is (fee/100)% }; ```",
      "type": "object",
      "required": [
        "asset_a",
        "asset_b",
        "fee"
      ],
      "properties": {
        "asset_a": {
          "$ref": "#/definitions/Asset"
        },
        "asset_b": {
          "$ref": "#/definitions/Asset"
        },
        "fee": {
          "type": "integer",
          "format": "int32"
        }
      }
    },
    "LiquidityPoolDepositOp": {
      "description": "LiquidityPoolDepositOp is an XDR Struct defines as:\n\n```text struct LiquidityPoolDepositOp { PoolID liquidityPoolID; int64 maxAmountA; // maximum amount of first asset to deposit int64 maxAmountB; // maximum amount of second asset to deposit Price minPrice;   // minimum depositA/depositB Price maxPrice;   // maximum depositA/depositB }; ```",
      "type": "object",
      "required": [
        "liquidity_pool_id",
        "max_amount_a",
        "max_amount_b",
        "max_price",
        "min_price"
      ],
      "properties": {
        "liquidity_pool_id": {
          "$ref": "#/definitions/PoolId"
        },
        "max_amount_a": {
          "type": "integer",
          "format": "int64"
        },
        "max_amount_b": {
          "type": "integer",
          "format": "int64"
        },
        "max_price": {
          "$ref": "#/definitions/Price"
        },
        "min_price": {
          "$ref": "#/definitions/Price"
        }
      }
    },
    "LiquidityPoolParameters": {
      "description": "LiquidityPoolParameters is an XDR Union defines as:\n\n```text union LiquidityPoolParameters switch (LiquidityPoolType type) { case LIQUIDITY_POOL_CONSTANT_PRODUCT: LiquidityPoolConstantProductParameters constantProduct; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "liquidity_pool_constant_product"
          ],
          "properties": {
            "liquidity_pool_constant_product": {
              "$ref": "#/definitions/LiquidityPoolConstantProductParameters"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "LiquidityPoolWithdrawOp": {
      "description": "LiquidityPoolWithdrawOp is an XDR Struct defines as:\n\n```text struct LiquidityPoolWithdrawOp { PoolID liquidityPoolID; int64 amount;     // amount of pool shares to withdraw int64 minAmountA; // minimum amount of first asset to withdraw int64 minAmountB; // minimum amount of second asset to withdraw }; ```",
      "type": "object",
      "required": [
        "amount",
        "liquidity_pool_id",
        "min_amount_a",
        "min_amount_b"
      ],
      "properties": {
        "amount": {
          "type": "integer",
          "format": "int64"
        },
        "liquidity_pool_id": {
          "$ref": "#/definitions/PoolId"
        },
        "min_amount_a": {
          "type": "integer",
          "format": "int64"
        },
        "min_amount_b": {
          "type": "integer",
          "format": "int64"
        }
      }
    },
    "ManageBuyOfferOp": {
      "description": "ManageBuyOfferOp is an XDR Struct defines as:\n\n```text struct ManageBuyOfferOp { Asset selling; Asset buying; int64 buyAmount; // amount being bought. if set to 0, delete the offer Price price;     // price of thing being bought in terms of what you are // selling\n\n// 0=create a new offer, otherwise edit an existing offer int64 offerID; }; ```",
      "type": "object",
      "required": [
        "buy_amount",
        "buying",
        "offer_id",
        "price",
        "selling"
      ],
      "properties": {
        "buy_amount": {
          "type": "integer",
          "format": "int64"
        },
        "buying": {
          "$ref": "#/definitions/Asset"
        },
        "offer_id": {
          "type": "integer",
          "format": "int64"
        },
        "price": {
          "$ref": "#/definitions/Price"
        },
        "selling": {
          "$ref": "#/definitions/Asset"
        }
      }
    },
    "ManageDataOp": {
      "description": "ManageDataOp is an XDR Struct defines as:\n\n```text struct ManageDataOp { string64 dataName; DataValue* dataValue; // set to null to clear }; ```",
      "type": "object",
      "required": [
        "data_name"
      ],
      "properties": {
        "data_name": {
          "$ref": "#/definitions/String64"
        },
        "data_value": {
          "anyOf": [
            {
              "$ref": "#/definitions/DataValue"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "ManageSellOfferOp": {
      "description": "ManageSellOfferOp is an XDR Struct defines as:\n\n```text struct ManageSellOfferOp { Asset selling; Asset buying; int64 amount; // amount being sold. if set to 0, delete the offer Price price;  // price of thing being sold in terms of what you are buying\n\n// 0=create a new offer, otherwise edit an existing offer int64 offerID; }; ```",
      "type": "object",
      "required": [
        "amount",
        "buying",
        "offer_id",
        "price",
        "selling"
      ],
      "properties": {
        "amount": {
          "type": "integer",
          "format": "int64"
        },
        "buying": {
          "$ref": "#/definitions/Asset"
        },
        "offer_id": {
          "type": "integer",
          "format": "int64"
        },
        "price": {
          "$ref": "#/definitions/Price"
        },
        "selling": {
          "$ref": "#/definitions/Asset"
        }
      }
    },
    "Memo": {
      "description": "Memo is an XDR Union defines as:\n\n```text union Memo switch (MemoType type) { case MEMO_NONE: void; case MEMO_TEXT: string text<28>; case MEMO_ID: uint64 id; case MEMO_HASH: Hash hash; // the hash of what to pull from the content server case MEMO_RETURN: Hash retHash; // the hash of the tx you are rejecting }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "none"
          ]
        },
        {
          "type": "object",
          "required": [
            "text"
          ],
          "properties": {
            "text": {
              "$ref": "#/definitions/StringM_for_28"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "id"
          ],
          "properties": {
            "id": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0.0
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "hash"
          ],
          "properties": {
            "hash": {
              "$ref": "#/definitions/Hash"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "return"
          ],
          "properties": {
            "return": {
              "$ref": "#/definitions/Hash"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "MuxedAccount": {
      "description": "MuxedAccount is an XDR Union defines as:\n\n```text union MuxedAccount switch (CryptoKeyType type) { case KEY_TYPE_ED25519: uint256 ed25519; case KEY_TYPE_MUXED_ED25519: struct { uint64 id; uint256 ed25519; } med25519; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "Ed25519"
          ],
          "properties": {
            "Ed25519": {
              "$ref": "#/definitions/Uint256"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "MuxedEd25519"
          ],
          "properties": {
            "MuxedEd25519": {
              "$ref": "#/definitions/MuxedAccountMed25519"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "MuxedAccountMed25519": {
      "description": "MuxedAccountMed25519 is an XDR NestedStruct defines as:\n\n```text struct { uint64 id; uint256 ed25519; } ```",
      "type": "object",
      "required": [
        "ed25519",
        "id"
      ],
      "properties": {
        "ed25519": {
          "$ref": "#/definitions/Uint256"
        },
        "id": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        }
      }
    },
    "Operation": {
      "description": "Operation is an XDR Struct defines as:\n\n```text struct Operation { // sourceAccount is the account used to run the operation // if not set, the runtime defaults to \"sourceAccount\" specified at // the transaction level MuxedAccount* sourceAccount;\n\nunion switch (OperationType type) { case CREATE_ACCOUNT: CreateAccountOp createAccountOp; case PAYMENT: PaymentOp paymentOp; case PATH_PAYMENT_STRICT_RECEIVE: PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp; case MANAGE_SELL_OFFER: ManageSellOfferOp manageSellOfferOp; case CREATE_PASSIVE_SELL_OFFER: CreatePassiveSellOfferOp createPassiveSellOfferOp; case SET_OPTIONS: SetOptionsOp setOptionsOp; case CHANGE_TRUST: ChangeTrustOp changeTrustOp; case ALLOW_TRUST: AllowTrustOp allowTrustOp; case ACCOUNT_MERGE: MuxedAccount destination; case INFLATION: void; case MANAGE_DATA: ManageDataOp manageDataOp; case BUMP_SEQUENCE: BumpSequenceOp bumpSequenceOp; case MANAGE_BUY_OFFER: ManageBuyOfferOp manageBuyOfferOp; case PATH_PAYMENT_STRICT_SEND: PathPaymentStrictSendOp pathPaymentStrictSendOp; case CREATE_CLAIMABLE_BALANCE: CreateClaimableBalanceOp createClaimableBalanceOp; case CLAIM_CLAIMABLE_BALANCE: ClaimClaimableBalanceOp claimClaimableBalanceOp; case BEGIN_SPONSORING_FUTURE_RESERVES: BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp; case END_SPONSORING_FUTURE_RESERVES: void; case REVOKE_SPONSORSHIP: RevokeSponsorshipOp revokeSponsorshipOp; case CLAWBACK: ClawbackOp clawbackOp; case CLAWBACK_CLAIMABLE_BALANCE: ClawbackClaimableBalanceOp clawbackClaimableBalanceOp; case SET_TRUST_LINE_FLAGS: SetTrustLineFlagsOp setTrustLineFlagsOp; case LIQUIDITY_POOL_DEPOSIT: LiquidityPoolDepositOp liquidityPoolDepositOp; case LIQUIDITY_POOL_WITHDRAW: LiquidityPoolWithdrawOp liquidityPoolWithdrawOp; case INVOKE_HOST_FUNCTION: InvokeHostFunctionOp invokeHostFunctionOp; case EXTEND_FOOTPRINT_TTL: ExtendFootprintTTLOp extendFootprintTTLOp; case RESTORE_FOOTPRINT: RestoreFootprintOp restoreFootprintOp; } body; }; ```",
      "type": "object",
      "required": [
        "body"
      ],
      "properties": {
        "body": {
          "$ref": "#/definitions/OperationBody"
        },
        "source_account": {
          "anyOf": [
            {
              "$ref": "#/definitions/MuxedAccount"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "OperationBody": {
      "description": "OperationBody is an XDR NestedUnion defines as:\n\n```text union switch (OperationType type) { case CREATE_ACCOUNT: CreateAccountOp createAccountOp; case PAYMENT: PaymentOp paymentOp; case PATH_PAYMENT_STRICT_RECEIVE: PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp; case MANAGE_SELL_OFFER: ManageSellOfferOp manageSellOfferOp; case CREATE_PASSIVE_SELL_OFFER: CreatePassiveSellOfferOp createPassiveSellOfferOp; case SET_OPTIONS: SetOptionsOp setOptionsOp; case CHANGE_TRUST: ChangeTrustOp changeTrustOp; case ALLOW_TRUST: AllowTrustOp allowTrustOp; case ACCOUNT_MERGE: MuxedAccount destination; case INFLATION: void; case MANAGE_DATA: ManageDataOp manageDataOp; case BUMP_SEQUENCE: BumpSequenceOp bumpSequenceOp; case MANAGE_BUY_OFFER: ManageBuyOfferOp manageBuyOfferOp; case PATH_PAYMENT_STRICT_SEND: PathPaymentStrictSendOp pathPaymentStrictSendOp; case CREATE_CLAIMABLE_BALANCE: CreateClaimableBalanceOp createClaimableBalanceOp; case CLAIM_CLAIMABLE_BALANCE: ClaimClaimableBalanceOp claimClaimableBalanceOp; case BEGIN_SPONSORING_FUTURE_RESERVES: BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp; case END_SPONSORING_FUTURE_RESERVES: void; case REVOKE_SPONSORSHIP: RevokeSponsorshipOp revokeSponsorshipOp; case CLAWBACK: ClawbackOp clawbackOp; case CLAWBACK_CLAIMABLE_BALANCE: ClawbackClaimableBalanceOp clawbackClaimableBalanceOp; case SET_TRUST_LINE_FLAGS: SetTrustLineFlagsOp setTrustLineFlagsOp; case LIQUIDITY_POOL_DEPOSIT: LiquidityPoolDepositOp liquidityPoolDepositOp; case LIQUIDITY_POOL_WITHDRAW: LiquidityPoolWithdrawOp liquidityPoolWithdrawOp; case INVOKE_HOST_FUNCTION: InvokeHostFunctionOp invokeHostFunctionOp; case EXTEND_FOOTPRINT_TTL: ExtendFootprintTTLOp extendFootprintTTLOp; case RESTORE_FOOTPRINT: RestoreFootprintOp restoreFootprintOp; } ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "inflation",
            "end_sponsoring_future_reserves"
          ]
        },
        {
          "type": "object",
          "required": [
            "create_account"
          ],
          "properties": {
            "create_account": {
              "$ref": "#/definitions/CreateAccountOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "payment"
          ],
          "properties": {
            "payment": {
              "$ref": "#/definitions/PaymentOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "path_payment_strict_receive"
          ],
          "properties": {
            "path_payment_strict_receive": {
              "$ref": "#/definitions/PathPaymentStrictReceiveOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "manage_sell_offer"
          ],
          "properties": {
            "manage_sell_offer": {
              "$ref": "#/definitions/ManageSellOfferOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "create_passive_sell_offer"
          ],
          "properties": {
            "create_passive_sell_offer": {
              "$ref": "#/definitions/CreatePassiveSellOfferOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "set_options"
          ],
          "properties": {
            "set_options": {
              "$ref": "#/definitions/SetOptionsOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "change_trust"
          ],
          "properties": {
            "change_trust": {
              "$ref": "#/definitions/ChangeTrustOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "allow_trust"
          ],
          "properties": {
            "allow_trust": {
              "$ref": "#/definitions/AllowTrustOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "account_merge"
          ],
          "properties": {
            "account_merge": {
              "$ref": "#/definitions/MuxedAccount"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "manage_data"
          ],
          "properties": {
            "manage_data": {
              "$ref": "#/definitions/ManageDataOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "bump_sequence"
          ],
          "properties": {
            "bump_sequence": {
              "$ref": "#/definitions/BumpSequenceOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "manage_buy_offer"
          ],
          "properties": {
            "manage_buy_offer": {
              "$ref": "#/definitions/ManageBuyOfferOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "path_payment_strict_send"
          ],
          "properties": {
            "path_payment_strict_send": {
              "$ref": "#/definitions/PathPaymentStrictSendOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "create_claimable_balance"
          ],
          "properties": {
            "create_claimable_balance": {
              "$ref": "#/definitions/CreateClaimableBalanceOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "claim_claimable_balance"
          ],
          "properties": {
            "claim_claimable_balance": {
              "$ref": "#/definitions/ClaimClaimableBalanceOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "begin_sponsoring_future_reserves"
          ],
          "properties": {
            "begin_sponsoring_future_reserves": {
              "$ref": "#/definitions/BeginSponsoringFutureReservesOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "revoke_sponsorship"
          ],
          "properties": {
            "revoke_sponsorship": {
              "$ref": "#/definitions/RevokeSponsorshipOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "clawback"
          ],
          "properties": {
            "clawback": {
              "$ref": "#/definitions/ClawbackOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "clawback_claimable_balance"
          ],
          "properties": {
            "clawback_claimable_balance": {
              "$ref": "#/definitions/ClawbackClaimableBalanceOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "set_trust_line_flags"
          ],
          "properties": {
            "set_trust_line_flags": {
              "$ref": "#/definitions/SetTrustLineFlagsOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "liquidity_pool_deposit"
          ],
          "properties": {
            "liquidity_pool_deposit": {
              "$ref": "#/definitions/LiquidityPoolDepositOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "liquidity_pool_withdraw"
          ],
          "properties": {
            "liquidity_pool_withdraw": {
              "$ref": "#/definitions/LiquidityPoolWithdrawOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "invoke_host_function"
          ],
          "properties": {
            "invoke_host_function": {
              "$ref": "#/definitions/InvokeHostFunctionOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "extend_footprint_ttl"
          ],
          "properties": {
            "extend_footprint_ttl": {
              "$ref": "#/definitions/ExtendFootprintTtlOp"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "restore_footprint"
          ],
          "properties": {
            "restore_footprint": {
              "$ref": "#/definitions/RestoreFootprintOp"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "PathPaymentStrictReceiveOp": {
      "description": "PathPaymentStrictReceiveOp is an XDR Struct defines as:\n\n```text struct PathPaymentStrictReceiveOp { Asset sendAsset; // asset we pay with int64 sendMax;   // the maximum amount of sendAsset to // send (excluding fees). // The operation will fail if can't be met\n\nMuxedAccount destination; // recipient of the payment Asset destAsset;          // what they end up with int64 destAmount;         // amount they end up with\n\nAsset path<5>; // additional hops it must go through to get there }; ```",
      "type": "object",
      "required": [
        "dest_amount",
        "dest_asset",
        "destination",
        "path",
        "send_asset",
        "send_max"
      ],
      "properties": {
        "dest_amount": {
          "type": "integer",
          "format": "int64"
        },
        "dest_asset": {
          "$ref": "#/definitions/Asset"
        },
        "destination": {
          "$ref": "#/definitions/MuxedAccount"
        },
        "path": {
          "$ref": "#/definitions/VecM_for_Asset_and_5"
        },
        "send_asset": {
          "$ref": "#/definitions/Asset"
        },
        "send_max": {
          "type": "integer",
          "format": "int64"
        }
      }
    },
    "PathPaymentStrictSendOp": {
      "description": "PathPaymentStrictSendOp is an XDR Struct defines as:\n\n```text struct PathPaymentStrictSendOp { Asset sendAsset;  // asset we pay with int64 sendAmount; // amount of sendAsset to send (excluding fees)\n\nMuxedAccount destination; // recipient of the payment Asset destAsset;          // what they end up with int64 destMin;            // the minimum amount of dest asset to // be received // The operation will fail if it can't be met\n\nAsset path<5>; // additional hops it must go through to get there }; ```",
      "type": "object",
      "required": [
        "dest_asset",
        "dest_min",
        "destination",
        "path",
        "send_amount",
        "send_asset"
      ],
      "properties": {
        "dest_asset": {
          "$ref": "#/definitions/Asset"
        },
        "dest_min": {
          "type": "integer",
          "format": "int64"
        },
        "destination": {
          "$ref": "#/definitions/MuxedAccount"
        },
        "path": {
          "$ref": "#/definitions/VecM_for_Asset_and_5"
        },
        "send_amount": {
          "type": "integer",
          "format": "int64"
        },
        "send_asset": {
          "$ref": "#/definitions/Asset"
        }
      }
    },
    "PaymentOp": {
      "description": "PaymentOp is an XDR Struct defines as:\n\n```text struct PaymentOp { MuxedAccount destination; // recipient of the payment Asset asset;              // what they end up with int64 amount;             // amount they end up with }; ```",
      "type": "object",
      "required": [
        "amount",
        "asset",
        "destination"
      ],
      "properties": {
        "amount": {
          "type": "integer",
          "format": "int64"
        },
        "asset": {
          "$ref": "#/definitions/Asset"
        },
        "destination": {
          "$ref": "#/definitions/MuxedAccount"
        }
      }
    },
    "PoolId": {
      "description": "PoolId is an XDR Typedef defines as:\n\n```text typedef Hash PoolID; ```",
      "allOf": [
        {
          "$ref": "#/definitions/Hash"
        }
      ]
    },
    "Preconditions": {
      "description": "Preconditions is an XDR Union defines as:\n\n```text union Preconditions switch (PreconditionType type) { case PRECOND_NONE: void; case PRECOND_TIME: TimeBounds timeBounds; case PRECOND_V2: PreconditionsV2 v2; }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "none"
          ]
        },
        {
          "type": "object",
          "required": [
            "time"
          ],
          "properties": {
            "time": {
              "$ref": "#/definitions/TimeBounds"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "v2"
          ],
          "properties": {
            "v2": {
              "$ref": "#/definitions/PreconditionsV2"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "PreconditionsV2": {
      "description": "PreconditionsV2 is an XDR Struct defines as:\n\n```text struct PreconditionsV2 { TimeBounds* timeBounds;\n\n// Transaction only valid for ledger numbers n such that // minLedger <= n < maxLedger (if maxLedger == 0, then // only minLedger is checked) LedgerBounds* ledgerBounds;\n\n// If NULL, only valid when sourceAccount's sequence number // is seqNum - 1.  Otherwise, valid when sourceAccount's // sequence number n satisfies minSeqNum <= n < tx.seqNum. // Note that after execution the account's sequence number // is always raised to tx.seqNum, and a transaction is not // valid if tx.seqNum is too high to ensure replay protection. SequenceNumber* minSeqNum;\n\n// For the transaction to be valid, the current ledger time must // be at least minSeqAge greater than sourceAccount's seqTime. Duration minSeqAge;\n\n// For the transaction to be valid, the current ledger number // must be at least minSeqLedgerGap greater than sourceAccount's // seqLedger. uint32 minSeqLedgerGap;\n\n// For the transaction to be valid, there must be a signature // corresponding to every Signer in this array, even if the // signature is not otherwise required by the sourceAccount or // operations. SignerKey extraSigners<2>; }; ```",
      "type": "object",
      "required": [
        "extra_signers",
        "min_seq_age",
        "min_seq_ledger_gap"
      ],
      "properties": {
        "extra_signers": {
          "$ref": "#/definitions/VecM_for_SignerKey_and_2"
        },
        "ledger_bounds": {
          "anyOf": [
            {
              "$ref": "#/definitions/LedgerBounds"
            },
            {
              "type": "null"
            }
          ]
        },
        "min_seq_age": {
          "$ref": "#/definitions/Duration"
        },
        "min_seq_ledger_gap": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "min_seq_num": {
          "anyOf": [
            {
              "$ref": "#/definitions/SequenceNumber"
            },
            {
              "type": "null"
            }
          ]
        },
        "time_bounds": {
          "anyOf": [
            {
              "$ref": "#/definitions/TimeBounds"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "Price": {
      "description": "Price is an XDR Struct defines as:\n\n```text struct Price { int32 n; // numerator int32 d; // denominator }; ```",
      "type": "object",
      "required": [
        "d",
        "n"
      ],
      "properties": {
        "d": {
          "type": "integer",
          "format": "int32"
        },
        "n": {
          "type": "integer",
          "format": "int32"
        }
      }
    },
    "PublicKey": {
      "description": "PublicKey is an XDR Union defines as:\n\n```text union PublicKey switch (PublicKeyType type) { case PUBLIC_KEY_TYPE_ED25519: uint256 ed25519; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "PublicKeyTypeEd25519"
          ],
          "properties": {
            "PublicKeyTypeEd25519": {
              "$ref": "#/definitions/Uint256"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "RestoreFootprintOp": {
      "description": "RestoreFootprintOp is an XDR Struct defines as:\n\n```text struct RestoreFootprintOp { ExtensionPoint ext; }; ```",
      "type": "object",
      "required": [
        "ext"
      ],
      "properties": {
        "ext": {
          "$ref": "#/definitions/ExtensionPoint"
        }
      }
    },
    "RevokeSponsorshipOp": {
      "description": "RevokeSponsorshipOp is an XDR Union defines as:\n\n```text union RevokeSponsorshipOp switch (RevokeSponsorshipType type) { case REVOKE_SPONSORSHIP_LEDGER_ENTRY: LedgerKey ledgerKey; case REVOKE_SPONSORSHIP_SIGNER: struct { AccountID accountID; SignerKey signerKey; } signer; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "ledger_entry"
          ],
          "properties": {
            "ledger_entry": {
              "$ref": "#/definitions/LedgerKey"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "signer"
          ],
          "properties": {
            "signer": {
              "$ref": "#/definitions/RevokeSponsorshipOpSigner"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "RevokeSponsorshipOpSigner": {
      "description": "RevokeSponsorshipOpSigner is an XDR NestedStruct defines as:\n\n```text struct { AccountID accountID; SignerKey signerKey; } ```",
      "type": "object",
      "required": [
        "account_id",
        "signer_key"
      ],
      "properties": {
        "account_id": {
          "$ref": "#/definitions/AccountId"
        },
        "signer_key": {
          "$ref": "#/definitions/SignerKey"
        }
      }
    },
    "ScAddress": {
      "description": "ScAddress is an XDR Union defines as:\n\n```text union SCAddress switch (SCAddressType type) { case SC_ADDRESS_TYPE_ACCOUNT: AccountID accountId; case SC_ADDRESS_TYPE_CONTRACT: Hash contractId; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "Account"
          ],
          "properties": {
            "Account": {
              "$ref": "#/definitions/AccountId"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "Contract"
          ],
          "properties": {
            "Contract": {
              "$ref": "#/definitions/Hash"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ScBytes": {
      "description": "ScBytes is an XDR Typedef defines as:\n\n```text typedef opaque SCBytes<>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/BytesM_for_4294967295"
        }
      ]
    },
    "ScContractInstance": {
      "description": "ScContractInstance is an XDR Struct defines as:\n\n```text struct SCContractInstance { ContractExecutable executable; SCMap* storage; }; ```",
      "type": "object",
      "required": [
        "executable"
      ],
      "properties": {
        "executable": {
          "$ref": "#/definitions/ContractExecutable"
        },
        "storage": {
          "anyOf": [
            {
              "$ref": "#/definitions/ScMap"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "ScError": {
      "description": "ScError is an XDR Union defines as:\n\n```text union SCError switch (SCErrorType type) { case SCE_CONTRACT: uint32 contractCode; case SCE_WASM_VM: case SCE_CONTEXT: case SCE_STORAGE: case SCE_OBJECT: case SCE_CRYPTO: case SCE_EVENTS: case SCE_BUDGET: case SCE_VALUE: case SCE_AUTH: SCErrorCode code; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "contract"
          ],
          "properties": {
            "contract": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0.0
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "wasm_vm"
          ],
          "properties": {
            "wasm_vm": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "context"
          ],
          "properties": {
            "context": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "storage"
          ],
          "properties": {
            "storage": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "object"
          ],
          "properties": {
            "object": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "crypto"
          ],
          "properties": {
            "crypto": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "events"
          ],
          "properties": {
            "events": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "budget"
          ],
          "properties": {
            "budget": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "value"
          ],
          "properties": {
            "value": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "auth"
          ],
          "properties": {
            "auth": {
              "$ref": "#/definitions/ScErrorCode"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ScErrorCode": {
      "description": "ScErrorCode is an XDR Enum defines as:\n\n```text enum SCErrorCode { SCEC_ARITH_DOMAIN = 0,      // Some arithmetic was undefined (overflow, divide-by-zero). SCEC_INDEX_BOUNDS = 1,      // Something was indexed beyond its bounds. SCEC_INVALID_INPUT = 2,     // User provided some otherwise-bad data. SCEC_MISSING_VALUE = 3,     // Some value was required but not provided. SCEC_EXISTING_VALUE = 4,    // Some value was provided where not allowed. SCEC_EXCEEDED_LIMIT = 5,    // Some arbitrary limit -- gas or otherwise -- was hit. SCEC_INVALID_ACTION = 6,    // Data was valid but action requested was not. SCEC_INTERNAL_ERROR = 7,    // The host detected an error in its own logic. SCEC_UNEXPECTED_TYPE = 8,   // Some type wasn't as expected. SCEC_UNEXPECTED_SIZE = 9    // Something's size wasn't as expected. }; ```",
      "type": "string",
      "enum": [
        "arith_domain",
        "index_bounds",
        "invalid_input",
        "missing_value",
        "existing_value",
        "exceeded_limit",
        "invalid_action",
        "internal_error",
        "unexpected_type",
        "unexpected_size"
      ]
    },
    "ScMap": {
      "description": "ScMap is an XDR Typedef defines as:\n\n```text typedef SCMapEntry SCMap<>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/VecM_for_ScMapEntry_and_4294967295"
        }
      ]
    },
    "ScMapEntry": {
      "description": "ScMapEntry is an XDR Struct defines as:\n\n```text struct SCMapEntry { SCVal key; SCVal val; }; ```",
      "type": "object",
      "required": [
        "key",
        "val"
      ],
      "properties": {
        "key": {
          "$ref": "#/definitions/ScVal"
        },
        "val": {
          "$ref": "#/definitions/ScVal"
        }
      }
    },
    "ScNonceKey": {
      "description": "ScNonceKey is an XDR Struct defines as:\n\n```text struct SCNonceKey { int64 nonce; }; ```",
      "type": "object",
      "required": [
        "nonce"
      ],
      "properties": {
        "nonce": {
          "type": "integer",
          "format": "int64"
        }
      }
    },
    "ScString": {
      "description": "ScString is an XDR Typedef defines as:\n\n```text typedef string SCString<>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/StringM_for_4294967295"
        }
      ]
    },
    "ScSymbol": {
      "description": "ScSymbol is an XDR Typedef defines as:\n\n```text typedef string SCSymbol<SCSYMBOL_LIMIT>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/StringM_for_32"
        }
      ]
    },
    "ScVal": {
      "description": "ScVal is an XDR Union defines as:\n\n```text union SCVal switch (SCValType type) {\n\ncase SCV_BOOL: bool b; case SCV_VOID: void; case SCV_ERROR: SCError error;\n\ncase SCV_U32: uint32 u32; case SCV_I32: int32 i32;\n\ncase SCV_U64: uint64 u64; case SCV_I64: int64 i64; case SCV_TIMEPOINT: TimePoint timepoint; case SCV_DURATION: Duration duration;\n\ncase SCV_U128: UInt128Parts u128; case SCV_I128: Int128Parts i128;\n\ncase SCV_U256: UInt256Parts u256; case SCV_I256: Int256Parts i256;\n\ncase SCV_BYTES: SCBytes bytes; case SCV_STRING: SCString str; case SCV_SYMBOL: SCSymbol sym;\n\n// Vec and Map are recursive so need to live // behind an option, due to xdrpp limitations. case SCV_VEC: SCVec *vec; case SCV_MAP: SCMap *map;\n\ncase SCV_ADDRESS: SCAddress address;\n\n// Special SCVals reserved for system-constructed contract-data // ledger keys, not generally usable elsewhere. case SCV_LEDGER_KEY_CONTRACT_INSTANCE: void; case SCV_LEDGER_KEY_NONCE: SCNonceKey nonce_key;\n\ncase SCV_CONTRACT_INSTANCE: SCContractInstance instance; }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "void",
            "ledger_key_contract_instance"
          ]
        },
        {
          "type": "object",
          "required": [
            "bool"
          ],
          "properties": {
            "bool": {
              "type": "boolean"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "error"
          ],
          "properties": {
            "error": {
              "$ref": "#/definitions/ScError"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "u32"
          ],
          "properties": {
            "u32": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0.0
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "i32"
          ],
          "properties": {
            "i32": {
              "type": "integer",
              "format": "int32"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "u64"
          ],
          "properties": {
            "u64": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0.0
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "i64"
          ],
          "properties": {
            "i64": {
              "type": "integer",
              "format": "int64"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "timepoint"
          ],
          "properties": {
            "timepoint": {
              "$ref": "#/definitions/TimePoint"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "duration"
          ],
          "properties": {
            "duration": {
              "$ref": "#/definitions/Duration"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "u128"
          ],
          "properties": {
            "u128": {
              "$ref": "#/definitions/UInt128Parts"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "i128"
          ],
          "properties": {
            "i128": {
              "$ref": "#/definitions/Int128Parts"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "u256"
          ],
          "properties": {
            "u256": {
              "$ref": "#/definitions/UInt256Parts"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "i256"
          ],
          "properties": {
            "i256": {
              "$ref": "#/definitions/Int256Parts"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "bytes"
          ],
          "properties": {
            "bytes": {
              "$ref": "#/definitions/ScBytes"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "string"
          ],
          "properties": {
            "string": {
              "$ref": "#/definitions/ScString"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "symbol"
          ],
          "properties": {
            "symbol": {
              "$ref": "#/definitions/ScSymbol"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "vec"
          ],
          "properties": {
            "vec": {
              "anyOf": [
                {
                  "$ref": "#/definitions/ScVec"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "map"
          ],
          "properties": {
            "map": {
              "anyOf": [
                {
                  "$ref": "#/definitions/ScMap"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "address"
          ],
          "properties": {
            "address": {
              "$ref": "#/definitions/ScAddress"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "ledger_key_nonce"
          ],
          "properties": {
            "ledger_key_nonce": {
              "$ref": "#/definitions/ScNonceKey"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "contract_instance"
          ],
          "properties": {
            "contract_instance": {
              "$ref": "#/definitions/ScContractInstance"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "ScVec": {
      "description": "ScVec is an XDR Typedef defines as:\n\n```text typedef SCVal SCVec<>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/VecM_for_ScVal_and_4294967295"
        }
      ]
    },
    "SequenceNumber": {
      "description": "SequenceNumber is an XDR Typedef defines as:\n\n```text typedef int64 SequenceNumber; ```",
      "type": "integer",
      "format": "int64"
    },
    "SetOptionsOp": {
      "description": "SetOptionsOp is an XDR Struct defines as:\n\n```text struct SetOptionsOp { AccountID* inflationDest; // sets the inflation destination\n\nuint32* clearFlags; // which flags to clear uint32* setFlags;   // which flags to set\n\n// account threshold manipulation uint32* masterWeight; // weight of the master account uint32* lowThreshold; uint32* medThreshold; uint32* highThreshold;\n\nstring32* homeDomain; // sets the home domain\n\n// Add, update or remove a signer for the account // signer is deleted if the weight is 0 Signer* signer; }; ```",
      "type": "object",
      "properties": {
        "clear_flags": {
          "type": [
            "integer",
            "null"
          ],
          "format": "uint32",
          "minimum": 0.0
        },
        "high_threshold": {
          "type": [
            "integer",
            "null"
          ],
          "format": "uint32",
          "minimum": 0.0
        },
        "home_domain": {
          "anyOf": [
            {
              "$ref": "#/definitions/String32"
            },
            {
              "type": "null"
            }
          ]
        },
        "inflation_dest": {
          "anyOf": [
            {
              "$ref": "#/definitions/AccountId"
            },
            {
              "type": "null"
            }
          ]
        },
        "low_threshold": {
          "type": [
            "integer",
            "null"
          ],
          "format": "uint32",
          "minimum": 0.0
        },
        "master_weight": {
          "type": [
            "integer",
            "null"
          ],
          "format": "uint32",
          "minimum": 0.0
        },
        "med_threshold": {
          "type": [
            "integer",
            "null"
          ],
          "format": "uint32",
          "minimum": 0.0
        },
        "set_flags": {
          "type": [
            "integer",
            "null"
          ],
          "format": "uint32",
          "minimum": 0.0
        },
        "signer": {
          "anyOf": [
            {
              "$ref": "#/definitions/Signer"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "SetTrustLineFlagsOp": {
      "description": "SetTrustLineFlagsOp is an XDR Struct defines as:\n\n```text struct SetTrustLineFlagsOp { AccountID trustor; Asset asset;\n\nuint32 clearFlags; // which flags to clear uint32 setFlags;   // which flags to set }; ```",
      "type": "object",
      "required": [
        "asset",
        "clear_flags",
        "set_flags",
        "trustor"
      ],
      "properties": {
        "asset": {
          "$ref": "#/definitions/Asset"
        },
        "clear_flags": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "set_flags": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "trustor": {
          "$ref": "#/definitions/AccountId"
        }
      }
    },
    "Signature": {
      "description": "Signature is an XDR Typedef defines as:\n\n```text typedef opaque Signature<64>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/BytesM_for_64"
        }
      ]
    },
    "SignatureHint": {
      "description": "SignatureHint is an XDR Typedef defines as:\n\n```text typedef opaque SignatureHint[4]; ```",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      },
      "maxItems": 4,
      "minItems": 4
    },
    "Signer": {
      "description": "Signer is an XDR Struct defines as:\n\n```text struct Signer { SignerKey key; uint32 weight; // really only need 1 byte }; ```",
      "type": "object",
      "required": [
        "key",
        "weight"
      ],
      "properties": {
        "key": {
          "$ref": "#/definitions/SignerKey"
        },
        "weight": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        }
      }
    },
    "SignerKey": {
      "description": "SignerKey is an XDR Union defines as:\n\n```text union SignerKey switch (SignerKeyType type) { case SIGNER_KEY_TYPE_ED25519: uint256 ed25519; case SIGNER_KEY_TYPE_PRE_AUTH_TX: /* SHA-256 Hash of TransactionSignaturePayload structure */ uint256 preAuthTx; case SIGNER_KEY_TYPE_HASH_X: /* Hash of random 256 bit preimage X */ uint256 hashX; case SIGNER_KEY_TYPE_ED25519_SIGNED_PAYLOAD: struct { /* Public key that must sign the payload. */ uint256 ed25519; /* Payload to be raw signed by ed25519. */ opaque payload<64>; } ed25519SignedPayload; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "Ed25519"
          ],
          "properties": {
            "Ed25519": {
              "$ref": "#/definitions/Uint256"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "PreAuthTx"
          ],
          "properties": {
            "PreAuthTx": {
              "$ref": "#/definitions/Uint256"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "HashX"
          ],
          "properties": {
            "HashX": {
              "$ref": "#/definitions/Uint256"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "Ed25519SignedPayload"
          ],
          "properties": {
            "Ed25519SignedPayload": {
              "$ref": "#/definitions/SignerKeyEd25519SignedPayload"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "SignerKeyEd25519SignedPayload": {
      "description": "SignerKeyEd25519SignedPayload is an XDR NestedStruct defines as:\n\n```text struct { /* Public key that must sign the payload. */ uint256 ed25519; /* Payload to be raw signed by ed25519. */ opaque payload<64>; } ```",
      "type": "object",
      "required": [
        "ed25519",
        "payload"
      ],
      "properties": {
        "ed25519": {
          "$ref": "#/definitions/Uint256"
        },
        "payload": {
          "$ref": "#/definitions/BytesM_for_64"
        }
      }
    },
    "SorobanAddressCredentials": {
      "description": "SorobanAddressCredentials is an XDR Struct defines as:\n\n```text struct SorobanAddressCredentials { SCAddress address; int64 nonce; uint32 signatureExpirationLedger; SCVal signature; }; ```",
      "type": "object",
      "required": [
        "address",
        "nonce",
        "signature",
        "signature_expiration_ledger"
      ],
      "properties": {
        "address": {
          "$ref": "#/definitions/ScAddress"
        },
        "nonce": {
          "type": "integer",
          "format": "int64"
        },
        "signature": {
          "$ref": "#/definitions/ScVal"
        },
        "signature_expiration_ledger": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        }
      }
    },
    "SorobanAuthorizationEntry": {
      "description": "SorobanAuthorizationEntry is an XDR Struct defines as:\n\n```text struct SorobanAuthorizationEntry { SorobanCredentials credentials; SorobanAuthorizedInvocation rootInvocation; }; ```",
      "type": "object",
      "required": [
        "credentials",
        "root_invocation"
      ],
      "properties": {
        "credentials": {
          "$ref": "#/definitions/SorobanCredentials"
        },
        "root_invocation": {
          "$ref": "#/definitions/SorobanAuthorizedInvocation"
        }
      }
    },
    "SorobanAuthorizedFunction": {
      "description": "SorobanAuthorizedFunction is an XDR Union defines as:\n\n```text union SorobanAuthorizedFunction switch (SorobanAuthorizedFunctionType type) { case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CONTRACT_FN: InvokeContractArgs contractFn; case SOROBAN_AUTHORIZED_FUNCTION_TYPE_CREATE_CONTRACT_HOST_FN: CreateContractArgs createContractHostFn; }; ```",
      "oneOf": [
        {
          "type": "object",
          "required": [
            "contract_fn"
          ],
          "properties": {
            "contract_fn": {
              "$ref": "#/definitions/InvokeContractArgs"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "create_contract_host_fn"
          ],
          "properties": {
            "create_contract_host_fn": {
              "$ref": "#/definitions/CreateContractArgs"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "SorobanAuthorizedInvocation": {
      "description": "SorobanAuthorizedInvocation is an XDR Struct defines as:\n\n```text struct SorobanAuthorizedInvocation { SorobanAuthorizedFunction function; SorobanAuthorizedInvocation subInvocations<>; }; ```",
      "type": "object",
      "required": [
        "function",
        "sub_invocations"
      ],
      "properties": {
        "function": {
          "$ref": "#/definitions/SorobanAuthorizedFunction"
        },
        "sub_invocations": {
          "$ref": "#/definitions/VecM_for_SorobanAuthorizedInvocation_and_4294967295"
        }
      }
    },
    "SorobanCredentials": {
      "description": "SorobanCredentials is an XDR Union defines as:\n\n```text union SorobanCredentials switch (SorobanCredentialsType type) { case SOROBAN_CREDENTIALS_SOURCE_ACCOUNT: void; case SOROBAN_CREDENTIALS_ADDRESS: SorobanAddressCredentials address; }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "source_account"
          ]
        },
        {
          "type": "object",
          "required": [
            "address"
          ],
          "properties": {
            "address": {
              "$ref": "#/definitions/SorobanAddressCredentials"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "SorobanResources": {
      "description": "SorobanResources is an XDR Struct defines as:\n\n```text struct SorobanResources { // The ledger footprint of the transaction. LedgerFootprint footprint; // The maximum number of instructions this transaction can use uint32 instructions;\n\n// The maximum number of bytes this transaction can read from ledger uint32 readBytes; // The maximum number of bytes this transaction can write to ledger uint32 writeBytes; }; ```",
      "type": "object",
      "required": [
        "footprint",
        "instructions",
        "read_bytes",
        "write_bytes"
      ],
      "properties": {
        "footprint": {
          "$ref": "#/definitions/LedgerFootprint"
        },
        "instructions": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "read_bytes": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "write_bytes": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        }
      }
    },
    "SorobanTransactionData": {
      "description": "SorobanTransactionData is an XDR Struct defines as:\n\n```text struct SorobanTransactionData { ExtensionPoint ext; SorobanResources resources; // Amount of the transaction `fee` allocated to the Soroban resource fees. // The fraction of `resourceFee` corresponding to `resources` specified // above is *not* refundable (i.e. fees for instructions, ledger I/O), as // well as fees for the transaction size. // The remaining part of the fee is refundable and the charged value is // based on the actual consumption of refundable resources (events, ledger // rent bumps). // The `inclusionFee` used for prioritization of the transaction is defined // as `tx.fee - resourceFee`. int64 resourceFee; }; ```",
      "type": "object",
      "required": [
        "ext",
        "resource_fee",
        "resources"
      ],
      "properties": {
        "ext": {
          "$ref": "#/definitions/ExtensionPoint"
        },
        "resource_fee": {
          "type": "integer",
          "format": "int64"
        },
        "resources": {
          "$ref": "#/definitions/SorobanResources"
        }
      }
    },
    "String32": {
      "description": "String32 is an XDR Typedef defines as:\n\n```text typedef string string32<32>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/StringM_for_32"
        }
      ]
    },
    "String64": {
      "description": "String64 is an XDR Typedef defines as:\n\n```text typedef string string64<64>; ```",
      "allOf": [
        {
          "$ref": "#/definitions/StringM_for_64"
        }
      ]
    },
    "StringM_for_28": {
      "description": "A string type that contains arbitrary bytes.\n\nConvertible, fallibly, to/from a Rust UTF-8 String using [`TryFrom`]/[`TryInto`]/[`StringM::to_utf8_string`].\n\nConvertible, lossyly, to a Rust UTF-8 String using [`StringM::to_utf8_string_lossy`].\n\nConvertible to/from escaped printable-ASCII using [`Display`]/[`ToString`]/[`FromStr`].",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      }
    },
    "StringM_for_32": {
      "description": "A string type that contains arbitrary bytes.\n\nConvertible, fallibly, to/from a Rust UTF-8 String using [`TryFrom`]/[`TryInto`]/[`StringM::to_utf8_string`].\n\nConvertible, lossyly, to a Rust UTF-8 String using [`StringM::to_utf8_string_lossy`].\n\nConvertible to/from escaped printable-ASCII using [`Display`]/[`ToString`]/[`FromStr`].",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      }
    },
    "StringM_for_4294967295": {
      "description": "A string type that contains arbitrary bytes.\n\nConvertible, fallibly, to/from a Rust UTF-8 String using [`TryFrom`]/[`TryInto`]/[`StringM::to_utf8_string`].\n\nConvertible, lossyly, to a Rust UTF-8 String using [`StringM::to_utf8_string_lossy`].\n\nConvertible to/from escaped printable-ASCII using [`Display`]/[`ToString`]/[`FromStr`].",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      }
    },
    "StringM_for_64": {
      "description": "A string type that contains arbitrary bytes.\n\nConvertible, fallibly, to/from a Rust UTF-8 String using [`TryFrom`]/[`TryInto`]/[`StringM::to_utf8_string`].\n\nConvertible, lossyly, to a Rust UTF-8 String using [`StringM::to_utf8_string_lossy`].\n\nConvertible to/from escaped printable-ASCII using [`Display`]/[`ToString`]/[`FromStr`].",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      }
    },
    "TimeBounds": {
      "description": "TimeBounds is an XDR Struct defines as:\n\n```text struct TimeBounds { TimePoint minTime; TimePoint maxTime; // 0 here means no maxTime }; ```",
      "type": "object",
      "required": [
        "max_time",
        "min_time"
      ],
      "properties": {
        "max_time": {
          "$ref": "#/definitions/TimePoint"
        },
        "min_time": {
          "$ref": "#/definitions/TimePoint"
        }
      }
    },
    "TimePoint": {
      "description": "TimePoint is an XDR Typedef defines as:\n\n```text typedef uint64 TimePoint; ```",
      "type": "integer",
      "format": "uint64",
      "minimum": 0.0
    },
    "Transaction": {
      "description": "Transaction is an XDR Struct defines as:\n\n```text struct Transaction { // account used to run the transaction MuxedAccount sourceAccount;\n\n// the fee the sourceAccount will pay uint32 fee;\n\n// sequence number to consume in the account SequenceNumber seqNum;\n\n// validity conditions Preconditions cond;\n\nMemo memo;\n\nOperation operations<MAX_OPS_PER_TX>;\n\n// reserved for future use union switch (int v) { case 0: void; case 1: SorobanTransactionData sorobanData; } ext; }; ```",
      "type": "object",
      "required": [
        "cond",
        "ext",
        "fee",
        "memo",
        "operations",
        "seq_num",
        "source_account"
      ],
      "properties": {
        "cond": {
          "$ref": "#/definitions/Preconditions"
        },
        "ext": {
          "$ref": "#/definitions/TransactionExt"
        },
        "fee": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "memo": {
          "$ref": "#/definitions/Memo"
        },
        "operations": {
          "$ref": "#/definitions/VecM_for_Operation_and_100"
        },
        "seq_num": {
          "$ref": "#/definitions/SequenceNumber"
        },
        "source_account": {
          "$ref": "#/definitions/MuxedAccount"
        }
      }
    },
    "TransactionExt": {
      "description": "TransactionExt is an XDR NestedUnion defines as:\n\n```text union switch (int v) { case 0: void; case 1: SorobanTransactionData sorobanData; } ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "v0"
          ]
        },
        {
          "type": "object",
          "required": [
            "v1"
          ],
          "properties": {
            "v1": {
              "$ref": "#/definitions/SorobanTransactionData"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "TransactionV0": {
      "description": "TransactionV0 is an XDR Struct defines as:\n\n```text struct TransactionV0 { uint256 sourceAccountEd25519; uint32 fee; SequenceNumber seqNum; TimeBounds* timeBounds; Memo memo; Operation operations<MAX_OPS_PER_TX>; union switch (int v) { case 0: void; } ext; }; ```",
      "type": "object",
      "required": [
        "ext",
        "fee",
        "memo",
        "operations",
        "seq_num",
        "source_account_ed25519"
      ],
      "properties": {
        "ext": {
          "$ref": "#/definitions/TransactionV0Ext"
        },
        "fee": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0.0
        },
        "memo": {
          "$ref": "#/definitions/Memo"
        },
        "operations": {
          "$ref": "#/definitions/VecM_for_Operation_and_100"
        },
        "seq_num": {
          "$ref": "#/definitions/SequenceNumber"
        },
        "source_account_ed25519": {
          "$ref": "#/definitions/Uint256"
        },
        "time_bounds": {
          "anyOf": [
            {
              "$ref": "#/definitions/TimeBounds"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "TransactionV0Envelope": {
      "description": "TransactionV0Envelope is an XDR Struct defines as:\n\n```text struct TransactionV0Envelope { TransactionV0 tx; /* Each decorated signature is a signature over the SHA256 hash of * a TransactionSignaturePayload */ DecoratedSignature signatures<20>; }; ```",
      "type": "object",
      "required": [
        "signatures",
        "tx"
      ],
      "properties": {
        "signatures": {
          "$ref": "#/definitions/VecM_for_DecoratedSignature_and_20"
        },
        "tx": {
          "$ref": "#/definitions/TransactionV0"
        }
      }
    },
    "TransactionV0Ext": {
      "description": "TransactionV0Ext is an XDR NestedUnion defines as:\n\n```text union switch (int v) { case 0: void; } ```",
      "type": "string",
      "enum": [
        "v0"
      ]
    },
    "TransactionV1Envelope": {
      "description": "TransactionV1Envelope is an XDR Struct defines as:\n\n```text struct TransactionV1Envelope { Transaction tx; /* Each decorated signature is a signature over the SHA256 hash of * a TransactionSignaturePayload */ DecoratedSignature signatures<20>; }; ```",
      "type": "object",
      "required": [
        "signatures",
        "tx"
      ],
      "properties": {
        "signatures": {
          "$ref": "#/definitions/VecM_for_DecoratedSignature_and_20"
        },
        "tx": {
          "$ref": "#/definitions/Transaction"
        }
      }
    },
    "TrustLineAsset": {
      "description": "TrustLineAsset is an XDR Union defines as:\n\n```text union TrustLineAsset switch (AssetType type) { case ASSET_TYPE_NATIVE: // Not credit void;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM4: AlphaNum4 alphaNum4;\n\ncase ASSET_TYPE_CREDIT_ALPHANUM12: AlphaNum12 alphaNum12;\n\ncase ASSET_TYPE_POOL_SHARE: PoolID liquidityPoolID;\n\n// add other asset types here in the future }; ```",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "native"
          ]
        },
        {
          "type": "object",
          "required": [
            "credit_alphanum4"
          ],
          "properties": {
            "credit_alphanum4": {
              "$ref": "#/definitions/AlphaNum4"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "credit_alphanum12"
          ],
          "properties": {
            "credit_alphanum12": {
              "$ref": "#/definitions/AlphaNum12"
            }
          },
          "additionalProperties": false
        },
        {
          "type": "object",
          "required": [
            "pool_share"
          ],
          "properties": {
            "pool_share": {
              "$ref": "#/definitions/PoolId"
            }
          },
          "additionalProperties": false
        }
      ]
    },
    "UInt128Parts": {
      "description": "UInt128Parts is an XDR Struct defines as:\n\n```text struct UInt128Parts { uint64 hi; uint64 lo; }; ```",
      "type": "object",
      "required": [
        "hi",
        "lo"
      ],
      "properties": {
        "hi": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "lo": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        }
      }
    },
    "UInt256Parts": {
      "description": "UInt256Parts is an XDR Struct defines as:\n\n```text struct UInt256Parts { uint64 hi_hi; uint64 hi_lo; uint64 lo_hi; uint64 lo_lo; }; ```",
      "type": "object",
      "required": [
        "hi_hi",
        "hi_lo",
        "lo_hi",
        "lo_lo"
      ],
      "properties": {
        "hi_hi": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "hi_lo": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "lo_hi": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        },
        "lo_lo": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0.0
        }
      }
    },
    "Uint256": {
      "description": "Uint256 is an XDR Typedef defines as:\n\n```text typedef opaque uint256[32]; ```",
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0.0
      },
      "maxItems": 32,
      "minItems": 32
    },
    "VecM_for_Asset_and_5": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Asset"
      }
    },
    "VecM_for_ClaimPredicate_and_2": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/ClaimPredicate"
      }
    },
    "VecM_for_Claimant_and_10": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Claimant"
      }
    },
    "VecM_for_DecoratedSignature_and_20": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/DecoratedSignature"
      }
    },
    "VecM_for_LedgerKey_and_4294967295": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/LedgerKey"
      }
    },
    "VecM_for_Operation_and_100": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/Operation"
      }
    },
    "VecM_for_ScMapEntry_and_4294967295": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/ScMapEntry"
      }
    },
    "VecM_for_ScVal_and_4294967295": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/ScVal"
      }
    },
    "VecM_for_SignerKey_and_2": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/SignerKey"
      }
    },
    "VecM_for_SorobanAuthorizationEntry_and_4294967295": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/SorobanAuthorizationEntry"
      }
    },
    "VecM_for_SorobanAuthorizedInvocation_and_4294967295": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/SorobanAuthorizedInvocation"
      }
    }
  }
}"##,
    );

    Ok(())
}
