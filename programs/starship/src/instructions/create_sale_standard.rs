use anchor_lang::accounts::{program, signer};
use anchor_lang::prelude::*;

use crate::constant::{SALE_STANDARD_SEED, SIGNER_SEED};
use crate::state::sale_standard::SaleStandard;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>)]
pub struct CreateSaleStandard<'info> {
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        init,
        seeds = [
            &SALE_STANDARD_SEED,
            &*launchpad_path
        ],
        bump,
        payer = root,
        space = SaleStandard::LEN,
    )]
    pub launchpad: Account<'info, SaleStandard>,

    pub system_program: Program<'info, System>
}

impl <'info> CreateSaleStandard<'info> {
    pub fn process(
        &mut self, 
        launchpad_path: Vec<u8>, 
        token_mint: Pubkey, 
        owner: Pubkey, 
        protocol_fee: u64,
        bump: u8,
        program_id: &Pubkey,
    ) -> Result<()> {
        require!(protocol_fee <= 2000, ErrorCode::MaxFeeReached);
        let launchpad = &mut self.launchpad;

        launchpad.nonce = bump;
        let(_, signer_nonce) = Pubkey::find_program_address(
            &[
                &SIGNER_SEED,
                &launchpad.key().to_bytes(),
            ],
            program_id,
        );

        launchpad.signer_nonce = signer_nonce;
        launchpad.owner = owner;
        launchpad.token_mint = token_mint;
        launchpad.protocol_fee = protocol_fee;

        Ok(())
    }
}