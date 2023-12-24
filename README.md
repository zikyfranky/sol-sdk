<div align="center">
  <h1>Sol-SDK: A Solana POWH</h1>
  <a href="#overview">Overview</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#repo-structure">Repo Structure</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#development">Development</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#deployment">Deployment</a>
  <br />
  <hr />
</div>

## Overview

This is the Solana program that powers powh.

- **Devnet address**: `AHScsmJEzPyCPEpYqS66C3zkoEgoKscR6p4Rkp8xZoAN`
- **Live website (uses devnet)**:
- **[Frontend code for App](https://github.com/zikyfranky)**
- **[Backend code for App](https://github.com/zikyfranky)**

This Solana program implements a dividend app.

- Transaction 1
  - Instruction 1 [optional]: `initialize`. Creates a `Skwizz` account, which is a PDA of `[program]` and a `User` account, which is a PDA of `[user, Pubkey]`. If the `Skwizz` account already exists, this instruction can be omitted.
  - Signers: `admin`

### Accounts

- **`Skwizz`**: This account stores information about the App
- **`User`**: This account stores information about a specific user.

### Instructions

- `initialize`: This instruction creates a new `Skwizz` account, and initializes it with the Admin `User` account.

## Repo Structure

This repo contains the Solana program source code and the source code for a TypeScript SDK, in addition to some client-side program tests written in TypeScript.

```.
├── programs            # Solana program source code
├── scripts             # Some helper bash scripts
├── src                 # TypeScript source folder
│   ├── generated       # Generated program IDL and type definitions
│   ├── sdk             # Gumdrop program TypeScript SDK
│   └── tests           # Program tests
├── ...                 # Other misc. project config files
└── README.md
```

## Development

Use the same version of Anchor CLI as `.github/workflows/release-package.yml`

I.e. run `avm use 0.29.0`

### Prerequisites

- Install Rust, Solana, Anchor: [https://book.anchor-lang.com/getting_started/installation.html](https://book.anchor-lang.com/getting_started/installation.html)
  - Helpful [resources for installing on M1 Mac](https://twitter.com/friedbrioche/status/1494075962874499075)
- Install the [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool)

### Setup Steps

1. Run `yarn`
2. Run `yarn test`

If everything is set up correctly, all tests should pass and you should be ready to start developing!

## Deployment

### Solana Program

Run `solana-test-validator`.
Run `yarn deploy-program -c localhost`.

### TypeScript SDK

Follow the following steps to publish a new version of the TypeScript SDK:

1. Run `yarn version` and enter a new appropriate [semver version](https://docs.npmjs.com/about-semantic-versioning) for the npm package. That will create a new tag and commit.
2. Run `git push origin NEW_TAG`.
3. `git push` the new commit as well.

This will push the new release tag to GitHub and trigger the release pipeline, after which clients can install the latest SDK with `yarn add @isaacfrank/sol-sdk@latest`.
