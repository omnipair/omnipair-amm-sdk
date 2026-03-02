# omnipair-amm-sdk

Rust SDK for integrating Omnipair with the Jupiter AMM interface.

## What this crate provides

- `OmnipairAmmClient`: Jupiter `Amm` implementation for Omnipair pairs.
- `OmnipairPair`: deserialization + local quote math that mirrors on-chain swap math.
- `OmnipairSwapAccounts`: helper type for building swap account metas.

## Layout

- `src/lib.rs` keeps shared constants, state structs, and quote math.
- `src/omnipair_amm_client.rs` contains the Jupiter client implementation.

## Quick test

```bash
cargo test
```

For the live on-chain test, set:

- `OMNIPAIR_RPC_URL` (optional, defaults to mainnet-beta)
- `OMNIPAIR_PAIR` (required)
