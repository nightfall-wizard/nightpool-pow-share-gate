use std::collections::HashSet;

use nightfall_consensus::{BlockHeader, Chain};

use nightfall_crypto::{
    meets_difficulty,
    WalletKeys,
};

use nightfall_types::{
    Hash256,
    NetworkId,
};

#[derive(Debug, PartialEq, Eq)]
enum Reject {
    InvalidDifficulty,
    InvalidCapacity,
    StaleTemplate,
    WrongTemplate,
    DuplicateShare,
    CapacityReached,
    ForgedHash,
    InsufficientWork,
}

#[derive(Debug, PartialEq, Eq)]
struct Accepted {
    credited_work: u64,
    block_candidate: bool,
}

struct ShareGate {
    header: BlockHeader,
    share_difficulty: u64,
    accepted: HashSet<u64>,
    capacity: usize,
}

impl ShareGate {
    fn new(
        header: BlockHeader,
        share_difficulty: u64,
        capacity: usize,
    ) -> Result<Self, Reject> {

        if share_difficulty == 0
            || header.difficulty == 0
            || share_difficulty > header.difficulty
        {
            return Err(Reject::InvalidDifficulty);
        }

        if capacity == 0 || capacity > 65536 {
            return Err(Reject::InvalidCapacity);
        }

        Ok(Self {
            header,
            share_difficulty,
            accepted: HashSet::new(),
            capacity,
        })
    }

    fn verify(
        &mut self,
        submitted: &BlockHeader,
        claimed_hash: Hash256,
        current_tip: Hash256,
    ) -> Result<Accepted, Reject> {

        if current_tip != self.header.prev_hash {
            return Err(Reject::StaleTemplate);
        }

        if submitted.pow_preimage()
            != self.header.pow_preimage()
        {
            return Err(Reject::WrongTemplate);
        }

        if self.accepted.contains(&submitted.nonce) {
            return Err(Reject::DuplicateShare);
        }

        if self.accepted.len() >= self.capacity {
            return Err(Reject::CapacityReached);
        }

        // Network parameters are local policy.
        // They are never accepted from a submission.
        let params = NetworkId::Devnet.pow_params();

        // Recompute using upstream canonical Nighthash.
        let actual = submitted.pow_hash(params);

        if actual != claimed_hash {
            return Err(Reject::ForgedHash);
        }

        if !meets_difficulty(
            actual,
            self.share_difficulty,
        ) {
            return Err(Reject::InsufficientWork);
        }

        let block_candidate = meets_difficulty(
            actual,
            self.header.difficulty,
        );

        self.accepted.insert(submitted.nonce);

        Ok(Accepted {
            credited_work: self.share_difficulty,
            block_candidate,
        })
    }
}

const SHARE_DIFFICULTY: u64 = 4;
const NOW: u64 = 1_800_000_000;

fn test_chain() -> Chain {
    Chain::new_fair(NetworkId::Devnet)
        .expect("Devnet genesis")
}

fn test_header() -> BlockHeader {
    let chain = test_chain();

    let address = WalletKeys::from_seed([7u8; 32])
        .address();

    chain.build_template(
        &address,
        Vec::new(),
        NOW,
    )
    .expect("Devnet template")
    .header
}

fn find_share(
    header: &BlockHeader,
    kind: u8,
    start: u64,
) -> (BlockHeader, Hash256) {

    let params = NetworkId::Devnet.pow_params();

    for offset in 0..8192u64 {
        let mut h = header.clone();
        h.nonce = start + offset;

        let hash = h.pow_hash(params);

        let share = meets_difficulty(
            hash,
            SHARE_DIFFICULTY,
        );

        let block = meets_difficulty(
            hash,
            h.difficulty,
        );

        let matched = match kind {
            0 => !share,
            1 => share && !block,
            2 => block,
            _ => false,
        };

        if matched {
            return (h, hash);
        }
    }

    panic!("Bounded test nonce search exhausted")
}

fn gate(header: &BlockHeader) -> ShareGate {
    ShareGate::new(
        header.clone(),
        SHARE_DIFFICULTY,
        128,
    )
    .expect("Valid test policy")
}

#[test]
fn zero_share_difficulty_rejected() {
    let h = test_header();

    assert!(matches!(
        ShareGate::new(h, 0, 128),
        Err(Reject::InvalidDifficulty)
    ));
}

#[test]
fn zero_block_difficulty_rejected() {
    let mut h = test_header();
    h.difficulty = 0;

    assert!(matches!(
        ShareGate::new(h, 1, 128),
        Err(Reject::InvalidDifficulty)
    ));
}

#[test]
fn share_target_above_block_target_rejected() {
    let h = test_header();
    let d = h.difficulty + 1;

    assert!(matches!(
        ShareGate::new(h, d, 128),
        Err(Reject::InvalidDifficulty)
    ));
}

#[test]
fn invalid_capacity_rejected() {
    let h = test_header();

    assert!(matches!(
        ShareGate::new(
            h.clone(),
            SHARE_DIFFICULTY,
            0,
        ),
        Err(Reject::InvalidCapacity)
    ));

    assert!(matches!(
        ShareGate::new(
            h,
            SHARE_DIFFICULTY,
            65537,
        ),
        Err(Reject::InvalidCapacity)
    ));
}

#[test]
fn valid_share_receives_exact_work_credit() {
    let h = test_header();
    let mut g = gate(&h);
    let (s, hash) = find_share(&h, 1, 0);

    assert_eq!(
        g.verify(&s, hash, h.prev_hash),
        Ok(Accepted {
            credited_work: SHARE_DIFFICULTY,
            block_candidate: false,
        })
    );
}

#[test]
fn duplicate_cannot_receive_credit_twice() {
    let h = test_header();
    let mut g = gate(&h);
    let (s, hash) = find_share(&h, 1, 0);

    assert!(g.verify(
        &s, hash, h.prev_hash
    ).is_ok());

    assert_eq!(
        g.verify(&s, hash, h.prev_hash),
        Err(Reject::DuplicateShare)
    );

    assert_eq!(g.accepted.len(), 1);
}

#[test]
fn insufficient_work_rejected() {
    let h = test_header();
    let mut g = gate(&h);
    let (s, hash) = find_share(&h, 0, 0);

    assert_eq!(
        g.verify(&s, hash, h.prev_hash),
        Err(Reject::InsufficientWork)
    );

    assert!(g.accepted.is_empty());
}

#[test]
fn fabricated_hash_rejected() {
    let h = test_header();
    let mut g = gate(&h);
    let (s, hash) = find_share(&h, 1, 0);

    let mut forged = hash;
    forged.0[0] ^= 1;

    assert_eq!(
        g.verify(&s, forged, h.prev_hash),
        Err(Reject::ForgedHash)
    );

    assert!(g.accepted.is_empty());

    assert!(g.verify(
        &s, hash, h.prev_hash
    ).is_ok());
}

#[test]
fn mutated_reward_rejected() {
    let h = test_header();
    let mut g = gate(&h);
    let mut changed = h.clone();

    changed.reward_darks ^= 1;

    assert_eq!(
        g.verify(
            &changed,
            Hash256::ZERO,
            h.prev_hash,
        ),
        Err(Reject::WrongTemplate)
    );
}

#[test]
fn mutated_difficulty_rejected() {
    let h = test_header();
    let mut g = gate(&h);
    let mut changed = h.clone();

    changed.difficulty = 1;

    assert_eq!(
        g.verify(
            &changed,
            Hash256::ZERO,
            h.prev_hash,
        ),
        Err(Reject::WrongTemplate)
    );
}

#[test]
fn mutated_body_root_rejected() {
    let h = test_header();
    let mut g = gate(&h);
    let mut changed = h.clone();

    changed.body_root.0[0] ^= 1;

    assert_eq!(
        g.verify(
            &changed,
            Hash256::ZERO,
            h.prev_hash,
        ),
        Err(Reject::WrongTemplate)
    );
}

#[test]
fn stale_template_rejected() {
    let h = test_header();
    let mut g = gate(&h);
    let mut other_tip = h.prev_hash;

    other_tip.0[0] ^= 1;

    assert_eq!(
        g.verify(
            &h,
            Hash256::ZERO,
            other_tip,
        ),
        Err(Reject::StaleTemplate)
    );
}

#[test]
fn storage_capacity_is_bounded() {
    let h = test_header();

    let mut g = ShareGate::new(
        h.clone(),
        SHARE_DIFFICULTY,
        1,
    ).unwrap();

    let (a, ha) = find_share(&h, 1, 0);

    let (b, hb) = find_share(
        &h,
        1,
        a.nonce + 1,
    );

    assert!(g.verify(
        &a, ha, h.prev_hash
    ).is_ok());

    assert_eq!(
        g.verify(&b, hb, h.prev_hash),
        Err(Reject::CapacityReached)
    );

    assert_eq!(g.accepted.len(), 1);
}

#[test]
fn real_devnet_candidate_passes_consensus() {
    let mut chain = test_chain();

    let address = WalletKeys::from_seed(
        [11u8; 32]
    ).address();

    let template = chain.build_template(
        &address,
        Vec::new(),
        NOW,
    ).unwrap();

    let mut g = gate(&template.header);

    let (header, hash) = find_share(
        &template.header,
        2,
        0,
    );

    assert_eq!(
        g.verify(
            &header,
            hash,
            chain.tip_hash(),
        ),
        Ok(Accepted {
            credited_work: SHARE_DIFFICULTY,
            block_candidate: true,
        })
    );

    let block = template.seal(header.nonce);

    assert!(block.pow_is_valid(
        NetworkId::Devnet.pow_params()
    ));

    chain.apply_block(block, NOW)
        .expect("Full Devnet consensus acceptance");

    assert_eq!(chain.block_count(), 1);

    assert_eq!(
        g.verify(
            &header,
            hash,
            chain.tip_hash(),
        ),
        Err(Reject::StaleTemplate)
    );

    println!("DEVNET_FULL_CONSENSUS_ACCEPTANCE=PASS");
    println!("DEVNET_BLOCK_COUNT={}", chain.block_count());
}

fn main() {
    println!(
        "Research harness. Run with cargo test --example."
    );
}
