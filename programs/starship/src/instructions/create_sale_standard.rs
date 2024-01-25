use anchor_lang::prelude::*;

use crate::constant::{SALE_STANDARD_SEED, SIGNER_SEED};
use crate::state::launchpad::SaleStandard;
use crate::error::ErrorCode;
use crate::events::CreateLaunchpadEvent;

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

        launchpad.info.nonce = bump;
        let(_, signer_nonce) = Pubkey::find_program_address(
            &[
                &SIGNER_SEED,
                &launchpad.key().to_bytes(),
            ],
            program_id,
        );

        launchpad.info.signer_nonce = signer_nonce;
        launchpad.info.owner = owner;
        launchpad.info.token_mint = token_mint;
        launchpad.info.protocol_fee = protocol_fee;

        emit!(CreateLaunchpadEvent {
            launchpad_path,
            token_mint
        });

        Ok(())
    }
}