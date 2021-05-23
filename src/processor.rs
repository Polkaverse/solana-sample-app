
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::state::Account as TokenAccount;

use crate::{error::TokenError, instruction::AdvanceTokenInstruction };

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = AdvanceTokenInstruction::unpack(instruction_data)?;

        match instruction {
            AdvanceTokenInstruction::InitializeABCToken {
                ..
            } => {
                msg!("Instruction: InitializeABCToken");
                Self::process_init_token(accounts, amount, program_id)
            }
            AdvanceTokenInstruction::InitializeABCTokenFuture { .. } => {
                msg!("Instruction: InitializeABCTokenFuture");
                Self::process_init_future_token(accounts, amount, program_id)
            }
            AdvanceTokenInstruction::BurnABCFutureAndMintABC {  .. } => {
                msg!("Instruction: BurnABCFutureAndMintABC");
                Self::process_init_burn_token(accounts, amount, program_id)
            }
        }
    }

    fn process_init_token(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        Ok(())
    }

    fn process_init_future_token(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        Ok(())
    }

    fn process_init_burn_token(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        Ok(())
    }
}
