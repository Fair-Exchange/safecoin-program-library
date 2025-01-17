# Token Lending program

A lending protocol for the Token program on the Safecoin blockchain inspired by Aave and Compound.

Full documentation is available at https://spl.solana.com/token-lending

Web3 bindings are available in the `./js` directory.

### On-chain programs

Please note that only the lending program deployed to devnet is currently operational.

| Cluster | Program Address |
| --- | --- |
| Mainnet Beta | [`LendZqTs8gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi`](https://explorer.solana.com/address/LENeX3L4CE1euBZp4zUNuicLP2SUZCbgXYZgBpZ9hWZ) |
| Testnet | [`6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH`](https://explorer.solana.com/address/6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH?cluster=testnet) |
| Devnet | [`6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH`](https://explorer.solana.com/address/6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH?cluster=devnet) |

### Documentation

- [CLI docs](https://github.com/fair-exchange/safecoin-program-library/tree/master/token-lending/cli)
- [Client library docs](https://solana-labs.github.io/safecoin-program-library/token-lending/)

### Deploy a lending program (optional)

This is optional! You can skip these steps and use the [Token Lending CLI](./cli/README.md) with one of the on-chain programs listed above to create a lending market and add reserves to it.

1. [Install the Safecoin CLI](https://docs.solana.com/cli/install-safecoin-cli-tools)

1. Install the Token and Token Lending CLIs:
   ```shell
   cargo install safe-token-cli
   cargo install safe-token-lending-cli
   ```

1. Clone the SPL repo:
   ```shell
   git clone https://github.com/fair-exchange/safecoin-program-library.git
   ```

1. Go to the new directory:
   ```shell
   cd safecoin-program-library
   ```

1. Generate a keypair for yourself:
   ```shell
   safecoin-keygen new -o owner.json

   # Wrote new keypair to owner.json
   # ================================================================================
   # pubkey: JAgN4SZLNeCo9KTnr8EWt4FzEV1UDgHkcZwkVtWtfp6P
   # ================================================================================
   # Save this seed phrase and your BIP39 passphrase to recover your new keypair:
   # your seed words here never share them not even with your mom
   # ================================================================================
   ```
   This pubkey will be the owner of the lending market that can add reserves to it.

1. Generate a keypair for the program:
   ```shell
   safecoin-keygen new -o lending.json

   # Wrote new keypair to lending.json
   # ============================================================================
   # pubkey: 6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH
   # ============================================================================
   # Save this seed phrase and your BIP39 passphrase to recover your new keypair:
   # your seed words here never share them not even with your mom
   # ============================================================================
   ```
   This pubkey will be your Program ID.

1. Open `./token-lending/program/src/lib.rs` in your editor. In the line
   ```rust
   solana_program::declare_id!("6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH");
   ```
   replace the Program ID with yours.

1. Build the program binaries:
   ```shell
   cargo build
   cargo build-sbf
   ```

1. Prepare to deploy to devnet:
   ```shell
   safecoin config set --url https://api.devnet.safecoin.org
   ```

1. Score yourself some sweet SAFE:
   ```shell
   safecoin airdrop -k owner.json 2
   safecoin airdrop -k owner.json 2
   safecoin airdrop -k owner.json 2
   ```
   You'll use this for transaction fees, rent for your program accounts, and initial reserve liquidity. If you run
   into issues with the airdrop command, see the [docs](https://docs.solana.com/cli/transfer-tokens#airdrop-some-tokens-to-get-started) for more info.

1. Deploy the program:
   ```shell
   safecoin program deploy \
     -k owner.json \
     --program-id lending.json \
     target/deploy/safe_token_lending.so

   # Program Id: 6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH
   ```
   If the deployment doesn't succeed, follow [this guide](https://docs.solana.com/cli/deploy-a-program#resuming-a-failed-deploy) to resume it.

1. Wrap some of your SAFE as an SPL Token:
   ```shell
   safe-token wrap \
      --fee-payer owner.json \
      2.0 \
      -- owner.json

   # Wrapping 2 SAFE into AJ2sgpgj6ZeQazPPiDyTYqN9vbj58QMaZQykB9Sr6XY
   ```
   You'll use this for initial reserve liquidity. Note the SPL Token account pubkey (e.g. `AJ2sgpgj6ZeQazPPiDyTYqN9vbj58QMaZQykB9Sr6XY`).

1. Use the [Token Lending CLI](./cli/README.md) to create a lending market and add reserves to it.
