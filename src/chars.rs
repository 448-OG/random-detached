use rand_chacha::{ChaCha12Rng, ChaCha20Rng, rand_core::SeedableRng};
use zeroize::{Zeroize, Zeroizing};

pub const ALPHABET: [char; 36] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub const ALPHABET_SAFE: [char; 33] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub struct RandomChars<const N: usize>(pub Zeroizing<[char; N]>);

impl<const N: usize> RandomChars<N> {
    pub fn new() -> Result<RandomChars<N>, getrandom::Error> {
        Ok(Self(Self::generate_chars(false)?))
    }
    pub fn new_safe_display() -> Result<RandomChars<N>, getrandom::Error> {
        Ok(Self(Self::generate_chars(true)?))
    }

    pub fn to_string(&self) -> Zeroizing<String> {
        let mut outcome = Zeroizing::new(String::with_capacity(N));

        self.0.into_iter().for_each(|char| {
            outcome.push(char);
        });

        outcome
    }

    #[cfg(all(feature = "const_cmp", feature = "chars"))]
    pub fn chars_const_cmp(&self, other: &Self) -> bool {
        use subtle::ConstantTimeEq;

        let current = self.0.iter().collect::<String>().to_string();
        let other = other.0.iter().collect::<String>().to_string();

        current.as_bytes().ct_eq(other.as_bytes()).into()
    }

    pub fn generate_chars(safe_display: bool) -> Result<Zeroizing<[char; N]>, getrandom::Error> {
        let mut seed = [0u8; 32];
        getrandom::fill(seed.as_mut())?;

        let mut buffer = Zeroizing::new(['\0'; N]);
        let rng = ChaCha20Rng::from_seed(seed);

        Self::fill_with_chars(&mut buffer, rng, safe_display);

        Ok(buffer)
    }

    pub fn generate_chars_with_chacha12(
        safe_display: bool,
    ) -> Result<Zeroizing<[char; N]>, getrandom::Error> {
        let mut seed = [0u8; 32];
        getrandom::fill(seed.as_mut())?;

        let mut buffer = Zeroizing::new(['\0'; N]);
        let rng = ChaCha12Rng::from_seed(seed);

        Self::fill_with_chars(&mut buffer, rng, safe_display);

        Ok(buffer)
    }

    pub fn fill_with_chars(
        buffer: &mut Zeroizing<[char; N]>,
        mut rng: impl rand_chacha::rand_core::CryptoRng,
        safe_display: bool,
    ) {
        for c in buffer.iter_mut() {
            // Generate a random index between 0 and 35
            let idx = (rng.next_u32() % 36) as usize;
            if safe_display {
                *c = ALPHABET_SAFE[idx];
            } else {
                *c = ALPHABET[idx];
            }
        }
    }

    pub fn zeroize(mut self) -> Self {
        self.0.zeroize();

        self
    }
}

#[cfg(test)]
mod sanity_checks {
    use crate::RandomChars;

    #[test]
    fn generate_chars() {
        let chars = RandomChars::<10>::new().unwrap();
        assert_eq!(chars.0.len(), 10);
        assert_ne!(*chars.0, ['\0'; 10]);

        let chars = chars.zeroize();
        assert_eq!(*(chars).0, ['\0'; 10]);
        assert_eq!(*chars.to_string(), ['\0'; 10].iter().collect::<String>());
    }
}
