use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_associated_token_account::instructions::Create;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::InitializeMint2;

use crate::errors::PinocchioError;

pub trait AccountCheck {
    fn check(account: &AccountInfo) -> Result<(), ProgramError>;
}

pub struct SignerAccount;

impl SignerAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer() {
            return Err(PinocchioError::NotSigner.into());
        }
        Ok(())
    }
}

pub struct SystemAccount;

impl SystemAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_system::ID) {
            return Err(PinocchioError::InvalidOwner.into());
        }

        Ok(())
    }
}

pub struct MintAccount;

impl MintAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(PinocchioError::InvalidOwner.into());
        }

        if account.data_len() != pinocchio_token::state::Mint::LEN {
            return Err(PinocchioError::InvalidAccountData.into());
        }

        Ok(())
    }
}

pub trait MintInit {
    fn init(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult;
    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult;
}

impl MintInit for MintAccount {
    fn init(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance(pinocchio_token::state::Mint::LEN);

        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: pinocchio_token::state::Mint::LEN as u64,
            owner: &pinocchio_token::ID,
        }
        .invoke()?;

        InitializeMint2 {
            mint: account,
            decimals,
            mint_authority,
            freeze_authority,
        }
        .invoke()
    }

    fn init_if_needed(
        account: &AccountInfo,
        payer: &AccountInfo,
        decimals: u8,
        mint_authority: &[u8; 32],
        freeze_authority: Option<&[u8; 32]>,
    ) -> ProgramResult {
        match Self::check(account) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, payer, decimals, mint_authority, freeze_authority),
        }
    }
}

pub struct TokenAccount;

impl TokenAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(PinocchioError::InvalidOwner.into());
        }

        if account
            .data_len()
            .ne(&pinocchio_token::state::TokenAccount::LEN)
        {
            return Err(PinocchioError::InvalidAccountData.into());
        }

        Ok(())
    }
}

pub struct AssociatedTokenAccount;

impl AssociatedTokenAccount {
    pub fn check(
        account: &AccountInfo,
        authority: &AccountInfo,
        mint: &AccountInfo,
        token_program: &AccountInfo,
    ) -> Result<(), ProgramError> {
        TokenAccount::check(account)?;

        if find_program_address(
            &[authority.key(), token_program.key(), mint.key()],
            &pinocchio_associated_token_account::ID,
        )
        .0
        .ne(account.key())
        {
            return Err(PinocchioError::InvalidAddress.into());
        }

        Ok(())
    }
}

pub trait AssociatedTokenAccountInit {
    fn init(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult;
    fn init_if_needed(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult;
}

impl AssociatedTokenAccount {
    pub fn init(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        Create {
            funding_account: payer,
            account,
            wallet: owner,
            mint,
            system_program,
            token_program,
        }
        .invoke()
    }

    pub fn init_if_needed(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        match Self::check(account, owner, mint, token_program) {
            Ok(_) => Ok(()),
            Err(_) => Self::init(account, mint, payer, owner, system_program, token_program),
        }
    }
}

pub struct ProgramAccount;

impl ProgramAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&crate::ID) {
            return Err(PinocchioError::InvalidOwner.into());
        }

        if account.data_len().ne(&crate::state::Escrow::LEN) {
            return Err(PinocchioError::InvalidAccountData.into());
        }

        Ok(())
    }
}

pub trait ProgramAccountInit {
    fn init<'a, T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult;
}

impl ProgramAccount {
    pub fn init<'a, T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult {
        let lamports = Rent::get()?.minimum_balance(space);

        let signer = [Signer::from(seeds)];

        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&signer)?;

        Ok(())
    }
}

pub trait AccountClose {
    fn close(account: &AccountInfo, destination: &AccountInfo) -> ProgramResult;
}

impl ProgramAccount {
    pub fn close(account: &AccountInfo, destination: &AccountInfo) -> ProgramResult {
        {
            let mut data = account.try_borrow_mut_data()?;
            data[0] = 0xff;
        }

        *destination.try_borrow_mut_lamports()? += *account.try_borrow_lamports()?;
        account.resize(1)?;
        account.close()
    }
}