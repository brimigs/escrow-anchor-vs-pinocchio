use anchor_escrow;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

/// Test that the program ID is correctly set
#[test]
fn test_program_id() {
    let expected_id = "22222222222222222222222222222222222222222222";
    let program_id = anchor_escrow::ID;
    assert_eq!(program_id.to_string(), expected_id);
    println!("✅ Program ID test passed");
}

/// Test PDA derivation for escrow accounts
#[test]
fn test_escrow_pda_derivation() {
    let maker = Keypair::new();
    let seed = 42u64;
    
    // Derive PDA for escrow account
    let (escrow_pda, bump) = Pubkey::find_program_address(
        &[b"escrow", maker.pubkey().as_ref(), &seed.to_le_bytes()],
        &anchor_escrow::ID,
    );
    
    // Verify PDA is valid
    assert!(escrow_pda != Pubkey::default());
    assert!(bump > 0);
    
    // Verify deterministic - same inputs produce same PDA
    let (escrow_pda2, bump2) = Pubkey::find_program_address(
        &[b"escrow", maker.pubkey().as_ref(), &seed.to_le_bytes()],
        &anchor_escrow::ID,
    );
    assert_eq!(escrow_pda, escrow_pda2);
    assert_eq!(bump, bump2);
    
    // Different seed should produce different PDA
    let different_seed = 100u64;
    let (escrow_pda3, _) = Pubkey::find_program_address(
        &[b"escrow", maker.pubkey().as_ref(), &different_seed.to_le_bytes()],
        &anchor_escrow::ID,
    );
    assert_ne!(escrow_pda, escrow_pda3);
    
    // Different maker should produce different PDA
    let other_maker = Keypair::new();
    let (escrow_pda4, _) = Pubkey::find_program_address(
        &[b"escrow", other_maker.pubkey().as_ref(), &seed.to_le_bytes()],
        &anchor_escrow::ID,
    );
    assert_ne!(escrow_pda, escrow_pda4);
    
    println!("✅ Escrow PDA derivation test passed");
}

/// Test that different seeds produce unique PDAs
#[test]
fn test_unique_escrow_pdas() {
    let maker = Keypair::new();
    let mut pdas = Vec::new();
    
    // Generate PDAs for different seeds
    for seed in 0..10u64 {
        let (pda, _) = Pubkey::find_program_address(
            &[b"escrow", maker.pubkey().as_ref(), &seed.to_le_bytes()],
            &anchor_escrow::ID,
        );
        
        // Check this PDA hasn't been generated before
        assert!(!pdas.contains(&pda), "Duplicate PDA found for seed {}", seed);
        pdas.push(pda);
    }
    
    println!("✅ Unique escrow PDAs test passed");
}

/// Test associated token address calculation
#[test]
fn test_associated_token_addresses() {
    let owner = Keypair::new();
    let mint = Keypair::new();
    
    // Calculate ATA using SPL method
    let ata = anchor_spl::associated_token::get_associated_token_address(
        &owner.pubkey(),
        &mint.pubkey(),
    );
    
    // Verify it's deterministic
    let ata2 = anchor_spl::associated_token::get_associated_token_address(
        &owner.pubkey(),
        &mint.pubkey(),
    );
    assert_eq!(ata, ata2);
    
    // Different owner should give different ATA
    let other_owner = Keypair::new();
    let ata3 = anchor_spl::associated_token::get_associated_token_address(
        &other_owner.pubkey(),
        &mint.pubkey(),
    );
    assert_ne!(ata, ata3);
    
    println!("✅ Associated token addresses test passed");
}

/// Test vault ATA calculation for escrow PDA
#[test]
fn test_vault_ata_for_escrow() {
    let maker = Keypair::new();
    let mint = Keypair::new();
    let seed = 42u64;
    
    // Get escrow PDA
    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", maker.pubkey().as_ref(), &seed.to_le_bytes()],
        &anchor_escrow::ID,
    );
    
    // Get vault (ATA of escrow PDA)
    let vault = anchor_spl::associated_token::get_associated_token_address(
        &escrow_pda,
        &mint.pubkey(),
    );
    
    // Verify vault is unique per mint
    let other_mint = Keypair::new();
    let vault2 = anchor_spl::associated_token::get_associated_token_address(
        &escrow_pda,
        &other_mint.pubkey(),
    );
    assert_ne!(vault, vault2);
    
    println!("✅ Vault ATA for escrow test passed");
}

/// Test that all required SPL program IDs are accessible
#[test]
fn test_spl_program_ids() {
    // Test Token Program ID
    let token_program = spl_token::ID;
    assert_eq!(
        token_program.to_string(),
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    );
    
    // Test Associated Token Program ID
    let ata_program = spl_associated_token_account::ID;
    assert_eq!(
        ata_program.to_string(),
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
    );
    
    // Test System Program ID
    let system_program = solana_program::system_program::ID;
    assert_eq!(
        system_program.to_string(),
        "11111111111111111111111111111111"
    );
    
    println!("✅ SPL program IDs test passed");
}