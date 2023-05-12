use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey,
};
use spl_token::state::Account;
use spl_token_2022::extension::{BaseState, StateWithExtensions};

use crate::assert_initialized;

pub static TOKEN_PROGRAM_IDS: [&Pubkey; 2] = [&spl_token::ID, &spl_token_2022::ID];

pub trait ToTokenAccount {
    fn to_token_account(self) -> Account;
}

impl ToTokenAccount for AccountInfo<'_> {
    fn to_token_account(self) -> Account {
        assert_initialized(&self, ProgramError::UninitializedAccount).unwrap()
    }
}

impl ToTokenAccount for Account {
    fn to_token_account(self) -> Account {
        self
    }
}

pub fn assert_token_program_matches_package(
    token_program_info: &AccountInfo,
    error: impl Into<ProgramError>,
) -> ProgramResult {
    if !TOKEN_PROGRAM_IDS.contains(&token_program_info.key) {
        return Err(error.into());
    } else {
        Ok(())
    }
}

pub fn unpack_with_error<S: BaseState>(
    account_data: &[u8],
    error: impl Into<ProgramError>,
) -> Result<StateWithExtensions<'_, S>, ProgramError> {
    StateWithExtensions::<S>::unpack(account_data).map_err(|_| error.into())
}

/// Asserts that
/// * the given token account is initialized
/// * it's owner matches the provided owner
/// * it's mint matches the provided mint
/// * it holds more than than 0 tokens of the given mint.
/// Accepts either an &AccountInfo or an Account for token_account parameter.
pub fn assert_holder(
    token_account: impl ToTokenAccount,
    owner_info: &AccountInfo,
    mint_info: &AccountInfo,
    error: impl Into<ProgramError> + Clone,
) -> ProgramResult {
    let token_account: Account = token_account.to_token_account();

    if token_account.owner != *owner_info.key {
        return Err(error.into());
    }

    if token_account.mint != *mint_info.key {
        return Err(error.into());
    }

    if token_account.amount == 0 {
        return Err(error.into());
    }

    Ok(())
}
