#!/usr/bin/env bash
set -Eeuo pipefail
export LC_ALL=C
export CARGO_BUILD_JOBS=2
export CARGO_INCREMENTAL=0
export CARGO_PROFILE_TEST_OPT_LEVEL=2
export CARGO_PROFILE_TEST_DEBUG=0
export CARGO_NET_RETRY=5
export CARGO_HTTP_TIMEOUT=120
export CARGO_TERM_COLOR=never
export GIT_TERMINAL_PROMPT=0

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

sha256sum -c SHA256SUMS

REV="$(cat UPSTREAM.txt)"
[[ "$REV" =~ ^[0-9a-f]{40}$ ]]

TMP="$(mktemp -d)"
trap 'rm -rf -- "$TMP"' EXIT

git -C "$TMP" init -q

git -C "$TMP" fetch --depth 1 https://github.com/Instinctes/nightfall.git "$REV"

git -C "$TMP" checkout --detach -q FETCH_HEAD

test "$(git -C "$TMP" rev-parse HEAD)" = "$REV"

DEST="$TMP/crates/nightfall-consensus/examples"
mkdir -p "$DEST"

test ! -e "$DEST/nightpool_share_gate.rs"

cp "$ROOT/nightpool_share_gate.rs" "$DEST/"

rustc --version
cargo --version

(
    cd "$TMP"
    cargo test --locked         -p nightfall-consensus         --example nightpool_share_gate         -- --test-threads=1 --nocapture
) 2>&1 | tee "$TMP/results.txt"

grep -Fq '14 passed; 0 failed; 0 ignored' "$TMP/results.txt"
grep -Fq 'DEVNET_FULL_CONSENSUS_ACCEPTANCE=PASS' "$TMP/results.txt"
grep -Fq 'DEVNET_BLOCK_COUNT=1' "$TMP/results.txt"

git -C "$TMP" diff --exit-code
git -C "$TMP" diff --cached --exit-code

cmp "$ROOT/nightpool_share_gate.rs"     "$DEST/nightpool_share_gate.rs"

echo "RESEARCH_VERIFICATION=PASS"
echo "TESTS=14"
echo "FAILED=0"
echo "DEVNET_CONSENSUS=PASS"
