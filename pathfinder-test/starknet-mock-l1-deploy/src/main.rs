use alloy::network::TransactionBuilder;
use alloy::primitives::{I256, U256};
use alloy::rpc::types::TransactionRequest;
use alloy::sol_types::SolValue;
use alloy::{providers::ProviderBuilder, sol};
use alloy_provider::Provider;
use alloy_provider::ext::AnvilApi;
use std::fs::File;
use std::io::Write;

// Mocked Starknet contract for testing (no governance).
sol!(
    #[sol(rpc)]
    Starknet,
    "resources/StarknetForSequencerTesting.json"
);

// The following structures are used with a mocked version of the Starknet L1 contract.
// This mocked contract (starknet_for_testing.json) differs from the production contract:
// - Includes an `initializeMock` function that bypasses governance requirements.
// - The `updateState` function doesn't require special permissions.
// - Removed a bunch of checks and functionality not necessary and that was difficult to mock,and
//   that are not called from the sequencer at this time.
// - Used exclusively for integration testing with Anvil.
sol! {
    #[derive(Debug, Default)]
    struct StateUpdate {
        uint256 globalRoot;
        int256 blockNumber;
        uint256 blockHash;
    }

    #[derive(Debug, Default)]
    struct InitializeData {
        uint256 programHash;
        uint256 aggregatorProgramHash;
        address verifier;
        uint256 configHash;
        StateUpdate initialState;
    }
}

const DEFAULT_ANVIL_PORT: u16 = 8545;

#[tokio::main]
async fn main() {
    let anvil_client = ProviderBuilder::new()
        .connect_anvil_with_wallet_and_config(|anvil| anvil.port(DEFAULT_ANVIL_PORT))
        .unwrap();

    let contract = Starknet::deploy(anvil_client.clone()).await.unwrap();
    println!("Contract deployed at: {:?}", contract.address());

    // Generate some transactions
    let anvil_accounts = anvil_client.get_accounts().await.unwrap();
    let sender_address = anvil_accounts[0];
    let receiver_address = anvil_accounts[1];

    let current_block_number = anvil_client.get_block_number().await.unwrap();
    println!("Current block number: {:?}", current_block_number);

    for _ in 0..100 {
        let tx = TransactionRequest::default()
            .with_from(sender_address)
            .with_to(receiver_address)
            .with_value(U256::from(100));
        let pending = anvil_client.send_transaction(tx).await.unwrap();
        let receipt = pending.get_receipt().await.unwrap();
        println!("Sent tx with hash: {:?}", receipt.transaction_hash);
    }

    let current_block_number = anvil_client.get_block_number().await.unwrap();
    println!("Current block number: {:?}", current_block_number);

    let init_data = InitializeData {
        programHash: U256::from(1),
        configHash: U256::from(1),
        initialState: StateUpdate {
            blockNumber: I256::from_dec_str("0").unwrap(),
            ..Default::default()
        },
        ..Default::default()
    };
    let encoded_data = init_data.abi_encode();
    let builder = contract.initializeMock(encoded_data.into());
    let receipt = builder.send().await.unwrap().get_receipt().await.unwrap();
    println!(
        "Initialized contract, tx hash: {:?}",
        receipt.transaction_hash
    );

    let node_info = anvil_client.anvil_node_info().await.unwrap();
    println!("Anvil node info: {:?}", node_info);

    // Dump Anvil state to a file
    let anvil_state = anvil_client.anvil_dump_state().await.unwrap();
    let mut file = File::create("anvil_state.bin").unwrap();
    file.write_all(&anvil_state).unwrap();
    println!("Anvil state written to anvil_state.bin");
}
