# Nightpool PoW Share Gate: evidence

Upstream commit:
dcd4727d9e3f355e157662e690473d8db522c41b

rustc 1.100.0 (0ef1bc386 2026-11-09) (built from a source tarball)
cargo 1.100.0 (3acc79c0c 2026-11-05) (built from a source tarball)

Public duplication scan:
Known prior work: research: add bounded Nightpool PPLNS accounting reference
PR #80 state: closed
PPLNS accounting will NOT be recreated.
Issues/PRs checked: 113
Branches checked: 25
Repository search results: 0
PASS: No matching implementation
identified by the public metadata scan.

Executed test results:
warning: unused workspace dependency `nightfall-node`
  --> Cargo.toml:34:1
   |
34 | nightfall-node = { path = "crates/nightfall-node" }
   | ^^^^^^^^^^^^^^
   |
   = note: `cargo::unused_workspace_dependencies` is set to `warn` by default
help: consider removing the workspace dependency `nightfall-node`
warning: unused workspace dependency `sha3`
  --> Cargo.toml:55:1
   |
55 | sha3 = "0.10"
   | ^^^^
   |
help: consider removing the workspace dependency `sha3`
warning: unused field `repository` in `workspace.package`
  --> Cargo.toml:23:1
   |
23 | repository = "https://github.com/nightfallcoin/nightfall"
   | ^^^^^^^^^^
   |
   = note: `cargo::unused_workspace_package_fields` is set to `warn` by default
help: consider removing the field `workspace.package.repository`
warning: unused field `authors` in `workspace.package`
  --> Cargo.toml:24:1
   |
24 | authors = ["Nightfall contributors"]
   | ^^^^^^^
   |
help: consider removing the field `workspace.package.authors`
warning: unused field `description` in `workspace.package`
  --> Cargo.toml:25:1
   |
25 | description = "NIGHTFALLCOIN — privacy-by-default sovereign L1"
   | ^^^^^^^^^^^
   |
help: consider removing the field `workspace.package.description`
warning: workspace (manifest) generated 5 warnings
warning: unused build dependency `uniffi`
  --> crates/nightfall-mobile/Cargo.toml:26:1
   |
26 | uniffi = { workspace = true, features = ["build"] }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `cargo::unused_dependencies` is set to `warn` by default
help: consider removing the dependency on `uniffi`
warning: `nightfall-mobile` (manifest) generated 1 warning
   Compiling libc v0.2.189
   Compiling cfg-if v1.0.4
   Compiling proc-macro2 v1.0.107
   Compiling getrandom v0.2.17
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.24
   Compiling rand_core v0.6.4
   Compiling version_check v0.9.5
   Compiling generic-array v0.14.7
   Compiling typenum v1.20.1
   Compiling crypto-common v0.1.7
   Compiling subtle v2.6.1
   Compiling syn v2.0.119
   Compiling syn v3.0.3
   Compiling serde_core v1.0.229
   Compiling zeroize_derive v1.5.0
   Compiling zeroize v1.9.0
   Compiling block-buffer v0.10.4
   Compiling serde v1.0.229
   Compiling digest v0.10.7
   Compiling serde_derive v1.0.229
   Compiling shlex v2.0.1
   Compiling find-msvc-tools v0.1.10
   Compiling cc v1.4.2
   Compiling semver v1.0.28
   Compiling rustc_version v0.4.1
   Compiling cpufeatures v0.2.17
   Compiling zerocopy v0.8.56
   Compiling curve25519-dalek v4.1.3
   Compiling ff v0.13.1
   Compiling group v0.13.0
   Compiling arrayvec v0.7.8
   Compiling ppv-lite86 v0.2.21
   Compiling keccak v0.1.6
   Compiling clear_on_drop v0.2.5
   Compiling inout v0.1.4
   Compiling thiserror v2.0.20
   Compiling thiserror v1.0.69
   Compiling tinyvec_macros v0.1.1
   Compiling zmij v1.0.23
   Compiling tinyvec v1.12.0
   Compiling cipher v0.4.4
   Compiling rand_chacha v0.3.1
   Compiling hex-conservative v0.2.2
   Compiling blake3 v1.8.6
   Compiling thiserror-impl v2.0.20
   Compiling thiserror-impl v1.0.69
   Compiling universal-hash v0.5.1
   Compiling base64ct v1.8.3
   Compiling byteorder v1.5.0
   Compiling signature v2.2.0
   Compiling serde_json v1.0.151
   Compiling opaque-debug v0.3.1
   Compiling poly1305 v0.8.0
   Compiling ed25519 v2.2.3
   Compiling merlin v3.0.0
   Compiling password-hash v0.5.0
   Compiling bitcoin_hashes v0.14.101
   Compiling rand v0.8.7
   Compiling chacha20 v0.9.1
   Compiling unicode-normalization v0.1.25
   Compiling sha3 v0.10.9
   Compiling sha2 v0.10.9
   Compiling blake2 v0.10.6
   Compiling aead v0.5.2
   Compiling hex v0.4.3
   Compiling itoa v1.0.18
   Compiling memchr v2.8.3
   Compiling constant_time_eq v0.4.2
   Compiling arrayref v0.3.9
   Compiling nightfall-types v1.1.6 (/data/data/com.termux/files/home/nightpool-pow-share-gate/upstream/crates/nightfall-types)
   Compiling chacha20poly1305 v0.10.1
   Compiling argon2 v0.5.3
   Compiling ed25519-dalek v2.2.0
   Compiling bulletproofs v5.0.0
   Compiling bip39 v2.2.2
   Compiling x25519-dalek v2.0.1
   Compiling nightfall-crypto v1.1.6 (/data/data/com.termux/files/home/nightpool-pow-share-gate/upstream/crates/nightfall-crypto)
   Compiling nightfall-ledger v1.1.6 (/data/data/com.termux/files/home/nightpool-pow-share-gate/upstream/crates/nightfall-ledger)
   Compiling nightfall-consensus v1.1.6 (/data/data/com.termux/files/home/nightpool-pow-share-gate/upstream/crates/nightfall-consensus)
    Finished `test` profile [optimized] target(s) in 5m 19s
     Running unittests examples/nightpool_share_gate.rs (target/debug/build/nightfall-consensus/250eab74852ffb8c/out/nightpool_share_gate-250eab74852ffb8c)

running 14 tests
test duplicate_cannot_receive_credit_twice ... ok
test fabricated_hash_rejected ... ok
test insufficient_work_rejected ... ok
test invalid_capacity_rejected ... ok
test mutated_body_root_rejected ... ok
test mutated_difficulty_rejected ... ok
test mutated_reward_rejected ... ok
test real_devnet_candidate_passes_consensus ... DEVNET_FULL_CONSENSUS_ACCEPTANCE=PASS
DEVNET_BLOCK_COUNT=1
ok
test share_target_above_block_target_rejected ... ok
test stale_template_rejected ... ok
test storage_capacity_is_bounded ... ok
test valid_share_receives_exact_work_credit ... ok
test zero_block_difficulty_rejected ... ok
test zero_share_difficulty_rejected ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.64s
