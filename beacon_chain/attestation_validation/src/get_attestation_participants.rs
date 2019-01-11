//! This implements [`get_attestation_participants`]
//! (https://github.com/ethereum/eth2.0-specs/blob/master/specs/core/0_beacon-chain.md#get_attestation_participants)

use failure::{Error, Fail};
use types::{AttestationData, BeaconState, Bitfield, ShardCommittee};

#[derive(Debug, Fail)]
/// An error type for `get_attestation_participants()` problems
pub enum AttestationParticipantError {
    #[fail(
        display = "Invalid participation_bitfield size: {}, expected {}",
        got, expected
    )]
    /// The participation bitfield was the wrong size
    InvalidBitfieldSize { expected: usize, got: usize },
    #[fail(display = "No shard committee found in attestation data shard {}", _0)]
    /// The attestation data shard doesn't match any committee's shard
    NoMatchingShard(u64),
}

use self::AttestationParticipantError::*;

fn dummy_get_shard_committee_at_slot(
    state: &BeaconState,
    slot: u64,
) -> Result<Vec<ShardCommittee>, Error> {
    Ok(vec![])
}

/// Get validator indices partaking in the attestation described by `attestation_data` and
/// `participation_bitfield`.
pub fn get_attestation_participants(
    state: &BeaconState,
    attestation_data: &AttestationData,
    participation_bitfield: &Bitfield,
) -> Result<Vec<usize>, Error> {
    // Find the relevant committee
    let shard_committees = dummy_get_shard_committee_at_slot(state, attestation_data.slot)?;
    let shard_committee = match shard_committees
        .iter()
        .filter(|committee| committee.shard == attestation_data.shard)
        .nth(0)
    {
        Some(committee) => committee,
        None => return Err(NoMatchingShard(attestation_data.shard).into()),
    };

    let desired_bitfield_len = (shard_committee.len() + 7) / 8;
    // The Bitfield type is bit-indexed while the spec assumes the byte-indexed Python bytes()
    if participation_bitfield.num_bytes() != desired_bitfield_len {
        return Err(InvalidBitfieldSize {
            expected: participation_bitfield.num_bytes(),
            got: desired_bitfield_len,
        }
        .into());
    }

    // Find the participating attesters in the committee
    let mut participants = vec![];
    for (i, validator_index) in shard_committee.committee.iter().enumerate() {
        // Again, we need `to_bytes()` because the spec assumes a byte-indexed vec
        let participation_bit = (participation_bitfield.to_bytes()[i / 8] >> (7 - (i % 8))) % 2;

        if participation_bit == 1 {
            participants.push(*validator_index);
        }
    }

    Ok(participants)
}
