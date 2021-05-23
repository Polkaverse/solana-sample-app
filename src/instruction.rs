use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::TokenError::InvalidInstruction;
use crate::error::TokenError;

/// Initialize TokenABC.
pub struct TokenABC {
    amount: u64,
}

/// Initialize TokenABCFuture.
pub struct TokenABCFuture {
    amount: u64,
    time: u64,
}

/// Instructions supported by the token program.
pub enum AdvanceTokenInstruction {
    InitializeABCToken(TokenABC),
    InitializeABCTokenFuture(TokenABCFuture),
    BurnABCFutureAndMintABC(TokenABC),
}

impl AdvanceTokenInstruction {
    /// Unpacks a byte buffer.
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                Self::InitializeABCToken(
                    TokenABC {
                        amount: amount
                    }
                )
            }

            1 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let time = 5000; //FIXME Intializer time

                Self::InitializeABCTokenFuture(
                    TokenABCFuture {
                        amount: amount,
                        time: time,
                    }
                )
            }

            2 => {
                let mut token_abc = TokenABC { amount: 0 };
                let mut token_abc_future = TokenABCFuture { amount: 0, time: 1500 };
                let current_time = 5000;
                if current_time > token_abc_future.time {
                    token_abc.amount += token_abc_future.amount;
                    token_abc_future.amount = 0;
                    Self::BurnABCFutureAndMintABC(token_abc)
                } else {
                    return Err(TokenError::MatureTime.into())
                }
            }
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}