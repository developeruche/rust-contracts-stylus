use crate::erc721::{
    base::{
        ERC721Base, ERC721BaseOverride, ERC721IncorrectOwner,
        ERC721InsufficientApproval, ERC721InvalidApprover,
        ERC721InvalidOperator, ERC721InvalidOwner, ERC721InvalidReceiver,
        ERC721InvalidSender, ERC721NonexistentToken, ERC721Virtual,
    },
    extensions::{
        burnable::{ERC721Burnable, ERC721BurnableOverride},
        pausable::{
            ERC721Pausable, ERC721PausableOverride, EnforcedPause,
            ExpectedPause,
        },
    },
};
use core::borrow::BorrowMut;
use stylus_sdk::prelude::*;

pub mod base;
pub mod extensions;

pub trait Storage<T: ERC721Virtual>:
    TopLevelStorage
    + BorrowMut<ERC721Burnable<T>>
    + BorrowMut<ERC721Pausable<T>>
    + BorrowMut<ERC721Base<T>>
{
    fn erc721_mut(&mut self) -> &mut ERC721Base<T> {
        self.borrow_mut()
    }

    fn erc721(&self) -> &ERC721Base<T> {
        self.borrow()
    }

    fn pausable_mut(&mut self) -> &mut ERC721Pausable<T> {
        self.borrow_mut()
    }

    fn pausable(&self) -> &ERC721Pausable<T> {
        self.borrow()
    }
}

type ERC721Override =
    ERC721BurnableOverride<ERC721PausableOverride<ERC721BaseOverride>>;

unsafe impl TopLevelStorage for ERC721 {}

impl Storage<ERC721Override> for ERC721 {}

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
#[restrict_storage_with(impl Storage<ERC721Override>)]
impl ERC721 {}

/// An ERC-721 error defined as described in [ERC-6093].
///
/// [ERC-6093]: https://eips.ethereum.org/EIPS/eip-6093
#[derive(SolidityError, Debug)]
pub enum Error {
    InvalidOwner(ERC721InvalidOwner),
    NonexistentToken(ERC721NonexistentToken),
    IncorrectOwner(ERC721IncorrectOwner),
    InvalidSender(ERC721InvalidSender),
    InvalidReceiver(ERC721InvalidReceiver),
    InsufficientApproval(ERC721InsufficientApproval),
    InvalidApprover(ERC721InvalidApprover),
    InvalidOperator(ERC721InvalidOperator),
    EnforcedPause(EnforcedPause),
    ExpectedPause(ExpectedPause),
}

#[cfg(test)]
pub(crate) mod tests {
    use core::marker::PhantomData;

    use alloy_primitives::U256;
    use stylus_sdk::storage::{StorageBool, StorageMap};

    use super::*;

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
