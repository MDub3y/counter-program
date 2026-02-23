use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::{ProgramResult}, 
    pubkey::Pubkey
};

#[derive(BorshDeserialize, BorshSerialize)]
struct OnChainData {
    count: u32
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?;

    let mut counter = OnChainData::try_from_slice(&data_account.data.borrow_mut())?;    // borrow_mut does runtime borrow checking, and &mut does compile time
    // RefCell<T>::borrow_mut(): 
    // - runtime checked, 
    // - allows mutation through shared reference (&AccountInfo)
    // - panics
    // - used for interior mutability

    if counter.count == 0{
        counter.count = 1;
    } else {
        counter.count = counter.count * 2;
    }

    counter.serialize(&mut *data_account.data.borrow_mut());

    Ok(())
}