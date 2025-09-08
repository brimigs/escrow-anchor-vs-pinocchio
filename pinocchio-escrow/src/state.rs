use core::mem::size_of;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

pub const ESCROW_DISCRIMINATOR: [u8; 8] = [0x45, 0x53, 0x43, 0x52, 0x4f, 0x57, 0x00, 0x01]; // "ESCROW\0\1"

#[repr(C)]
pub struct Escrow {
    pub discriminator: [u8; 8],
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub expiry: i64,
    pub bump: [u8; 1],
}

impl Escrow {
    pub const LEN: usize = size_of::<[u8; 8]>()
        + size_of::<u64>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<Pubkey>()
        + size_of::<u64>()
        + size_of::<i64>()
        + size_of::<[u8; 1]>();

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Escrow::LEN {
            return Err(crate::errors::PinocchioError::InvalidAccountData.into());
        }
        let escrow = unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) };
        
        // Validate discriminator for existing accounts (not for new initialization)
        if escrow.discriminator != [0; 8] && escrow.discriminator != ESCROW_DISCRIMINATOR {
            return Err(crate::errors::PinocchioError::InvalidDiscriminator.into());
        }
        
        Ok(escrow)
    }

    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Escrow::LEN {
            return Err(crate::errors::PinocchioError::InvalidAccountData.into());
        }
        let escrow = unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) };
        
        // Validate discriminator
        if escrow.discriminator != ESCROW_DISCRIMINATOR {
            return Err(crate::errors::PinocchioError::InvalidDiscriminator.into());
        }
        
        Ok(escrow)
    }

    #[inline(always)]
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    #[inline(always)]
    pub fn set_maker(&mut self, maker: Pubkey) {
        self.maker = maker;
    }

    #[inline(always)]
    pub fn set_mint_a(&mut self, mint_a: Pubkey) {
        self.mint_a = mint_a;
    }

    #[inline(always)]
    pub fn set_mint_b(&mut self, mint_b: Pubkey) {
        self.mint_b = mint_b;
    }

    #[inline(always)]
    pub fn set_receive(&mut self, receive: u64) {
        self.receive = receive;
    }

    #[inline(always)]
    pub fn set_expiry(&mut self, expiry: i64) {
        self.expiry = expiry;
    }

    #[inline(always)]
    pub fn set_bump(&mut self, bump: [u8; 1]) {
        self.bump = bump;
    }

    #[inline(always)]
    pub fn set_discriminator(&mut self, discriminator: [u8; 8]) {
        self.discriminator = discriminator;
    }

    #[inline(always)]
    pub fn set_inner(
        &mut self,
        seed: u64,
        maker: Pubkey,
        mint_a: Pubkey,
        mint_b: Pubkey,
        receive: u64,
        expiry: i64,
        bump: [u8; 1],
    ) {
        self.discriminator = ESCROW_DISCRIMINATOR;
        self.seed = seed;
        self.maker = maker;
        self.mint_a = mint_a;
        self.mint_b = mint_b;
        self.receive = receive;
        self.expiry = expiry;
        self.bump = bump;
    }

    #[inline(always)]
    pub fn is_expired(&self, current_timestamp: i64) -> bool {
        self.expiry > 0 && current_timestamp > self.expiry
    }
}