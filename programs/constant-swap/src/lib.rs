use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};
use solana_program::{pubkey, pubkey::Pubkey};

// for converting base58 to byte
use bs58;

fn convert_base58_to_byte() {
    let base58_token_a_mint = "Z0Y1X2W3V4U5T6S7R8Q9P0O1N2M3L4K5J6I7H8G9F0E1D2C3B4A5Z6Y7X8W9V0U1T2S3R4Q5P6O7N8M9L0K1J2I3H4G5F6E7D8C9B0A1";
    let base58_token_b_mint = "Z0Y1X2W3V4U5T6S7R8Q9P0O1N2M3L4K5J6I7H8G9F0E1D2C3B4A5Z6Y7X8W9V0U1T2S3R4Q5P6O7N8M9L0K1J2I3H4G5F6E7D8C9B0A1";

    let token_a_bytes = bs58::decode(base58_token_a_mint).into_vec().unwrap();
    let token_b_bytes = bs58::decode(base58_token_b_mint).into_vec().unwrap();

    let token_a_pubkey = Pubkey::new_from_array(token_a_bytes.try_into().unwrap());
    let token_b_pubkey = Pubkey::new_from_array(token_b_bytes.try_into().unwrap());

    (token_a_pubkey, token_b_pubkey)
}

declare_id!("YourProgramIdHere");

const TOKEN_A_MINT: Pubkey = Pubkey::new_from_array([convert_base58_to_byte(token_a_pubkey)]);
const TOKEN_B_MINT: Pubkey = Pubkey::new_from_array([convert_base58_to_byte(token_b_pubkey)]);

#[program]
pub mod token_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount: u64) -> Result<()> {
        // Burn token A from user's account
        let cpi_accounts = Burn {
            mint: ctx.accounts.token_a_mint.to_account_info(),
            from: ctx.accounts.user_token_a.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(cpi_ctx, amount)?;

        // Mint 100X token B to the user's associated token account
        let mint_amount = amount.checked_mul(100).ok_or(ErrorCode::Overflow)?;
        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_mint_b.to_account_info(),
            to: ctx.accounts.user_token_b.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, mint_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut, token::authority = user, constraint = user_token_a.mint == TOKEN_A_MINT)]
    pub user_token_a: Account<'info, TokenAccount>,
    #[account(address = TOKEN_A_MINT)]
    pub token_a_mint: Account<'info, Mint>,
    #[account(address = TOKEN_B_MINT)]
    pub token_mint_b: Account<'info, Mint>,
    #[account(mut, associated_token::mint = token_mint_b, associated_token::authority = user)]
    pub user_token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow occurred.")]
    Overflow,
}
