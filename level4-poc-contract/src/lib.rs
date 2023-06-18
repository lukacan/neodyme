use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // let source_account_info = next_account_info(account_info_iter)?;
    // let dest_account_info = next_account_info(account_info_iter)?;
    // let authority_info = next_account_info(account_info_iter)?;
    panic!("Nothing here yet.");
}

solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DB");
