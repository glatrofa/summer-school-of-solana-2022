use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{instruction::TurnstileInstruction, state::State};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], // array of bytes
) -> ProgramResult { // try_from_slice deserialization method that does other chekcs    
    let instruction = TurnstileInstruction::try_from_slice(instruction_data)?;

    match instruction {
        TurnstileInstruction::Initialze { init_state } => {
            initialize(program_id, accounts, init_state)
        }
        TurnstileInstruction::Coin => coin(program_id, accounts),
        TurnstileInstruction::Push => push(program_id, accounts),
    }
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    init_state: bool,
) -> ProgramResult {
    let account_into_iter = &mut accounts.iter(); // get the iterator for all the accounts
    let state_account_info = next_account_info(account_into_iter)?; // fetch the next account from iter
    let initialezer_account_info = next_account_info(account_into_iter)?;

    let rent = Rent::get()?;
    invoke( // from solana program library
        &system_instruction::create_account( // create a separate account for storing state information
            &initialezer_account_info.key, // pay for the creation of account
            &state_account_info.key, // sent the rent here
            rent.minimum_balance(State::SERIALZED_SIZE), // 2 years rent for the account
            State::SERIALZED_SIZE as u64,
            &program_id,
        ),
        &[initialezer_account_info.clone(), state_account_info.clone()],
    )?;

    let state = State { locked: init_state }; // create state instance
    state.serialize(&mut *state_account_info.data.borrow_mut())?; // write data into state (previous created solana account)

    Ok(())
}

pub fn coin(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    let state = State { locked: false };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}

pub fn push(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_into_iter = &mut accounts.iter();
    let state_account_info = next_account_info(account_into_iter)?;

    let state = State { locked: true };
    state.serialize(&mut *state_account_info.data.borrow_mut())?;

    Ok(())
}
