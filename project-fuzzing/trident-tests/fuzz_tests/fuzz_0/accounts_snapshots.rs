use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct CreateSnapshot<'info> {
    pub action_state: Option<Account<'info, basic_5::ActionState>>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct WalkSnapshot<'info> {
    pub action_state: Account<'info, basic_5::ActionState>,
    pub user: Signer<'info>,
}
pub struct RunSnapshot<'info> {
    pub action_state: Account<'info, basic_5::ActionState>,
    pub user: Signer<'info>,
}
pub struct JumpSnapshot<'info> {
    pub action_state: Account<'info, basic_5::ActionState>,
    pub user: Signer<'info>,
}
pub struct ResetSnapshot<'info> {
    pub action_state: Account<'info, basic_5::ActionState>,
    pub user: Signer<'info>,
}
impl<'info> CreateSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let action_state: Option<anchor_lang::accounts::account::Account<basic_5::ActionState>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("action_state".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("action_state".to_string())
                        })
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided(
                            "action_state".to_string(),
                        ))
                    }
                })
                .transpose()
                .unwrap_or(None);
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            action_state,
            user,
            system_program,
        })
    }
}
impl<'info> WalkSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let action_state: anchor_lang::accounts::account::Account<basic_5::ActionState> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("action_state".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("action_state".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("action_state".to_string()))?;
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        Ok(Self { action_state, user })
    }
}
impl<'info> RunSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let action_state: anchor_lang::accounts::account::Account<basic_5::ActionState> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("action_state".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("action_state".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("action_state".to_string()))?;
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        Ok(Self { action_state, user })
    }
}
impl<'info> JumpSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let action_state: anchor_lang::accounts::account::Account<basic_5::ActionState> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("action_state".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("action_state".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("action_state".to_string()))?;
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        Ok(Self { action_state, user })
    }
}
impl<'info> ResetSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let action_state: anchor_lang::accounts::account::Account<basic_5::ActionState> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("action_state".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("action_state".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("action_state".to_string()))?;
        let user: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("user".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("user".to_string()))?;
        Ok(Self { action_state, user })
    }
}
