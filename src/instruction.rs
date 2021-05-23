use solana_program::program_error::ProgramError;
use std::convert::TryInto;
use chrono::prelude::*;

use crate::error::TokenError::InvalidInstruction;
use crate::error::TokenError;

pub struct TokenABC {
    amount: u64,
}

pub struct TokenABCFuture {
    amount: u64,
    time: u64,
}

pub enum AdvanceTokenInstruction {
    InitializeABCToken(TokenABC),
    InitializeABCTokenFuture(TokenABCFuture),
    BurnABCFutureAndMintABC(TokenABC),
}

impl AdvanceTokenInstruction {
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
                let mut token_abc_future = TokenABCFuture { amount: 0, time: 15 };
                let current_time = utc::now();
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