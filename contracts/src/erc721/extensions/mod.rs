cfg_if::cfg_if! {
    if #[cfg(any(test, feature = "erc721_burnable"))] {
        pub mod burnable;
        pub use burnable::ERC721Burnable;
        pub use burnable::ERC721BurnableOverride;
    }
}
cfg_if::cfg_if! {
    if #[cfg(any(test, feature = "erc721_pausable"))] {
        pub mod pausable;
        pub use pausable::ERC721Pausable;
        pub use pausable::ERC721PausableOverride;
    }
}