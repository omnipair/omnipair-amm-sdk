use anchor_lang::prelude::{AccountMeta, AnchorDeserialize, Pubkey};
use anyhow::Result;
use jupiter_amm_interface::{AmmProgramIdToLabel, ClockRef};

use crate::{
    OmnipairFutarchyAuthority, OmnipairPair, OmnipairRateModel, FUTARCHY_AUTHORITY_SEED_PREFIX,
    OMNIPAIR_PROGRAM_ID, RESERVE_VAULT_SEED_PREFIX,
};

#[derive(Clone)]
pub struct OmnipairAmmClient {
    pub pair_key: Pubkey,
    pub state: OmnipairPair,
    pub(crate) derived: DerivedAccounts,
    pub(crate) clock_ref: ClockRef,
    pub(crate) rate_model_data: Option<OmnipairRateModel>,
    pub(crate) interest_bps: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct DerivedAccounts {
    pub(crate) reserve_vault0: Pubkey,
    pub(crate) reserve_vault1: Pubkey,
    pub(crate) futarchy_authority: Pubkey,
    pub(crate) event_authority: Pubkey,
}

impl DerivedAccounts {
    pub(crate) fn compute(pair_key: &Pubkey, state: &OmnipairPair) -> Self {
        let (reserve_vault0, _) = Pubkey::find_program_address(
            &[
                RESERVE_VAULT_SEED_PREFIX,
                pair_key.as_ref(),
                state.token0.as_ref(),
            ],
            &OMNIPAIR_PROGRAM_ID,
        );
        let (reserve_vault1, _) = Pubkey::find_program_address(
            &[
                RESERVE_VAULT_SEED_PREFIX,
                pair_key.as_ref(),
                state.token1.as_ref(),
            ],
            &OMNIPAIR_PROGRAM_ID,
        );
        let (futarchy_authority, _) =
            Pubkey::find_program_address(&[FUTARCHY_AUTHORITY_SEED_PREFIX], &OMNIPAIR_PROGRAM_ID);
        let (event_authority, _) =
            Pubkey::find_program_address(&[b"__event_authority"], &OMNIPAIR_PROGRAM_ID);
        Self {
            reserve_vault0,
            reserve_vault1,
            futarchy_authority,
            event_authority,
        }
    }
}

impl AmmProgramIdToLabel for OmnipairAmmClient {
    const PROGRAM_ID_TO_LABELS: &[(Pubkey, jupiter_amm_interface::AmmLabel)] =
        &[(OMNIPAIR_PROGRAM_ID, "Omnipair")];
}

pub(crate) fn deserialize_pair(data: &[u8]) -> Result<OmnipairPair> {
    if data.len() < 8 {
        anyhow::bail!(crate::OmnipairError::InvalidAccountData);
    }
    Ok(OmnipairPair::deserialize(&mut &data[8..])?)
}

pub(crate) fn deserialize_rate_model(data: &[u8]) -> Result<OmnipairRateModel> {
    if data.len() < 8 {
        anyhow::bail!(crate::OmnipairError::InvalidAccountData);
    }
    Ok(OmnipairRateModel::deserialize(&mut &data[8..])?)
}

pub(crate) fn deserialize_futarchy_authority(data: &[u8]) -> Result<OmnipairFutarchyAuthority> {
    if data.len() < 8 {
        anyhow::bail!(crate::OmnipairError::InvalidAccountData);
    }
    Ok(OmnipairFutarchyAuthority::deserialize(&mut &data[8..])?)
}

pub struct OmnipairSwapAccounts {
    pub pair: Pubkey,
    pub rate_model: Pubkey,
    pub futarchy_authority: Pubkey,
    pub token_in_vault: Pubkey,
    pub token_out_vault: Pubkey,
    pub user_token_in_account: Pubkey,
    pub user_token_out_account: Pubkey,
    pub token_in_mint: Pubkey,
    pub token_out_mint: Pubkey,
    pub user: Pubkey,
    pub token_program: Pubkey,
    pub token_2022_program: Pubkey,
    pub event_authority: Pubkey,
    pub omnipair_program: Pubkey,
}

impl From<OmnipairSwapAccounts> for Vec<AccountMeta> {
    fn from(accounts: OmnipairSwapAccounts) -> Self {
        vec![
            AccountMeta::new(accounts.pair, false),
            AccountMeta::new(accounts.rate_model, false),
            AccountMeta::new_readonly(accounts.futarchy_authority, false),
            AccountMeta::new(accounts.token_in_vault, false),
            AccountMeta::new(accounts.token_out_vault, false),
            AccountMeta::new(accounts.user_token_in_account, false),
            AccountMeta::new(accounts.user_token_out_account, false),
            AccountMeta::new_readonly(accounts.token_in_mint, false),
            AccountMeta::new_readonly(accounts.token_out_mint, false),
            AccountMeta::new(accounts.user, true),
            AccountMeta::new_readonly(accounts.token_program, false),
            AccountMeta::new_readonly(accounts.token_2022_program, false),
            AccountMeta::new_readonly(accounts.event_authority, false),
            AccountMeta::new_readonly(accounts.omnipair_program, false),
        ]
    }
}
