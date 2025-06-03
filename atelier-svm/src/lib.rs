//! # atelier-agents
//!
//! Agentic Functionality

/// Client abstraction for Solana Virtual Machine
pub mod svm_client;

/// Model Abstraction for Solana Virtual Machine
pub mod svm_model;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey
};

use std::mem;

// Program instructions
#[derive(Debug)]
pub enum ModelInstruction {

    /// Initialize a new model
    /// Accounts:
    /// 0. `[writable]` Model account to initialize
    /// 1. `[signer]` Owner account
    /// 2. `[]` System program
    InitializeModel {
        weights: Vec<f64>,
        bias: f64,
        precision_decimals: u32,
    },
    
    /// Perform forward prediction
    /// Accounts:
    /// 0. `[]` Model account
    /// 1. `[writable]` Result account to store prediction
    ForwardPass {
        inputs: Vec<u64>,
    },

}

// Program entry point
entrypoint!(process_instruction);

// Process instructions of the model
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    ) -> ProgramResult {

    let instruction = ModelInstruction::try_from(instruction_data)?;
    
    match instruction {
        ModelInstruction::InitializeModel { weights, bias, precision_decimals } => {
            process_initialize_model(program_id, accounts, weights, bias, precision_decimals)
        }
        ModelInstruction::ForwardPass { inputs } => {
            process_forward_pass(program_id, accounts, inputs)
        }
    }
}

// Initialize model in account
fn process_initialize_model(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    weights: Vec<f64>,
    bias: f64,
    precision_decimals: u32,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let model_account = next_account_info(account_iter)?;
    let owner_account = next_account_info(account_iter)?;
    
    // Verify owner is signer
    if !owner_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Check account size
    let required_size = mem::size_of::<svm_model::SolanaModel>();
    if model_account.data_len() != required_size {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Create model from trained weights
    let model = svm_model::SolanaModel::from_trained_weights(
        &weights,
        bias,
        *owner_account.key,
        precision_decimals,
    );
    
    // Serialize model to account data
    let mut account_data = model_account.try_borrow_mut_data()?;
    unsafe {
        let model_ptr = &model as *const svm_model::SolanaModel as *const u8;
        let model_slice = std::slice::from_raw_parts(model_ptr, required_size);
        account_data[..required_size].copy_from_slice(model_slice);
    }
    
    msg!("Model initialized with {} features", weights.len());
    Ok(())
}

// Perform forward pass
fn process_forward_pass(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    inputs: Vec<u64>,
    ) -> ProgramResult {

    let account_iter = &mut accounts.iter();
    let model_account = next_account_info(account_iter)?;
    let result_account = next_account_info(account_iter)?;
    
    // Deserialize model from account data
    let account_data = model_account.try_borrow_data()?;
    
    let model = unsafe {
        let model_ptr = account_data.as_ptr() as *const svm_model::SolanaModel;
        &*model_ptr
    };
    
    // Perform prediction
    let prediction = model.forward(&inputs)?;
    
    // Store result (simplified - in practice you might want a more complex result structure)
    let mut result_data = result_account.try_borrow_mut_data()?;
    result_data[..8].copy_from_slice(&prediction.to_le_bytes());
    
    msg!("Prediction: {}", prediction);
    Ok(())
}

