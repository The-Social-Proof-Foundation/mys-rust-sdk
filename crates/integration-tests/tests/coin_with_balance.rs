use anyhow::Result;
use futures::TryStreamExt;
use integration_tests::*;
use mys_crypto::SuiSigner;
use mys_rpc::field::FieldMask;
use mys_rpc::field::FieldMaskUtil;
use mys_rpc::proto::mys::rpc::v2::ExecuteTransactionRequest;
use mys_rpc::proto::mys::rpc::v2::ListOwnedObjectsRequest;
use mys_sdk_types::Address;
use mys_sdk_types::TransactionKind;
use mys_transaction_builder::TransactionBuilder;
use mys_transaction_builder::intent::CoinWithBalance;

#[tokio::test]
async fn large_number_of_requests() -> Result<()> {
    if !integration_tests::check_binary_available() {
        eprintln!("Skipping integration test: mys binary not found. Set MYS_BINARY env var or install mys to run this test.");
        return Ok(());
    }
    let mut mys = SuiNetworkBuilder::default().build().await?;
    let recipient = Address::ZERO;

    let requests = vec![(recipient, 1_000_000_000u64); 500];
    mys.fund(&requests).await?;
    mys.fund(&requests).await?;

    let coins = mys
        .client
        .list_owned_objects(ListOwnedObjectsRequest::default().with_owner(recipient))
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1000);

    // Build a request that requires filling out gas coins and multiple merge_coins
    let mut builder = TransactionBuilder::new();
    builder.set_sender(recipient);
    let arg = builder.intent(CoinWithBalance::mys(950));
    let self_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], self_address);
    builder.build(&mut mys.client).await.unwrap();

    // Build a request that doesn't use the gas coin but requires multiple merge_coins
    let mut builder = TransactionBuilder::new();
    builder.set_sender(recipient);
    let arg = builder.intent(CoinWithBalance::mys(950).with_use_gas_coin(false));
    let self_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], self_address);
    builder.build(&mut mys.client).await.unwrap();
    Ok(())
}

#[tokio::test]
async fn zero_value_requests() -> Result<()> {
    if !integration_tests::check_binary_available() {
        eprintln!("Skipping integration test: mys binary not found. Set MYS_BINARY env var or install mys to run this test.");
        return Ok(());
    }
    let mut mys = SuiNetworkBuilder::default().build().await?;
    let private_key = mys.user_keys.first().unwrap();
    let sender = private_key.public_key().derive_address();
    let recipient = Address::ZERO;

    let mut builder = TransactionBuilder::new();
    builder.set_sender(sender);
    let arg = builder.intent(CoinWithBalance::mys(0));
    let recipient_address = builder.pure(&recipient);
    builder.transfer_objects(vec![arg], recipient_address);
    let transaction = builder.build(&mut mys.client).await?;

    if let TransactionKind::ProgrammableTransaction(pt) = &transaction.kind
        && let mys_sdk_types::Command::MoveCall(call) = pt.commands.first().unwrap()
    {
        assert!(
            call.package == Address::TWO
                && call.module.as_str() == "coin"
                && call.function.as_str() == "zero"
        )
    } else {
        panic!("failed to use 0x2::coin::zero to create zero value coin");
    }

    let signature = private_key.sign_transaction(&transaction)?;

    let _response = mys
        .client
        .execute_transaction_and_wait_for_checkpoint(
            ExecuteTransactionRequest::new(transaction.into())
                .with_signatures(vec![signature.into()])
                .with_read_mask(FieldMask::from_str("*")),
            std::time::Duration::from_secs(10),
        )
        .await?
        .into_inner();

    let coins = mys
        .client
        .list_owned_objects(ListOwnedObjectsRequest::default().with_owner(recipient))
        .try_collect::<Vec<_>>()
        .await?;

    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].balance(), 0);

    Ok(())
}
