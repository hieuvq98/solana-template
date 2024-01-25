use anchor_lang::prelude::*;

use crate::state::{AppData, SaleStandard, SaleSequenced};
use crate::APP_DATA_SEED;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        init,
        seeds = [&APP_DATA_SEED],
        bump,
        payer = root,
        space = 8 + AppData::LEN,
    )]
    pub app_data: Account<'info, AppData>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetFeeReceiver<'info> {
    #[account(mut)]
    pub root: Signer<'info>,
    
    #[account(
        mut,
        seeds=[&APP_DATA_SEED],
        bump=app_data.nonce,
    )]
    pub app_data: Account<'info, AppData>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetSaleStandardProtocolFee<'info> {
    #[account(mut)]
    pub root: Signer<'info>,
    
    #[account(mut)]
    pub launchpad: Account<'info, SaleStandard>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SetSaleSequencedProtocolFee<'info> {
    #[account(mut)]
    pub root: Signer<'info>,
    
    #[account(mut)]
    pub launchpad: Account<'info, SaleSequenced>,

    pub system_program: Program<'info, System>
}

