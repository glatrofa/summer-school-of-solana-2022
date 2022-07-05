use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
// instance of this enum to the instruction_data array of bytes
// Borsh library deserialize/serialize the enum to the bytes array
pub enum TurnstileInstruction {
    /// Initialize a Turnstile state
    ///
    /// Passed accounts:
    ///
    /// (1) [signer, writable] State Account
    /// (2) [signer, writable] Initializer (who pays for the transaction)
    /// (3) [] System Program will be called create_account instruction
    Initialze { init_state: bool },
    /// Push
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    Push,
    /// Coin
    ///
    /// Passed accounts:
    ///
    /// (1) [writable] State Account
    Coin,
}

pub fn initialize(
    turnstile_program: Pubkey,
    state: Pubkey,
    initializer: Pubkey,
    init_state: bool,
) -> Instruction {
    Instruction { // builds Solana instruction
        program_id: turnstile_program,
        accounts: vec![
            // AccountMeta defines if the account is signable, if it's true then the account is writeable
            AccountMeta::new(state, true),
            AccountMeta::new(initializer, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: TurnstileInstruction::Initialze { init_state }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn coin(turnstile_program: Pubkey, state: Pubkey) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![AccountMeta::new(state, false)],
        data: TurnstileInstruction::Coin.try_to_vec().unwrap(), //[2]
    }
}

pub fn push(turnstile_program: Pubkey, state: Pubkey) -> Instruction {
    Instruction {
        program_id: turnstile_program,
        accounts: vec![AccountMeta::new(state, false)],
        data: TurnstileInstruction::Push.try_to_vec().unwrap(), //[1]
    }
}
