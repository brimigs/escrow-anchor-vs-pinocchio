use {
    num_derive::FromPrimitive,
    pinocchio::program_error::ProgramError,
    thiserror::Error
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PinocchioError {
    #[error("Not a signer")]
    NotSigner,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("Invalid address")]
    InvalidAddress,

    #[error("Invalid mint")]
    InvalidMint,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Escrow expired")]
    EscrowExpired,

    #[error("Invalid discriminator")]
    InvalidDiscriminator,

    #[error("Account already initialized")]
    AlreadyInitialized,
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        ProgramError::Custom(e as u32)
    }
}