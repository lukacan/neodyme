use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use spl_token::instruction::TokenInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    //panic!("Nothing here yet.");
    let account_info_iter = &mut accounts.iter();

    let source_account_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let dest_account_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;

    // let mut accounts_new = Vec::with_capacity(4);
    // accounts_new.push(AccountMeta::new(*dest_account_info.key, false));
    // accounts_new.push(AccountMeta::new_readonly(*mint_info.key, false));
    // accounts_new.push(AccountMeta::new(*source_account_info.key, false));
    // accounts_new.push(AccountMeta::new_readonly(*token_program.key,false));

    // msg!("HERE");

    // let instr = Instruction {
    //     program_id: *token_program.key,
    //     accounts:accounts_new,
    //     data,
    // };

    // invoke(
    //     &instr,
    //     &[
    //         dest_account_info.clone(),
    //         source_account_info.clone(),
    //         token_program.clone(),
    //         mint_info.clone(),
    //     ],
    // )?;
    let data = TokenInstruction::Transfer { amount: 1 }.pack();

    let mut accounts = Vec::with_capacity(3);
    accounts.push(AccountMeta::new(*dest_account_info.key, false));
    accounts.push(AccountMeta::new(*source_account_info.key, false));
    accounts.push(AccountMeta::new_readonly(*authority_info.key, true));

    let instr = Instruction {
        program_id: *mint_info.key,
        accounts,
        data,
    };

    invoke(
        &instr,
        &[
            dest_account_info.clone(),
            source_account_info.clone(),
            mint_info.clone(),
            authority_info.clone(),
        ],
    )?;
    Ok(())
}

solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DB");
