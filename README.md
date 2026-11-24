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

Use the exact upstream revision in UPSTREAM.txt.

    git clone https://github.com/Instinctes/nightfall.git
    cd nightfall
    git checkout "$(cat ../UPSTREAM.txt)"
    mkdir -p crates/nightfall-consensus/examples
    cp ../research/nightpool_share_gate.rs \
       crates/nightfall-consensus/examples/
    cargo test --locked -p nightfall-consensus \
       --example nightpool_share_gate -- \
       --test-threads=1 --nocapture

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
