use zeroize::Zeroizing;

pub trait Generator<const N: usize>: Sized {
    fn new() -> Result<Self, getrandom::Error>;

    fn generate() -> Result<Zeroizing<[u8; N]>, getrandom::Error> {
        let mut buffer = Zeroizing::new([0u8; N]);

        getrandom::fill(buffer.as_mut())?;

        Ok(buffer)
    }

    fn expose(&self) -> &[u8; N];

    fn take(self) -> Zeroizing<[u8; N]>;

    fn zeroize_mem(&mut self);

    #[cfg(feature = "blake3_hash")]
    fn hash(&self) -> blake3::Hash;

    #[cfg(feature = "const_cmp")]
    fn const_cmp(&self, other: &Self) -> bool;
}
