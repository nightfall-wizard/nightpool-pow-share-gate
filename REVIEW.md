# Maintainer review guide

## Scope

External Nightpool Devnet research following PR #80.

The original Rust implementation is unchanged.
No PPLNS implementation is duplicated.

## Review

1. Inspect nightpool_share_gate.rs.
2. Inspect the GitHub Actions verification results.
3. Optionally execute bash verify.sh.

## Original Android evidence

- Fourteen tests passed.
- Zero failed tests.
- One block candidate accepted by Devnet consensus.

The verifier reproduces these checks on Linux.

## Boundaries

- Devnet only.
- Trusted template inputs.
- Local and in-memory replay tracking.
- No miner authentication.
- No persistent share ledger.
- No peer-to-peer share chain.
- No payout enforcement.
- No Mainnet changes.
- No independent security audit.

This is not a production-ready mining pool.

## Proposed upstream integration

One external documentation link only.

No changes to consensus, mining, emission,
wallets or protocol behavior.

Final acceptance remains with the maintainer.

## Suggested release-note entry

Documentation links external Nightpool research
demonstrating canonical Devnet PoW share validation.
