pub mod base;
pub mod extensions;

use base::{
    ERC721IncorrectOwner, ERC721InsufficientApproval, ERC721InvalidApprover,
    ERC721InvalidOperator, ERC721InvalidOwner, ERC721InvalidReceiver,
    ERC721InvalidSender, ERC721NonexistentToken,
};
use extensions::pausable::{EnforcedPause, ExpectedPause};
use stylus_sdk::prelude::*;

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
}

#[cfg(all(test, feature = "std"))]
pub(crate) mod tests {
    use core::marker::PhantomData;

    use alloy_primitives::U256;
    use stylus_sdk::storage::{StorageBool, StorageMap};

    use super::*;
    use crate::erc721::{
        base::{ERC721Base, ERC721BaseOverride},
        extensions::{
            burnable::{ERC721Burnable, ERC721BurnableOverride},
            pausable::{ERC721Pausable, ERC721PausableOverride},
        },
    };

    pub(crate) type ERC721Override =
        ERC721BurnableOverride<ERC721PausableOverride<ERC721BaseOverride>>;

    sol_storage! {
        pub struct ERC721 {
            #[borrow]
            ERC721Base<ERC721Override> erc721;
            #[borrow]
            ERC721Burnable<ERC721Override> burnable;
            #[borrow]
            ERC721Pausable<ERC721Override> pausable;
        }
    }

    #[external]
    #[inherit(ERC721Burnable<ERC721Override>)]
    #[inherit(ERC721Pausable<ERC721Override>)]
    #[inherit(ERC721Base<ERC721Override>)]
    impl ERC721 {}

    unsafe impl TopLevelStorage for ERC721 {
        fn get_storage<T: 'static>(&mut self) -> &mut T {
            use core::any::{Any, TypeId};
            if TypeId::of::<T>() == TypeId::of::<Self>() {
                unsafe { core::mem::transmute::<_, _>(self) }
            } else if TypeId::of::<T>() == self.erc721.type_id() {
                unsafe { core::mem::transmute::<_, _>(&mut self.erc721) }
            } else if TypeId::of::<T>() == self.pausable.type_id() {
                unsafe { core::mem::transmute::<_, _>(&mut self.pausable) }
            } else if TypeId::of::<T>() == self.burnable.type_id() {
                unsafe { core::mem::transmute::<_, _>(&mut self.burnable) }
            } else {
                panic!(
                    "storage for type doesn't exist - type name is {}",
                    core::any::type_name::<T>()
                )
            }
        }
    }

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
                    phantom_data: PhantomData,
                },
                burnable: ERC721Burnable { phantom_data: PhantomData },
                pausable: ERC721Pausable {
                    paused: unsafe {
                        // TODO: what should be size of bool with alignment?
                        StorageBool::new(root + U256::from(128), 0)
                    },
                    phantom_data: PhantomData,
                },
            }
        }
    }

    pub(crate) fn random_token_id() -> U256 {
        let num: u32 = rand::random();
        num.try_into().expect("conversion to U256")
    }
}
