use zeroize::{Zeroize, Zeroizing};

#[cfg(feature = "const_cmp")]
use subtle::ConstantTimeEq;

use crate::Generator;

pub struct RandomBytes<const N: usize>(Zeroizing<[u8; N]>);

impl<const N: usize> Generator<N> for RandomBytes<N> {
    fn new() -> Result<Self, getrandom::Error> {
        Ok(Self(Self::generate()?))
    }

    fn expose(&self) -> &[u8; N] {
        &self.0
    }

    fn take(self) -> Zeroizing<[u8; N]> {
        self.0
    }

    fn zeroize_mem(&mut self) {
        self.0.zeroize();
    }

    #[cfg(feature = "blake3_hash")]
    fn hash(&self) -> blake3::Hash {
        blake3::hash(self.expose())
    }

    #[cfg(feature = "const_cmp")]
    fn const_cmp(&self, other: &Self) -> bool {
        self.0.as_ref().ct_eq(other.0.as_ref()).into()
    }
}

#[cfg(test)]
mod sanity_checks {
    use crate::{Generator, RandomBytes};

    #[test]
    fn generate_bytes() {
        let mut bytes = RandomBytes::<32>::new().unwrap();
        assert_eq!(bytes.0.len(), 32);
        assert_ne!(*bytes.expose(), [0u8; 32]);

        bytes.zeroize_mem();
        assert_eq!(*(bytes.take()), [0u8; 32]);
    }
}
