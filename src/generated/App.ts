export type App = {
  accounts: [
    {
      name: "user";
      type: {
        fields: [
          {
            name: "authority";
            type: "publicKey";
          },
          {
            name: "balance";
            type: "u128";
          },
          {
            name: "referredBalance";
            type: "u128";
          },
          {
            name: "isAdmin";
            type: "bool";
          },
          {
            name: "isAmb";
            type: "bool";
          },
          {
            name: "referredBy";
            type: "publicKey";
          },
          {
            name: "payout";
            type: "i128";
          },
          {
            name: "usableLocked";
            type: "u128";
          },
          {
            name: "totalLocked";
            type: "u128";
          },
          {
            name: "lockedStarttime";
            type: "i64";
          },
          {
            name: "lockedEndtime";
            type: "i64";
          }
        ];
        kind: "struct";
      };
    },
    {
      name: "app";
      type: {
        fields: [
          {
            name: "name";
            type: "string";
          },
          {
            name: "symbol";
            type: "string";
          },
          {
            name: "decimals";
            type: "u8";
          },
          {
            name: "dividendFee";
            type: "u8";
          },
          {
            name: "tokenInitialPrice";
            type: "u128";
          },
          {
            name: "tokenIncrementalPrice";
            type: "u128";
          },
          {
            name: "contractBalance";
            type: "u128";
          },
          {
            name: "tokenSupply";
            type: "u128";
          },
          {
            name: "magnitude";
            type: "u64";
          },
          {
            name: "stakingRequirement";
            type: "u128";
          },
          {
            name: "profitPerShare";
            type: "u128";
          },
          {
            name: "isInitialized";
            type: "bool";
          },
          {
            name: "isInitialPhase";
            type: "bool";
          }
        ];
        kind: "struct";
      };
    }
  ];
  errors: [
    {
      code: 6000;
      msg: "You are not a holder";
      name: "NotABagHolder";
    },
    {
      code: 6001;
      msg: "You do not have a profit";
      name: "NoPofit";
    },
    {
      code: 6002;
      msg: "You are not an administrator";
      name: "NotAnAdmin";
    },
    {
      code: 6003;
      msg: "It is in initial phase";
      name: "IsInitialPhase";
    },
    {
      code: 6004;
      msg: "It is not in initial phase";
      name: "IsPostInitialPhase";
    },
    {
      code: 6005;
      msg: "You sent less token than required";
      name: "SentLessToken";
    },
    {
      code: 6006;
      msg: "You do not have enough funds";
      name: "InsufficientBalance";
    },
    {
      code: 6007;
      msg: "Already Initialized Account";
      name: "AlreadyInitialized";
    },
    {
      code: 6008;
      msg: "To address doesn't match generated to account";
      name: "InvalidToAccount";
    },
    {
      code: 6009;
      msg: "Signer isn't Owner";
      name: "NotOwner";
    },
    {
      code: 6010;
      msg: "You can't refer yourself";
      name: "SelfReferral";
    },
    {
      code: 6011;
      msg: "You can't transfer to yourself";
      name: "SelfTransfer";
    }
  ];
  instructions: [
    {
      accounts: [
        {
          isMut: true;
          isSigner: true;
          name: "admin";
        },
        {
          isMut: true;
          isSigner: false;
          name: "adminData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "metadata";
        },
        {
          docs: ["CHECK gets created by the program"];
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "rent";
        },
        {
          isMut: false;
          isSigner: false;
          name: "sysvarInstructions";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenMetadataProgram";
        }
      ];
      args: [
        {
          name: "metadata";
          type: {
            defined: "InitTokenParams";
          };
        }
      ];
      name: "initialize";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userAta";
        },
        {
          isMut: true;
          isOptional: true;
          isSigner: false;
          name: "referredByData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [
        {
          name: "lamportsToSend";
          type: "u128";
        },
        {
          name: "referredBy";
          type: {
            option: "publicKey";
          };
        }
      ];
      name: "buy";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [];
      name: "reinvest";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [];
      name: "exit";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "toData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "toInfo";
        },
        {
          isMut: true;
          isSigner: false;
          name: "toAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [
        {
          name: "to";
          type: "publicKey";
        },
        {
          name: "lamportsToSend";
          type: "u128";
        }
      ];
      name: "transfer";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [];
      name: "withdraw";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [
        {
          name: "lamportsToSend";
          type: "u128";
        }
      ];
      name: "sell";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        }
      ];
      args: [];
      name: "disableInitialStage";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: false;
          name: "mint";
        },
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "fromData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "fromAta";
        },
        {
          isMut: false;
          isSigner: false;
          name: "receipientInfo";
        },
        {
          isMut: true;
          isSigner: false;
          name: "receipientData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "receipientAta";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "tokenProgram";
        },
        {
          isMut: false;
          isSigner: false;
          name: "associatedTokenProgram";
        }
      ];
      args: [
        {
          name: "amountOfTokens";
          type: "u128";
        },
        {
          name: "updatePayoutBy";
          type: "i128";
        },
        {
          name: "receipient";
          type: "publicKey";
        }
      ];
      name: "distributeToken";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: true;
          name: "admin";
        },
        {
          isMut: true;
          isSigner: false;
          name: "adminData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        }
      ];
      args: [
        {
          name: "user";
          type: "publicKey";
        },
        {
          name: "status";
          type: "bool";
        }
      ];
      name: "setAdministrator";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: true;
          name: "admin";
        },
        {
          isMut: true;
          isSigner: false;
          name: "adminData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        }
      ];
      args: [
        {
          name: "user";
          type: "publicKey";
        },
        {
          name: "status";
          type: "bool";
        }
      ];
      name: "setAmbassador";
    },
    {
      accounts: [
        {
          isMut: true;
          isSigner: true;
          name: "user";
        },
        {
          isMut: true;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: true;
          isSigner: false;
          name: "programData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "systemProgram";
        }
      ];
      args: [
        {
          name: "amountOfTokens";
          type: "u128";
        }
      ];
      name: "setStakingRequirement";
    },
    {
      accounts: [
        {
          isMut: false;
          isSigner: false;
          name: "userData";
        },
        {
          isMut: false;
          isSigner: false;
          name: "programData";
        }
      ];
      args: [
        {
          name: "includingRef";
          type: "bool";
        }
      ];
      name: "myDividends";
      returns: "u128";
    },
    {
      accounts: [
        {
          isMut: false;
          isSigner: false;
          name: "programData";
        }
      ];
      args: [];
      name: "sellPrice";
      returns: "u128";
    },
    {
      accounts: [
        {
          isMut: false;
          isSigner: false;
          name: "programData";
        }
      ];
      args: [];
      name: "buyPrice";
      returns: "u128";
    },
    {
      accounts: [
        {
          isMut: false;
          isSigner: false;
          name: "programData";
        }
      ];
      args: [
        {
          name: "tokens";
          type: "u128";
        }
      ];
      name: "calculateLamportsReceived";
      returns: "u128";
    },
    {
      accounts: [
        {
          isMut: false;
          isSigner: false;
          name: "programData";
        }
      ];
      args: [
        {
          name: "lamports";
          type: "u128";
        }
      ];
      name: "calculateTokensReceived";
      returns: "u128";
    }
  ];
  name: "app";
  types: [
    {
      name: "InitTokenParams";
      type: {
        fields: [
          {
            name: "name";
            type: "string";
          },
          {
            name: "symbol";
            type: "string";
          },
          {
            name: "uri";
            type: "string";
          },
          {
            name: "decimals";
            type: "u8";
          }
        ];
        kind: "struct";
      };
    }
  ];
  version: "0.1.0";
};

export const IDL: App = {
  accounts: [
    {
      name: "user",
      type: {
        fields: [
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "balance",
            type: "u128",
          },
          {
            name: "referredBalance",
            type: "u128",
          },
          {
            name: "isAdmin",
            type: "bool",
          },
          {
            name: "isAmb",
            type: "bool",
          },
          {
            name: "referredBy",
            type: "publicKey",
          },
          {
            name: "payout",
            type: "i128",
          },
          {
            name: "usableLocked",
            type: "u128",
          },
          {
            name: "totalLocked",
            type: "u128",
          },
          {
            name: "lockedStarttime",
            type: "i64",
          },
          {
            name: "lockedEndtime",
            type: "i64",
          },
        ],
        kind: "struct",
      },
    },
    {
      name: "app",
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
            type: "u128",
          },
          {
            name: "tokenIncrementalPrice",
            type: "u128",
          },
          {
            name: "contractBalance",
            type: "u128",
          },
          {
            name: "tokenSupply",
            type: "u128",
          },
          {
            name: "magnitude",
            type: "u64",
          },
          {
            name: "stakingRequirement",
            type: "u128",
          },
          {
            name: "profitPerShare",
            type: "u128",
          },
          {
            name: "isInitialized",
            type: "bool",
          },
          {
            name: "isInitialPhase",
            type: "bool",
          },
        ],
        kind: "struct",
      },
    },
  ],
  errors: [
    {
      code: 6000,
      msg: "You are not a holder",
      name: "NotABagHolder",
    },
    {
      code: 6001,
      msg: "You do not have a profit",
      name: "NoPofit",
    },
    {
      code: 6002,
      msg: "You are not an administrator",
      name: "NotAnAdmin",
    },
    {
      code: 6003,
      msg: "It is in initial phase",
      name: "IsInitialPhase",
    },
    {
      code: 6004,
      msg: "It is not in initial phase",
      name: "IsPostInitialPhase",
    },
    {
      code: 6005,
      msg: "You sent less token than required",
      name: "SentLessToken",
    },
    {
      code: 6006,
      msg: "You do not have enough funds",
      name: "InsufficientBalance",
    },
    {
      code: 6007,
      msg: "Already Initialized Account",
      name: "AlreadyInitialized",
    },
    {
      code: 6008,
      msg: "To address doesn't match generated to account",
      name: "InvalidToAccount",
    },
    {
      code: 6009,
      msg: "Signer isn't Owner",
      name: "NotOwner",
    },
    {
      code: 6010,
      msg: "You can't refer yourself",
      name: "SelfReferral",
    },
    {
      code: 6011,
      msg: "You can't transfer to yourself",
      name: "SelfTransfer",
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
          name: "metadata",
        },
        {
          docs: ["CHECK gets created by the program"],
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: false,
          name: "programData",
        },
        {
          isMut: false,
          isSigner: false,
          name: "rent",
        },
        {
          isMut: false,
          isSigner: false,
          name: "sysvarInstructions",
        },
        {
          isMut: false,
          isSigner: false,
          name: "systemProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "tokenMetadataProgram",
        },
      ],
      args: [
        {
          name: "metadata",
          type: {
            defined: "InitTokenParams",
          },
        },
      ],
      name: "initialize",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userAta",
        },
        {
          isMut: true,
          isOptional: true,
          isSigner: false,
          name: "referredByData",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [
        {
          name: "lamportsToSend",
          type: "u128",
        },
        {
          name: "referredBy",
          type: {
            option: "publicKey",
          },
        },
      ],
      name: "buy",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userAta",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [],
      name: "reinvest",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userAta",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [],
      name: "exit",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userAta",
        },
        {
          isMut: true,
          isSigner: false,
          name: "toData",
        },
        {
          isMut: false,
          isSigner: false,
          name: "toInfo",
        },
        {
          isMut: true,
          isSigner: false,
          name: "toAta",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [
        {
          name: "to",
          type: "publicKey",
        },
        {
          name: "lamportsToSend",
          type: "u128",
        },
      ],
      name: "transfer",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userAta",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [],
      name: "withdraw",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userAta",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [
        {
          name: "lamportsToSend",
          type: "u128",
        },
      ],
      name: "sell",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
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
      name: "disableInitialStage",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: false,
          name: "mint",
        },
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "fromData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "fromAta",
        },
        {
          isMut: false,
          isSigner: false,
          name: "receipientInfo",
        },
        {
          isMut: true,
          isSigner: false,
          name: "receipientData",
        },
        {
          isMut: true,
          isSigner: false,
          name: "receipientAta",
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
        {
          isMut: false,
          isSigner: false,
          name: "tokenProgram",
        },
        {
          isMut: false,
          isSigner: false,
          name: "associatedTokenProgram",
        },
      ],
      args: [
        {
          name: "amountOfTokens",
          type: "u128",
        },
        {
          name: "updatePayoutBy",
          type: "i128",
        },
        {
          name: "receipient",
          type: "publicKey",
        },
      ],
      name: "distributeToken",
    },
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
          name: "userData",
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
      args: [
        {
          name: "user",
          type: "publicKey",
        },
        {
          name: "status",
          type: "bool",
        },
      ],
      name: "setAdministrator",
    },
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
          name: "userData",
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
      args: [
        {
          name: "user",
          type: "publicKey",
        },
        {
          name: "status",
          type: "bool",
        },
      ],
      name: "setAmbassador",
    },
    {
      accounts: [
        {
          isMut: true,
          isSigner: true,
          name: "user",
        },
        {
          isMut: true,
          isSigner: false,
          name: "userData",
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
      args: [
        {
          name: "amountOfTokens",
          type: "u128",
        },
      ],
      name: "setStakingRequirement",
    },
    {
      accounts: [
        {
          isMut: false,
          isSigner: false,
          name: "userData",
        },
        {
          isMut: false,
          isSigner: false,
          name: "programData",
        },
      ],
      args: [
        {
          name: "includingRef",
          type: "bool",
        },
      ],
      name: "myDividends",
      returns: "u128",
    },
    {
      accounts: [
        {
          isMut: false,
          isSigner: false,
          name: "programData",
        },
      ],
      args: [],
      name: "sellPrice",
      returns: "u128",
    },
    {
      accounts: [
        {
          isMut: false,
          isSigner: false,
          name: "programData",
        },
      ],
      args: [],
      name: "buyPrice",
      returns: "u128",
    },
    {
      accounts: [
        {
          isMut: false,
          isSigner: false,
          name: "programData",
        },
      ],
      args: [
        {
          name: "tokens",
          type: "u128",
        },
      ],
      name: "calculateLamportsReceived",
      returns: "u128",
    },
    {
      accounts: [
        {
          isMut: false,
          isSigner: false,
          name: "programData",
        },
      ],
      args: [
        {
          name: "lamports",
          type: "u128",
        },
      ],
      name: "calculateTokensReceived",
      returns: "u128",
    },
  ],
  name: "app",
  types: [
    {
      name: "InitTokenParams",
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
            name: "uri",
            type: "string",
          },
          {
            name: "decimals",
            type: "u8",
          },
        ],
        kind: "struct",
      },
    },
  ],
  version: "0.1.0",
};
