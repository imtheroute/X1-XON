#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    self, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use solana_security_txt::security_txt;  // 👈 ADD THIS

declare_id!("D5Ssp6VDvrZ7u6ccmM8UHggoBr8PxkkGZvijo4HbmPyX");

// 👇 ADD SECURITY.TXT HERE (after declare_id, before the next use)
security_txt! {
    name: "X1 XON",
    project_url: "https://x1xon.xyz",
    contacts: "email:imtheroute@x1xon.xyz",
    policy: "https://github.com/imtheroute/X1-XON/security/policy",
    source_code: "https://github.com/imtheroute/X1-XON",
    auditors: "None",
    expiry: "2027-01-01T00:00:00.000Z"
}

use anchor_spl::associated_token::get_associated_token_address;

// 👇 PUT YOUR REAL XON TOKEN ADDRESS HERE
pub const REAL_XON_MINT: Pubkey =
    pubkey!("8cx9R6UrQJXRpvhjLXktEsJKSicgqJN2i2aXw4TNUFeH");

/// ======================
/// CONSTANTS
/// ======================
const MAX_VAULT_SUPPLY: u64 = 800_000_000 * 1_000_000_000;
const TAP_AMOUNT: u64 = 1_000 * 1_000_000_000;

// Network throttle: 1M XON per minute globally
const NETWORK_PER_MINUTE: u64 = 1_000_000 * 1_000_000_000;

// XNT pricing (Native X1)
const START_PRICE: u64 = 2;        // 0.000000002 XNT
const MAX_PRICE: u64 = 80_000;     // 0.00008 XNT
const STEP_SIZE: u64 = 10_000 * 1_000_000_000;

#[program]
pub mod xon_time_mint {
    use super::*;

    /// ----------------------
    /// INITIALIZE (ONCE)
    /// ----------------------
    pub fn initialize(ctx: Context<Initialize>, creator: Pubkey) -> Result<()> {
        let global = &mut ctx.accounts.global;
        global.total_minted = 0;
        global.total_claimed = 0;
        global.start_time = 0; 
        global.creator = creator;
        global.bump = ctx.bumps.global;
        Ok(())
    } 

    /// ----------------------
    /// TAP → CLAIM
    /// ----------------------
    pub fn tap_claim(ctx: Context<TapClaim>) -> Result<()> {
        let global = &mut ctx.accounts.global; 

        msg!("ONCHAIN total_claimed = {}", global.total_claimed);

        let user_stats = &mut ctx.accounts.user_stats;
        if user_stats.total_claimed == 0 {
    user_stats.user = ctx.accounts.user.key();
}
        let current_ts = Clock::get()?.unix_timestamp;

        // --- SECURITY FIX 1: DECIMAL VALIDATION ---
        require!(ctx.accounts.xon_mint.decimals == 9, XonError::InvalidDecimals);
        require_keys_eq!(
    ctx.accounts.xon_mint.key(),
    REAL_XON_MINT,
    XonError::InvalidMint
);

        // --- SECURITY FIX 2: IMMUTABLE CREATOR CHECK ---
        // Ensures Native XNT fees go to the address stored during initialize
        require_keys_eq!(
            ctx.accounts.creator_wallet.key(),
            global.creator,
            XonError::InvalidCreator
        );
        // --- SECURITY FIX 3: VERIFY REAL VAULT ATA ---
let real_vault_ata =
    get_associated_token_address(
        &ctx.accounts.vault_authority.key(),
        &ctx.accounts.xon_mint.key(),
    );

require_keys_eq!(
    ctx.accounts.vault_xon_ata.key(),
    real_vault_ata,
    XonError::InvalidVault
);
        // --- SECURITY FIX 4: VERIFY REAL USER ATA ---
let real_user_ata =
    get_associated_token_address(
        &ctx.accounts.user.key(),
        &ctx.accounts.xon_mint.key(),
    );

require_keys_eq!(
    ctx.accounts.user_xon_ata.key(),
    real_user_ata,
    XonError::InvalidUserATA
);

        // FIRST TAP automatically sets start time
        if global.start_time == 0 {
            global.start_time = current_ts;
        }

        // NETWORK RATE LIMIT (1M/min NETWORK)
        let elapsed = current_ts.saturating_sub(global.start_time);
let minutes_elapsed = elapsed / 60;


let network_max = if minutes_elapsed == 0 {
    NETWORK_PER_MINUTE
} else {
    NETWORK_PER_MINUTE
        .checked_mul(minutes_elapsed as u64)
        .ok_or(XonError::MathOverflow)?
};

        require!(
            global.total_minted + TAP_AMOUNT <= network_max,
            XonError::RateLimited
        );

        // VAULT SUPPLY CHECK
        require!(
            global.total_claimed + TAP_AMOUNT <= MAX_VAULT_SUPPLY,
            XonError::VaultEmpty
        );

        // PRICE LOGIC
        let steps = global.total_claimed / STEP_SIZE;
        let mut price = START_PRICE.checked_add(steps).ok_or(XonError::MathOverflow)?;
        if price > MAX_PRICE {
            price = MAX_PRICE;
        }

        let xnt_fee = price
            .checked_mul(TAP_AMOUNT / 1_000_000_000)
            .ok_or(XonError::MathOverflow)?;

        // 1. PAY NATIVE XNT → CREATOR (SYSTEM TRANSFER)
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.creator_wallet.to_account_info(),
                },
            ),
            xnt_fee,
        )?;

        // PDA Signer for the Vault Authority
        let vault_bump = ctx.bumps.vault_authority;


let signer_seeds: &[&[&[u8]]] = &[&[
    b"vault-authority",
    &[vault_bump],
]];
        // --- SECURITY FIX 5: ENSURE VAULT HAS ENOUGH TOKENS ---
require!(
    ctx.accounts.vault_xon_ata.amount >= TAP_AMOUNT,
    XonError::VaultEmpty
);

        // 2. TRANSFER XON FROM VAULT (Tap Claim)
        token_interface::transfer_checked(

            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.vault_xon_ata.to_account_info(),
                    to: ctx.accounts.user_xon_ata.to_account_info(),
                    mint: ctx.accounts.xon_mint.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                signer_seeds,
            ),
            TAP_AMOUNT, 9,
        )?;

        // UPDATE STATE
        global.total_minted = global
    .total_minted
    .checked_add(TAP_AMOUNT)
    .ok_or(XonError::MathOverflow)?;

global.total_claimed = global
    .total_claimed
    .checked_add(TAP_AMOUNT)
    .ok_or(XonError::MathOverflow)?;

        user_stats.total_claimed = user_stats
    .total_claimed
    .checked_add(TAP_AMOUNT)
    .ok_or(XonError::MathOverflow)?;

        emit!(LeaderboardEvent {
            user: ctx.accounts.user.key(),
            total: user_stats.total_claimed,
        });

        Ok(())
    }

    /// ----------------------
    /// SEND XON (TOKEN-2022)
    /// ----------------------
    pub fn send_tokens(ctx: Context<SendTokens>, amount: u64) -> Result<()> {
       token_interface::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.from_ata.to_account_info(),
                    to: ctx.accounts.to_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            amount, 9,
        )?;
        Ok(())
    }

    /// ----------------------
    /// SEND XNT (NATIVE X1)
    /// ----------------------
    pub fn send_native(ctx: Context<SendNative>, amount: u64) -> Result<()> {
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.sender.to_account_info(),
                    to: ctx.accounts.receiver.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

/// ======================
/// ACCOUNTS
/// ======================

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub global: Account<'info, GlobalState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TapClaim<'info> {
    #[account(mut, seeds = [b"config"], bump = global.bump)]
    pub global: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    pub xon_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = xon_mint,
        token::authority = vault_authority
    )]
    pub vault_xon_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
    mut,
    token::mint = xon_mint,
    token::authority = user
)]
    pub user_xon_ata: InterfaceAccount<'info, TokenAccount>,


    /// CHECK: PDA signer
    #[account(seeds = [b"vault-authority"], bump)]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub creator_wallet: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendTokens<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub from_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,

   pub mint: InterfaceAccount<'info, Mint>,

   pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct SendNative<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    /// CHECK: Native wallet receiving XNT
    pub receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GlobalState {
    pub total_minted: u64,
    pub total_claimed: u64,
    pub start_time: i64,
    pub creator: Pubkey,
    pub bump: u8,
}

impl GlobalState {
    pub const INIT_SPACE: usize =
        8 + // total_minted
        8 + // total_claimed
        8 + // start_time
        32 + // creator
        1; // bump
}

#[account]
pub struct UserStats {
    pub user: Pubkey,
    pub total_claimed: u64,
}

#[event]
pub struct LeaderboardEvent {
    pub user: Pubkey,
    pub total: u64,
}

#[error_code]
pub enum XonError {
    #[msg("Vault empty")]
    VaultEmpty,

    #[msg("Rate limited")]
    RateLimited,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Invalid creator wallet passed")]
    InvalidCreator,

    #[msg("Mint must have 9 decimals")]
    InvalidDecimals,

    #[msg("Invalid XON mint")]
    InvalidMint,

    #[msg("Invalid vault token account")]
    InvalidVault,

    #[msg("Invalid user token account")]
    InvalidUserATA,
} 
