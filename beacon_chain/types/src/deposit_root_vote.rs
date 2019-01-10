use super::Hash256;
use crate::test_utils::TestRandom;
use rand::RngCore;
use ssz::{Decodable, DecodeError, Encodable, SszStream};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DepositRootVote {
    deposit_root: Hash256,
    vote_count: u64,
}

impl Encodable for DepositRootVote {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append(&self.deposit_root);
        s.append(&self.vote_count);
    }
}

impl Decodable for DepositRootVote {
    fn ssz_decode(bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        let (deposit_root, i) = <_>::ssz_decode(bytes, i)?;
        let (vote_count, i) = <_>::ssz_decode(bytes, i)?;

        Ok((
            Self {
                deposit_root,
                vote_count,
            },
            i,
        ))
    }
}

impl<T: RngCore> TestRandom<T> for DepositRootVote {
    fn random_for_test(rng: &mut T) -> Self {
        Self {
            deposit_root: <_>::random_for_test(rng),
            vote_count: <_>::random_for_test(rng),
        }
    }
}
