use std::io::{Cursor, Read};

use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::BlockId;
use brotli2::bufread::BrotliDecoder;
use revm_primitives::{Address, Bytes, Env, PrecompileError, PrecompileResult};

// use stylus::native::activate;
use tokio::{runtime::Handle, task::block_in_place};

const ARBWASM_ACTIVATE_METHOD_HASH: [u8; 4] = [88, 199, 128, 194];
const STYLUS_VERSION: u16 = 1;
const PAGE_LIMIT: u16 = 128;

pub fn arb_wasm(
    bytes: &Bytes,
    _gas_price: u64,
    _env: &Env,
) -> PrecompileResult {
    let selector = &bytes[..4];
    // We only support activation calls for now.
    if selector != ARBWASM_ACTIVATE_METHOD_HASH {
        return Err(PrecompileError::other("invalid function selector"));
    }

    let rpc_url = "http://localhost:8545".parse().unwrap();
    let provider = ProviderBuilder::new()
        .on_hyper_http(rpc_url)
        .map_err(|e| PrecompileError::other(format!("{e:#?}")))?;

    let addr = Address::from_slice(&bytes[16..]);
    let compressed_wasm = block_in_place(|| {
        Handle::current()
            .block_on(provider.get_code_at(addr, BlockId::latest()))
    })
    .map_err(|e| PrecompileError::other(format!("{e}")))?;

    // Strip Stylus prefix (`0xeff000`).
    let compressed_wasm: Bytes = compressed_wasm.into_iter().skip(3).collect();
    let stream = Cursor::new(compressed_wasm.to_vec());
    let mut decoder = BrotliDecoder::new(stream);
    let mut wasm = Vec::new();
    decoder.read_to_end(&mut wasm).unwrap();

    // let (wasm, _, _) =
    //     activate(&wasm, STYLUS_VERSION, PAGE_LIMIT, false, &mut u64::MAX)
    //         .unwrap();
    //
    // println!("{:?}", Bytes::from(wasm.clone()));

    Ok((0, Bytes::from(wasm)))
}
