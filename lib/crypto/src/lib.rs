/*!
Common cryptographic procedures for a blockchain environment.

> Note that `crypto` is still `0.*.*`, so breaking changes
> [may occur at any time](https://semver.org/#spec-item-4). If you must depend
> on `crypto`, we recommend pinning to a specific version, i.e., `=0.y.z`.

## Verifying Merkle Proofs

[`merkle.rs`](./src/merkle.rs) provides:

- A `verify` function which can prove that some value is part of a
  [Merkle tree].
- A `verify_multi_proof` function which can prove multiple values are part of a
  [Merkle tree].

[Merkle tree]: https://en.wikipedia.org/wiki/Merkle_tree

*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

pub mod hash;
pub mod ieip712;
pub mod merkle;
pub mod message_hash_utils;

pub mod keccak;
pub use ieip712::IEIP712;
pub use keccak::KeccakBuilder;

/// Type representing Solidity `bytes32`.
pub type Bytes1 = [u8; 1];

/// Type representing Solidity `bytes32`.
pub type Bytes32 = [u8; 32];

/// Type representing Solidity `Address`.
pub type Address = [u8; 20];
