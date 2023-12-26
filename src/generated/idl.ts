import { Idl } from "@coral-xyz/anchor";

const APP_IDL: Idl = {
  accounts: [
    {
      name: "User",
      type: {
        fields: [
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "balance",
            type: "u64",
          },
          {
            name: "referredBalance",
            type: "u64",
          },
          {
            name: "isAdmin",
            type: "bool",
          },
          {
            name: "isAmbassador",
            type: "bool",
          },
          {
            name: "myAmbassadorQuota",
            type: "u64",
          },
          {
            name: "payout",
            type: "i64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
        kind: "struct",
      },
    },
    {
      name: "App",
      type: {
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "symbol",
            type: "string",
          },
          {
            name: "decimals",
            type: "u8",
          },
          {
            name: "dividendFee",
            type: "u8",
          },
          {
            name: "tokenInitialPrice",
            type: "u64",
          },
          {
            name: "tokenIncrementalPrice",
            type: "u64",
          },
          {
            name: "tokenSupply",
            type: "u64",
          },
          {
            name: "magnitude",
            type: "u64",
          },
          {
            name: "stakingRequirement",
            type: "u64",
          },
          {
            name: "ambassadorMaxPurchase",
            type: "u64",
          },
          {
            name: "ambassadorQuota",
            type: "u64",
          },
          {
            name: "profitPerShare",
            type: "u64",
          },
          {
            name: "onlyAmbassadors",
            type: "bool",
          },
          {
            name: "isInitialized",
            type: "bool",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
        kind: "struct",
      },
    },
  ],
  errors: [
    {
      code: 6000,
      msg: "BP must be less than or equal to 10000",
      name: "InvalidBasisPoints",
    },
    {
      code: 6001,
      msg: "UninitializedAccount",
      name: "UninitializedAccount",
    },
    {
      code: 6002,
      msg: "PublicKeyMismatch",
      name: "PublicKeyMismatch",
    },
    {
      code: 6003,
      msg: "IncorrectOwner",
      name: "IncorrectOwner",
    },
    {
      code: 6004,
      msg: "You are only allowed to place one bet at a time",
      name: "MultipleBetsNotAllowed",
    },
    {
      code: 6005,
      msg: "Bet amounts must be greater than 0",
      name: "InvalidBetAmount",
    },
    {
      code: 6006,
      msg: "Number of flips must equal 1",
      name: "InvalidNumFlips",
    },
    {
      code: 6007,
      msg: "Invalid value for bets (may not match on-chain data)",
      name: "InvalidBets",
    },
    {
      code: 6008,
      msg: "Invalid auction house authority",
      name: "InvalidAuctionHouseAuthority",
    },
    {
      code: 6009,
      msg: "Account is already initialized",
      name: "AlreadyInitialized",
    },
  ],
  instructions: [
    {
      accounts: [
        {
          isMut: true,
          isSigner: true,
          name: "admin",
        },
        {
          isMut: true,
          isSigner: false,
          name: "adminData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "programData",
        },
        {
          isMut: false,
          isSigner: false,
          name: "systemProgram",
        },
      ],
      args: [],
      name: "initialize",
    },
  ],
  metadata: {
    address: "AHScsmJEzPyCPEpYqS66C3zkoEgoKscR6p4Rkp8xZoAN",
  },
  name: "app",
  version: "0.1.0",
};

export default APP_IDL;
