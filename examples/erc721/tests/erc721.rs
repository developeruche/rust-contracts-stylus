#![cfg(feature = "e2e")]

use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::eth::Filter,
    sol,
    sol_types::SolConstructor,
};
use e2e::{prelude::Assert, user::User};

use crate::abi::Erc721;

mod abi;

sol!("src/constructor.sol");

const TOKEN_NAME: &str = "Test Token";
const TOKEN_SYMBOL: &str = "NFT";

fn random_token_id() -> U256 {
    let num: u32 = rand::random();
    U256::from(num)
}

async fn deploy(rpc_url: &str, private_key: &str) -> eyre::Result<Address> {
    let name = env!("CARGO_PKG_NAME").replace('-', "_");
    let pkg_dir = env!("CARGO_MANIFEST_DIR");
    let args = Erc721Example::constructorCall {
        name_: TOKEN_NAME.to_owned(),
        symbol_: TOKEN_SYMBOL.to_owned(),
    };
    let args = alloy::hex::encode(args.abi_encode());
    let contract_addr =
        e2e::deploy::deploy(&name, pkg_dir, rpc_url, private_key, Some(args))
            .await?;

    Ok(contract_addr)
}

macro_rules! send {
    ($e:expr) => {
        $e.send().await
    };
}

macro_rules! watch {
    ($e:expr) => {
        $e.send().await?.watch().await
    };
}

#[e2e::test]
async fn constructs(alice: User) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let name = contract.name().call().await?.name;
    let symbol = contract.symbol().call().await?.symbol;

    assert_eq!(name, TOKEN_NAME.to_owned());
    assert_eq!(symbol, TOKEN_SYMBOL.to_owned());
    Ok(())
}

#[e2e::test]
async fn mints(alice: User) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let alice_addr = alice.address();
    let token_id = random_token_id();
    let _ = watch!(contract.mint(alice_addr, token_id).from(alice_addr))?;
    let owner_of = contract.ownerOf(token_id).call().await?.ownerOf;
    assert_eq!(owner_of, alice_addr);

    let balance = contract.balanceOf(alice_addr).call().await?.balance;
    assert!(balance >= U256::from(1));
    Ok(())
}

#[e2e::test]
async fn errors_when_reusing_token_id(alice: User) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let alice_addr = alice.address();
    let token_id = random_token_id();
    let _ = watch!(contract.mint(alice_addr, token_id))?;
    let err = send!(contract.mint(alice_addr, token_id))
        .expect_err("should not mint a token id twice");
    err.assert(Erc721::ERC721InvalidSender { sender: Address::ZERO });
    Ok(())
}

#[e2e::test]
async fn transfers(alice: User, bob: User) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let alice_addr = alice.address();
    let bob_addr = bob.address();
    let token_id = random_token_id();
    let _ = watch!(contract.mint(alice_addr, token_id).from(alice_addr))?;
    let _ = watch!(contract
        .transferFrom(alice_addr, bob_addr, token_id)
        .from(alice_addr))?;

    // TODO: Implement a helper that abstracts away this boilerplate code.
    // Something like `emits(Erc721::Transfer {from, to, tokenId});`.
    // Work tracked [here](https://github.com/OpenZeppelin/rust-contracts-stylus/issues/88).
    let block = alice.signer.get_block_number().await?;
    let filter = Filter::new().address(contract_addr).from_block(block);
    let logs = alice.signer.get_logs(&filter).await?;
    let transfer: Erc721::Transfer =
        logs[logs.len() - 1].log_decode()?.inner.data;
    assert_eq!(transfer.from, alice_addr);
    assert_eq!(transfer.to, bob_addr);
    assert_eq!(transfer.tokenId, token_id);

    let Erc721::ownerOfReturn { ownerOf } =
        contract.ownerOf(token_id).call().await?;
    assert_eq!(ownerOf, bob_addr);
    Ok(())
}

#[e2e::test]
async fn errors_when_transfer_nonexistent_token(
    alice: User,
    bob: User,
) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let alice_addr = alice.address();
    let token_id = random_token_id();
    let tx = contract
        .transferFrom(alice_addr, bob.address(), token_id)
        .from(alice_addr);
    let err = send!(tx).expect_err("should not transfer a non-existent token");
    err.assert(Erc721::ERC721NonexistentToken { tokenId: token_id });
    Ok(())
}

#[e2e::test]
async fn approves_token_transfer(alice: User, bob: User) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let alice_addr = alice.address();
    let bob_addr = bob.address();
    let token_id = random_token_id();
    let _ = watch!(contract.mint(alice_addr, token_id).from(alice_addr))?;
    let _ = watch!(contract.approve(bob_addr, token_id).from(alice_addr))?;

    let contract = Erc721::new(contract_addr, &bob.signer);
    let _ = watch!(contract
        .transferFrom(alice_addr, bob_addr, token_id)
        .from(bob_addr))?;
    let Erc721::ownerOfReturn { ownerOf } =
        contract.ownerOf(token_id).call().await?;
    assert_ne!(ownerOf, alice_addr);
    assert_eq!(ownerOf, bob_addr);
    Ok(())
}

#[e2e::test]
async fn errors_when_transfer_unapproved_token(
    alice: User,
    bob: User,
) -> eyre::Result<()> {
    let contract_addr = deploy(alice.url(), &alice.pk()).await?;
    let contract = Erc721::new(contract_addr, &alice.signer);

    let alice_addr = alice.address();
    let bob_addr = bob.address();
    let token_id = random_token_id();
    let _ = watch!(contract.mint(alice_addr, token_id).from(alice_addr))?;

    let contract = Erc721::new(contract_addr, &bob.signer);
    let tx =
        contract.transferFrom(alice_addr, bob_addr, token_id).from(bob_addr);
    let err = send!(tx).expect_err("should not transfer unapproved token");
    err.assert(Erc721::ERC721InsufficientApproval {
        operator: bob_addr,
        tokenId: token_id,
    });
    Ok(())
}