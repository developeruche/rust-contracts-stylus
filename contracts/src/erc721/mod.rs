pub mod base;
pub mod extensions;

use alloy_sol_types::{SolError, SolType};
use base::{
    ERC721IncorrectOwner, ERC721InsufficientApproval, ERC721InvalidApprover,
    ERC721InvalidOperator, ERC721InvalidOwner, ERC721InvalidReceiver,
    ERC721InvalidSender, ERC721NonexistentToken,
};
use stylus_sdk::{call::MethodError, prelude::*};
use crate::utils::pausable::{EnforcedPause, ExpectedPause};

/// An ERC-721 error defined as described in [ERC-6093].
///
/// [ERC-6093]: https://eips.ethereum.org/EIPS/eip-6093
#[derive(SolidityError, Debug)]
pub enum Error {
    /// Indicates that an address can't be an owner.
    /// For example, `address(0)` is a forbidden owner in ERC-721. Used in
    /// balance queries.
    InvalidOwner(ERC721InvalidOwner),
    /// Indicates a `tokenId` whose `owner` is the zero address.
    NonexistentToken(ERC721NonexistentToken),
    /// Indicates an error related to the ownership over a particular token.
    /// Used in transfers.
    IncorrectOwner(ERC721IncorrectOwner),
    /// Indicates a failure with the token `sender`. Used in transfers.
    InvalidSender(ERC721InvalidSender),
    /// Indicates a failure with the token `receiver`. Used in transfers.
    InvalidReceiver(ERC721InvalidReceiver),
    /// Indicates a failure with the `operator`’s approval. Used in transfers.
    InsufficientApproval(ERC721InsufficientApproval),
    /// Indicates a failure with the `approver` of a token to be approved. Used
    /// in approvals.
    InvalidApprover(ERC721InvalidApprover),
    /// Indicates a failure with the `operator` to be approved. Used in
    /// approvals.
    InvalidOperator(ERC721InvalidOperator),
    EnforcedPause(EnforcedPause),
    ExpectedPause(ExpectedPause),
    /// Let to return custom user error from overridden function
    Custom(ERC721CustomError),
}

#[derive(Debug)]
pub struct ERC721CustomError(alloc::vec::Vec<u8>);

impl MethodError for ERC721CustomError {
    fn encode(self) -> alloc::vec::Vec<u8> {
        self.0
    }
}

impl<T: SolError> From<T> for ERC721CustomError {
    fn from(value: T) -> Self {
        ERC721CustomError(value.encode())
    }
}

#[cfg(all(test, feature = "std"))]
pub(crate) mod tests {
    use core::marker::PhantomData;

    use alloy_primitives::U256;
    use contracts_proc::inherit;
    use stylus_sdk::storage::{StorageBool, StorageMap};

    use super::*;
    use crate::erc721::{
        base::{ERC721Base, ERC721BaseOverride},
        extensions::{
            burnable::{ERC721Burnable, ERC721BurnableOverride},
            pausable::{ERC721Pausable, ERC721PausableOverride},
        },
    };
    use crate::utils::pausable::Pausable;

    pub(crate) type ERC721Override = inherit!(
        ERC721BurnableOverride,
        ERC721PausableOverride,
        ERC721BaseOverride
    );

    sol_storage! {
        pub struct ERC721 {
            ERC721Base<ERC721Override> erc721;
            ERC721Burnable<ERC721Override> burnable;
            ERC721Pausable<ERC721Override> pausable;
        }
    }

    #[external]
    #[inherit(ERC721Burnable<ERC721Override>)]
    #[inherit(ERC721Pausable<ERC721Override>)]
    #[inherit(ERC721Base<ERC721Override>)]
    impl ERC721 {}

    unsafe impl TopLevelStorage for ERC721 {}

    impl Default for ERC721 {
        fn default() -> Self {
            let root = U256::ZERO;

            ERC721 {
                erc721: ERC721Base {
                    _owners: unsafe { StorageMap::new(root, 0) },
                    _balances: unsafe {
                        StorageMap::new(root + U256::from(32), 0)
                    },
                    _token_approvals: unsafe {
                        StorageMap::new(root + U256::from(64), 0)
                    },
                    _operator_approvals: unsafe {
                        StorageMap::new(root + U256::from(96), 0)
                    },
                    _phantom_data: PhantomData,
                },
                burnable: ERC721Burnable { _phantom_data: PhantomData },
                pausable: ERC721Pausable {
                    pausable: Pausable {
                        _paused: unsafe {
                            // TODO: what should be size of bool with alignment?
                            StorageBool::new(root + U256::from(128), 0)
                        },
                    },
                    _phantom_data: PhantomData,
                },
            }
        }
    }

    pub(crate) fn random_token_id() -> U256 {
        let num: u32 = rand::random();
        num.try_into().expect("conversion to U256")
    }
}
