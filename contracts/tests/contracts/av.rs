use alloy_network::EthereumSigner;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::state::{AccountOverride, StateOverride};
use alloy_rpc_types::{TransactionRequest, WithOtherFields};
use alloy_signer_wallet::LocalWallet;
use alloy_sol_types::sol;
use anvil::{NodeConfig, PrecompileFactory};
use revm_primitives::{address, Address, U256};
use revm_primitives::{Bytes, Precompile};

use crate::arb_wasm;
use crate::constants::{ACTIVATED_WASM, COMPRESSED_WASM};

const STYLUS_RPC_URL: &str = "https://stylus-testnet.arbitrum.io/rpc";
const ARBWASM_ACTIVATE_METHOD_HASH: [u8; 4] = [88, 199, 128, 194];
const ARB_WASM_ADDRESS: Address =
    address!("0000000000000000000000000000000000000071");
const ALICE: Address = address!("A11CEacF9aa32246d767FCCD72e02d6bCbcC375d");
const CONTRACT: Address = address!("0000000F9aa32246d767FCCD72e02d6bCbcC375d");

#[derive(Debug, Clone)]
struct CustomPrecompileFactory;

impl PrecompileFactory for CustomPrecompileFactory {
    fn precompiles(&self) -> Vec<(Address, Precompile)> {
        vec![(ARB_WASM_ADDRESS, Precompile::Env(arb_wasm::arb_wasm))]
    }
}

sol! {
    #[sol(rpc)]
    contract Test {
        function getter() external returns (string memory);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn spawns_anvil() {
    let factory = CustomPrecompileFactory;
    let config = NodeConfig::default()
        // .silent()
        .with_tracing(true)
        .with_steps_tracing(true)
        .with_eth_rpc_url(Some(STYLUS_RPC_URL))
        .with_precompile_factory(factory);
    let (api, node) = anvil::spawn(config).await;
    assert_eq!(api.chain_id(), 23011913);

    api.anvil_set_logging(true).await.unwrap();

    // let alice = api.accounts().unwrap()[0];
    let alice = LocalWallet::random();
    let from = alice.address();

    // let rpc_url = node.http_endpoint().parse().unwrap();
    let rpc_url = STYLUS_RPC_URL.parse().unwrap();
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .signer(EthereumSigner::from(alice))
        // .on_hyper_http(rpc_url)
        .on_http(rpc_url)
        .unwrap();
    // let provider = ProviderBuilder::new().on_hyper_http(rpc_url).unwrap();

    // let calldata: Bytes = "58c780c20000000000000000000000000000000F9aa32246d767FCCD72e02d6bCbcC375d".parse().unwrap();
    // let tx = TransactionRequest::default()
    //     .to(ARB_WASM_ADDRESS.into())
    //     .input(calldata.into());

    // let compressed_wasm: Bytes = COMPRESSED_WASM.parse().unwrap();
    // api.anvil_set_code(CONTRACT, compressed_wasm.clone()).await.unwrap();

    // let activated_wasm =
    //     api.call(WithOtherFields::new(tx), None, None).await.unwrap();
    // api.anvil_set_code(CONTRACT, activated_wasm.clone()).await.unwrap();
    let activated_wasm: Bytes = ACTIVATED_WASM.parse().unwrap();
    api.anvil_set_code(CONTRACT, activated_wasm.clone()).await.unwrap();
    // let c = api.get_code(CONTRACT, None).await.unwrap();
    // assert_eq!(c, activated_wasm);
    //
    // let c1 = provider
    //     .get_code_at(CONTRACT, BlockId::Number(BlockNumberOrTag::Latest))
    //     .await
    //     .unwrap();
    // assert_eq!(c1, activated_wasm);

    // let contract = ITest::new(CONTRACT, &provider);
    // let r = contract.getter().call_raw().await.unwrap();

    // let tx = TransactionRequest::default()
    //     .from(from)
    //     .to(CONTRACT.into())
    //     .input(Bytes::from([153, 58, 4, 183]).into());
    // let mut state = StateOverride::default();
    // let mut account = AccountOverride::default();
    // account.code = Some(activated_wasm);
    // state.insert(CONTRACT, account);
    // let r =
    //     api.call(WithOtherFields::new(tx), None, Some(state)).await.unwrap();

    let mut state = StateOverride::default();
    let mut account = AccountOverride::default();
    account.code = Some(activated_wasm);
    state.insert(CONTRACT, account);
    let contract = Test::new(CONTRACT, &provider);
    let r = contract
        .getter()
        .from(from)
        .gas(100000000000000000)
        .nonce(0)
        .value(U256::ZERO)
        .state(state)
        .call_raw()
        .await
        .unwrap();
    // let args = contract.getter().abi_encode();
    // let args = Test::getterCall::SIGNATURE;

    // let a = vec![153, 58, 4, 183];
    // let mut s = String::with_capacity(a.len() * 2);
    // for (i, v) in a.into_iter().enumerate() {
    //     s.push_str(&format!("{:02x}", v));
    // }
    // println!("{s}");

    // let args = Test::getterCall::new(()).abi_encode();
    // let selector = Test::getterCall::SELECTOR;

    // let builder: SolCallBuilder<_, _, Test::getterCall, _> = contract.getter();
    // let r = builder.call().await;
    // match r {
    //     Ok(Test::getterReturn { v }) => println!("{v:?}"),
    //     Err(e) => println!("{e:?}"),
    //     _ => {}
    // }

    println!("=================================");
    println!("{r:?}");
    // println!("{args:?}");
    // println!("{selector:?}");

    // assert_eq!(a, U8::ZERO);
}
