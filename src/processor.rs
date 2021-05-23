/// Program state processor.

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized },
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::AdvanceTokenInstruction };
use crate::instruction::{TokenABC, TokenABCFuture};

/// Program state handler.
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        /// Unpack the instructions.
        let instruction = AdvanceTokenInstruction::unpack(instruction_data)?;

        match instruction {
            AdvanceTokenInstruction::InitializeABCToken(token) => {
                msg!("Instruction: InitializeABCToken");
                Self::process_init_token(token)
            }
            AdvanceTokenInstruction::InitializeABCTokenFuture(future_token) => {
                msg!("Instruction: InitializeABCTokenFuture");
                Self::process_init_future_token(future_token)
            }
            AdvanceTokenInstruction::BurnABCFutureAndMintABC(token) => {
                msg!("Instruction: BurnABCFutureAndMintABC");
                Self::process_init_burn_token(token)
            }
        }
    }

    /// Initialize the token.
    fn process_init_token(
        future_token: TokenABC,
    ) -> ProgramResult {

        Ok(())
    }

    /// Initialize the future token with Date/time.
    fn process_init_future_token(
        token: TokenABCFuture,
    ) -> ProgramResult {

        Ok(())
    }

    /// Burn the token.
    fn process_init_burn_token(
        token: TokenABC,
    ) -> ProgramResult {

        Ok(())
    }
}
