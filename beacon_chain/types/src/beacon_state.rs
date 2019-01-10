use super::crosslink_record::CrosslinkRecord;
use super::custody_challenge::CustodyChallenge;
use super::deposit_root_vote::DepositRootVote;
use super::fork_data::ForkData;
use super::pending_attestation_record::PendingAttestationRecord;
use super::shard_committee::ShardCommittee;
use super::validator_record::ValidatorRecord;
use super::Hash256;
use crate::test_utils::TestRandom;
use hashing::canonical_hash;
use rand::RngCore;
use ssz::{ssz_encode, Decodable, DecodeError, Encodable, SszStream};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct BeaconState {
    // Misc
    pub slot: u64,
    pub genesis_time: u64,
    pub fork_data: ForkData,

    // Validator registry
    pub validator_registry: Vec<ValidatorRecord>,
    pub validator_balances: Vec<u64>,
    pub validator_registry_latest_change_slot: u64,
    pub validator_registry_exit_count: u64,
    pub validator_registry_delta_chain_tip: Hash256,

    // Randomness and committees
    pub latest_randao_mixes: Vec<Hash256>,
    pub latest_vdf_outputs: Vec<Hash256>,
    pub shard_committees_at_slots: Vec<Vec<ShardCommittee>>,

    // Custody challenges
    pub custody_challenges: Vec<CustodyChallenge>,

    // Finality
    pub previous_justified_slot: u64,
    pub justified_slot: u64,
    pub justification_bitfield: u64,
    pub finalized_slot: u64,

    // Recent state
    pub latest_crosslinks: Vec<CrosslinkRecord>,
    pub latest_block_roots: Vec<Hash256>,
    pub latest_penalized_exit_balances: Vec<u64>,
    pub latest_attestations: Vec<PendingAttestationRecord>,
    pub batched_block_roots: Vec<Hash256>,

    // Ethereum 1.0 deposit root
    pub latest_deposit_root: Hash256,
    pub deposit_root_votes: Vec<DepositRootVote>,
}

impl BeaconState {
    pub fn canonical_root(&self) -> Hash256 {
        // TODO: implement tree hashing.
        // https://github.com/sigp/lighthouse/issues/70
        Hash256::from(&canonical_hash(&ssz_encode(self))[..])
    }

    /// Returns the `ShardCommittee` for the `slot`.
    pub fn get_shard_committees_at_slot(
        &self,
        slot: u64,
        epoch_length: u64,
    ) -> Option<&Vec<ShardCommittee>> {
        let earliest_slot_in_array = self.slot - (self.slot % epoch_length) - epoch_length;
        if earliest_slot_in_array <= slot && slot < earliest_slot_in_array + epoch_length * 2 {
            let index = (slot - earliest_slot_in_array) as usize;
            self.shard_committees_at_slots.get(index)
        } else {
            None
        }
    }

    /// Returns the beacon proposer index for the `slot`.
    pub fn get_beacon_proposer_index(&self, slot: u64, epoch_length: u64) -> Option<usize> {
        self.get_shard_committees_at_slot(slot, epoch_length)
            .and_then(|shard_committees| shard_committees.get(0))
            .and_then(|shard_committee| {
                let first_committee = &shard_committee.committee;
                let target_index = slot as usize % first_committee.len();
                first_committee.get(target_index).cloned()
            })
    }
}

impl Encodable for BeaconState {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append(&self.slot);
        s.append(&self.genesis_time);
        s.append(&self.fork_data);
        s.append(&self.validator_registry);
        s.append(&self.validator_balances);
        s.append(&self.validator_registry_latest_change_slot);
        s.append(&self.validator_registry_exit_count);
        s.append(&self.validator_registry_delta_chain_tip);
        s.append(&self.latest_randao_mixes);
        s.append(&self.latest_vdf_outputs);
        s.append(&self.custody_challenges);
        s.append(&self.previous_justified_slot);
        s.append(&self.justified_slot);
        s.append(&self.justification_bitfield);
        s.append(&self.finalized_slot);
        s.append(&self.latest_crosslinks);
        s.append(&self.latest_block_roots);
        s.append(&self.latest_penalized_exit_balances);
        s.append(&self.latest_attestations);
        s.append(&self.batched_block_roots);
        s.append(&self.latest_deposit_root);
        s.append(&self.deposit_root_votes);
    }
}

impl Decodable for BeaconState {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (slot, i) = <_>::ssz_decode(bytes, i)?;
        let (genesis_time, i) = <_>::ssz_decode(bytes, i)?;
        let (fork_data, i) = <_>::ssz_decode(bytes, i)?;
        let (validator_registry, i) = <_>::ssz_decode(bytes, i)?;
        let (validator_balances, i) = <_>::ssz_decode(bytes, i)?;
        let (validator_registry_latest_change_slot, i) = <_>::ssz_decode(bytes, i)?;
        let (validator_registry_exit_count, i) = <_>::ssz_decode(bytes, i)?;
        let (validator_registry_delta_chain_tip, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_randao_mixes, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_vdf_outputs, i) = <_>::ssz_decode(bytes, i)?;
        let (shard_committees_at_slots, i) = <_>::ssz_decode(bytes, i)?;
        let (custody_challenges, i) = <_>::ssz_decode(bytes, i)?;
        let (previous_justified_slot, i) = <_>::ssz_decode(bytes, i)?;
        let (justified_slot, i) = <_>::ssz_decode(bytes, i)?;
        let (justification_bitfield, i) = <_>::ssz_decode(bytes, i)?;
        let (finalized_slot, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_crosslinks, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_block_roots, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_penalized_exit_balances, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_attestations, i) = <_>::ssz_decode(bytes, i)?;
        let (batched_block_roots, i) = <_>::ssz_decode(bytes, i)?;
        let (latest_deposit_root, i) = <_>::ssz_decode(bytes, i)?;
        let (deposit_root_votes, i) = <_>::ssz_decode(bytes, i)?;

        Ok((
            Self {
                slot,
                genesis_time,
                fork_data,
                validator_registry,
                validator_balances,
                validator_registry_latest_change_slot,
                validator_registry_exit_count,
                validator_registry_delta_chain_tip,
                latest_randao_mixes,
                latest_vdf_outputs,
                shard_committees_at_slots,
                custody_challenges,
                previous_justified_slot,
                justified_slot,
                justification_bitfield,
                finalized_slot,
                latest_crosslinks,
                latest_block_roots,
                latest_penalized_exit_balances,
                latest_attestations,
                batched_block_roots,
                latest_deposit_root,
                deposit_root_votes,
            },
            i,
        ))
    }
}

impl<T: RngCore> TestRandom<T> for BeaconState {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            slot: <_>::random_for_test(rng),
            genesis_time: <_>::random_for_test(rng),
            fork_data: <_>::random_for_test(rng),
            validator_registry: <_>::random_for_test(rng),
            validator_balances: <_>::random_for_test(rng),
            validator_registry_latest_change_slot: <_>::random_for_test(rng),
            validator_registry_exit_count: <_>::random_for_test(rng),
            validator_registry_delta_chain_tip: <_>::random_for_test(rng),
            latest_randao_mixes: <_>::random_for_test(rng),
            latest_vdf_outputs: <_>::random_for_test(rng),
            shard_committees_at_slots: <_>::random_for_test(rng),
            custody_challenges: <_>::random_for_test(rng),
            previous_justified_slot: <_>::random_for_test(rng),
            justified_slot: <_>::random_for_test(rng),
            justification_bitfield: <_>::random_for_test(rng),
            finalized_slot: <_>::random_for_test(rng),
            latest_crosslinks: <_>::random_for_test(rng),
            latest_block_roots: <_>::random_for_test(rng),
            latest_penalized_exit_balances: <_>::random_for_test(rng),
            latest_attestations: <_>::random_for_test(rng),
            batched_block_roots: <_>::random_for_test(rng),
            latest_deposit_root: <_>::random_for_test(rng),
            deposit_root_votes: <_>::random_for_test(rng),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ssz::ssz_encode;
    use super::*;
    use crate::test_utils::{SeedableRng, TestRandom, XorShiftRng};

    #[test]
    pub fn test_ssz_round_trip() {
        let mut rng = XorShiftRng::from_seed([42; 16]);
        let original = BeaconState::random_for_test(&mut rng);

        let bytes = ssz_encode(&original);
        let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

        assert_eq!(original, decoded);
    }
}
