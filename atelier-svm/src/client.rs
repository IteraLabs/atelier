use crate::svm_model;

// client.rs - Deploy the model from off-chain
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::mem;

pub async fn deploy_model() -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://api.devnet.solana.com");
    
    // Your trained model parameters
    let trained_weights = vec![0.342, -0.156, 0.789, -0.234, 0.567];
    let trained_bias = 0.123;
    let precision_decimals = 9;
    
    // Create keypairs
    let payer = Keypair::new();
    let model_account = Keypair::new();
    let program_id = Pubkey::from_str("YourProgramId11111111111111111111111111")?;
    
    // Request airdrop for payer (devnet only)
    let airdrop_sig = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    client.confirm_transaction(&airdrop_sig)?;
    
    // Calculate required space for model account
    let required_space = mem::size_of::<svm_model::SolanaModel>();
    let rent = client.get_minimum_balance_for_rent_exemption(required_space)?;
    
    // Create model account
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &model_account.pubkey(),
        rent,
        required_space as u64,
        &program_id,
    );
    
    // Create initialize model instruction
    let initialize_ix = create_initialize_instruction(
        &program_id,
        &model_account.pubkey(),
        &payer.pubkey(),
        trained_weights,
        trained_bias,
        precision_decimals,
    )?;
    
    // Create and send transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_ix],
        Some(&payer.pubkey()),
        &[&payer, &model_account],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Model deployed! Transaction: {}", signature);
    println!("Model account: {}", model_account.pubkey());
    
    Ok(())
}

fn create_initialize_instruction(
    program_id: &Pubkey,
    model_account: &Pubkey,
    owner: &Pubkey,
    weights: Vec<f64>,
    bias: f64,
    precision_decimals: u32,
) -> Result<Instruction, Box<dyn std::error::Error>> {
    // Serialize instruction data (you'll need to implement proper serialization)
    let instruction_data = serialize_initialize_instruction(weights, bias, precision_decimals)?;
    
    Ok(Instruction::new_with_bytes(
        *program_id,
        &instruction_data,
        vec![
            AccountMeta::new(*model_account, false),
            AccountMeta::new_readonly(*owner, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
    ))
}

