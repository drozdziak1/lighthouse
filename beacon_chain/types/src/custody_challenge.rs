use crate::test_utils::TestRandom;
use rand::RngCore;
use ssz::{Decodable, DecodeError, Encodable, SszStream};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CustodyChallenge {}

impl Encodable for CustodyChallenge {
    fn ssz_append(&self, _s: &mut SszStream) {}
}

impl Decodable for CustodyChallenge {
    fn ssz_decode(_bytes: &[u8], i: usize) -> Result<(Self, usize), DecodeError> {
        Ok((CustodyChallenge {}, i))
    }
}

impl<T: RngCore> TestRandom<T> for CustodyChallenge {
    fn random_for_test(_rng: &mut T) -> Self {
        Self {}
    }
}
