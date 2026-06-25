# RANDOM-DETACHED

A random byte and char generator that dosen't keep breaking because different project use different verfsions of Rand.

The generation of random bytes is fast enough and fills the destination array in one syscall.

The generation of random chars uses chacha20/chacha12 algorithms seeded once using the OS entropy. This ensure higher speeds since seeding from OS is slower due to repeated syscalls.

## Usage

Add to project in `Cargo.toml` manifest

```toml
[dependencies]
random-detached = {version = "<recent version here>"}
```

### Generate random bytes

The `bytes` feature enables the default random bytes generator that implements the `Generator` trait. This feature is enabled by default.

```rust
#[cfg(feature = "bytes")]
pub fn test_bytes() {
    use random_detached::{Generator, RandomBytes};

    // Here `32` in  `RandomChars::<32>::` is specifies as the number of randon bytes to be generated
    let mut bytes = RandomBytes::<32>::new().unwrap();
    assert_eq!(bytes.expose().len(), 32);
    assert_ne!(*bytes.expose(), [0u8; 32]);

    bytes.zeroize_mem();
    assert_eq!(*(bytes.take()), [0u8; 32]);
}
```

### Generate random chars

Add to project in `Cargo.toml` manifest and enable `chars` feature

```toml
[dependencies]
random-detached = {version = "<recent version here>", features = ["chars"]}
```

```rust
#[cfg(feature = "foo")]
pub fn test_chars() {
    use random_detached::RandomChars;

    // Here `10` in  `RandomChars::<10>::` is specifies as the number of chars to be generated
    let chars = RandomChars::<10>::new().unwrap();

    // Convert the chars to one String (implements Zeroize)
    let random_string = chars.to_string();

    assert_eq!(chars.0.len(), 10);
    assert_ne!(*chars.0, ['\0'; 10]);


    let chars = chars.zeroize();
    assert_eq!(*(chars).0, ['\0'; 10]);
}
```

## License

Apache-2.0

## Caution

This crate depends on the correctness of `rand_chacha` and `getrandom` crates. Use at your own risk.
