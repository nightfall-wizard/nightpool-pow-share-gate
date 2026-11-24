# Nightpool PoW Share Gate

## Purpose

An isolated Devnet research prototype addressing
the missing proof-of-work share validation
identified in Nightfall PR #80.

This project does not recreate PPLNS accounting.

## Approach

Reuse upstream Nightfall canonical block headers,
Nighthash-v2, network parameters and difficulty
arithmetic instead of creating new cryptography.

The gate receives an already trusted block template
and a local chain tip.

## Security properties tested

- Zero difficulty rejection.
- Invalid capacity rejection.
- Canonical header binding.
- Exact Nighthash recomputation.
- Insufficient proof-of-work rejection.
- Fabricated hash rejection.
- Duplicate nonce rejection per template.
- Stale template rejection.
- Bounded accepted-share storage.
- Exact configured work credit.
- Distinction between a share and a block.
- Full Devnet consensus acceptance of a block
  candidate produced from the same template.

## Reproduction

Requirements: Bash, Git, SHA-256 utilities and Rust/Cargo.

From the research repository root:

    bash verify.sh

The verifier fetches the exact pinned Nightfall commit
and executes the original 14-test research harness.

It requires successful validation of one block candidate
by the in-memory Devnet consensus implementation.

No running node or real funds are required.

## Continuous verification

GitHub Actions executes the same verification on Linux.
The original Android/Termux evidence is in EVIDENCE.md.
See the Actions tab for independent results.

## Relationship to PR #80

Existing accounting research is acknowledged.
Its payout implementation is neither copied
nor reimplemented here.

The intended next integration is to pass only
verified work into a reviewed accounting layer.

## Limitations

Research prototype only.

Devnet parameters are deliberately inexpensive
and provide no production mining security.

The trusted-template assumption must be replaced
by independently verified miner templates before
use in a peer-to-peer share chain.

Replay tracking is local, in-memory and scoped
to a single gate instance. Restart persistence,
cross-peer replay protection, miner attribution,
share ancestry, network admission, DoS resistance,
share-chain fork choice and payout enforcement
remain outside this prototype.

No production pool or financial guarantee is
claimed.

The fixed test seeds are publicly known and must
never be used with real funds.

No Mainnet node, wallet, keys, funds or consensus
rules are modified.
