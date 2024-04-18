#![doc = include_str!("../../README.md")]
#![warn(missing_docs, unreachable_pub, rust_2021_compatibility)]
#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

mod arithmetic;
#[cfg(any(feature = "std", feature = "erc20"))]
pub mod erc20;
#[cfg(any(feature = "std", feature = "erc721"))]
pub mod erc721;

#[cfg(not(any(feature = "std", target_arch = "wasm32-unknown-unknown")))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
