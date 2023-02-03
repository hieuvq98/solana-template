use std::str::FromStr;

use anchor_lang::*;
use anchor_lang::solana_program::system_program;
use solana_program::instruction::{Instruction};
use solana_sdk::pubkey::{Pubkey};

use starship::constant::FEE_OWNER;

pub const LAUNCHPAD_SEED_1: &[u8] = &[8, 201, 24, 140, 93, 100, 30, 148];
pub const LAUNCHPAD_PURCHASE_SEED_1: &[u8] = &[68, 70, 141, 93, 102, 104, 120, 59, 54];
pub const GLOBAL_PROFILE_SEED_1: &[u8] = &[139, 126, 195, 157, 204, 134, 142, 146];
pub const GLOBAL_PROFILE_SEED_2: &[u8] = &[32, 40, 118, 173, 164, 46, 192, 86];
pub const LOCAL_PROFILE_SEED_1: &[u8] = &[133, 177, 201, 78, 13, 152, 198, 180];
pub const SIGNER_SEED_1: &[u8] = &[2, 151, 229, 53, 244, 77, 229, 7];
pub const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169]);
pub fn find_launchpad_address(path: Vec<u8>) -> (Pubkey, u8) {
    let vault_seeds = &[&LAUNCHPAD_SEED_1, &*path];
    Pubkey::find_program_address(vault_seeds, &starship::id())
}

pub fn find_launchpad_purchase_address(launchpad_address: Pubkey, token_mint: Pubkey) -> (Pubkey, u8) {
    let vault_seeds = &[&LAUNCHPAD_PURCHASE_SEED_1, launchpad_address.as_ref(), token_mint.as_ref()];
    Pubkey::find_program_address(vault_seeds, &starship::id())
}

pub fn find_global_profile_address(user_address: Pubkey) -> (Pubkey, u8) {
    let vault_seeds = &[&GLOBAL_PROFILE_SEED_1, &GLOBAL_PROFILE_SEED_2, user_address.as_ref()];
    Pubkey::find_program_address(vault_seeds, &starship::id())
}

pub fn find_local_profile_address(launchpad_address: Pubkey, user_address: Pubkey) -> (Pubkey, u8) {
    let vault_seeds = &[&LOCAL_PROFILE_SEED_1, launchpad_address.as_ref(), user_address.as_ref()];
    Pubkey::find_program_address(vault_seeds, &starship::id())
}

pub fn find_launchpad_signer_address(launchpad_address: Pubkey) -> (Pubkey, u8) {
    let vault_seeds = &[&SIGNER_SEED_1, launchpad_address.as_ref()];
    Pubkey::find_program_address(vault_seeds, &starship::id())
}

pub fn create_launchpad_data_instruction(
    owner: &Pubkey,
    path: Vec<u8>,
    token_mint: &Pubkey,
    protocol_fee: u64,
    sharing_fee: u64
)-> Instruction{

    let launchpad_path = path.clone();
    let (launchpad_address, _): (Pubkey, u8) = find_launchpad_address(path);

    let accounts = starship::accounts::CreateLaunchpadContext {
        root: *owner,
        launchpad: launchpad_address,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = starship::instruction::CreateLaunchpad {
        launchpad_path,
        token_mint: *token_mint,
        protocol_fee,
        sharing_fee
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_launchpad_data_instruction(
    owner: &Pubkey,
    launchpad_address: &Pubkey,
    price_n: u64,
    price_d: u64,
    min_per_tx: u64,
    max_per_user: u64,
    limit_sale: u64,
    register_start_timestamp: i64,
    register_end_timestamp: i64,
    redeem_start_timestamp: i64,
    redeem_end_timestamp: i64,
    private_sale_root: Option<[u8; 32]>,
)-> Instruction{
    let accounts = starship::accounts::SetLaunchpadContext {
        root: *owner,
        launchpad: *launchpad_address,
    }.to_account_metas(None);

    let data = starship::instruction::SetLaunchpad {
        price_n: price_n,
        price_d: price_d,
        min_per_tx: min_per_tx,
        max_per_user: max_per_user,
        limit_sale: limit_sale,
        register_start_timestamp: register_start_timestamp,
        register_end_timestamp: register_end_timestamp,
        redeem_start_timestamp: redeem_start_timestamp,
        redeem_end_timestamp: redeem_end_timestamp,
        private_sale_root: private_sale_root,
    }.data();
    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn create_launchpad_purchase_data_instruction(
    owner: &Pubkey,
    launchpad_address: &Pubkey,
    token_mint: &Pubkey,
)-> Instruction{

    let (lauchpad_purchase_address, _): (Pubkey, u8) = find_launchpad_purchase_address(*launchpad_address, *token_mint);
    let accounts = starship::accounts::CreateLaunchpadPurchaseContext {
        root: *owner,
        launchpad: *launchpad_address,
        launchpad_purchase: lauchpad_purchase_address,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = starship::instruction::CreateLaunchpadPurchase {
        token_mint: *token_mint
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_launchpad_purchase_data_instruction(
    owner: &Pubkey,
    launchpad_purchase_address: &Pubkey,
    limit_sale: u64,
    price_n: u64,
    price_d: u64,
    min_per_tx: u64,
    max_per_user: u64
)-> Instruction{

    let accounts = starship::accounts::SetLaunchPadPurchaseContext {
        root: *owner,
        launchpad_purchase: *launchpad_purchase_address,
    }.to_account_metas(None);

    let data = starship::instruction::SetLaunchpadPurchase {
        limit_sale: limit_sale,
        price_n: price_n,
        price_d: price_d,
        min_per_tx: min_per_tx,
        max_per_user: max_per_user
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_launchpad_status_data_instruction(
    owner: &Pubkey,
    launchpad_address: &Pubkey,
    is_active: bool
)-> Instruction{


    let accounts = starship::accounts::SetLaunchpadStatusContext {
        root: *owner,
        launchpad: *launchpad_address,
    }.to_account_metas(None);

    let data = starship::instruction::SetLaunchpadStatus {
        is_active: is_active
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn register_data_instruction(
    launchpad_address: &Pubkey,
    user_address: &Pubkey,
    user_global_profile:  &Pubkey,
    user_local_profile: &Pubkey,
    index: u32,
    proofs: Vec<[u8; 32]>,
)-> Instruction{

    let accounts = starship::accounts::RegisterContext {
        launchpad: *launchpad_address,
        user: *user_address,
        global_profile: *user_global_profile,
        local_profile: *user_local_profile,

    }.to_account_metas(None);

    let data = starship::instruction::Register {
        index: index,
        proofs: proofs
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };


    instruction
}

pub fn create_global_profile_data_instruction(
    owner: &Pubkey,
)-> Instruction{
    let (global_address,_) = find_global_profile_address(*owner);
    let accounts = starship::accounts::CreateGlobalProfileContext {
        payer: *owner,
        global_profile: global_address,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = starship::instruction::CreateGlobalProfile {
        user: *owner
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn create_local_profile_data_instruction(
    owner: &Pubkey,
    launchpad_address: &Pubkey,
)-> Instruction{
    let (local_address,_) = find_local_profile_address(*launchpad_address, *owner);
    let accounts = starship::accounts::CreateLocalProfileContext {
        payer: *owner,
        launchpad: *launchpad_address,
        local_profile: local_address,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = starship::instruction::CreateLocalProfile {
        user: *owner
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn redeem_by_sol_data_instruction(
    user_address: &Pubkey,
    launchpad_address: &Pubkey,
    launchpad_signer_address: &Pubkey,
    global_profile: &Pubkey,
    local_profile: &Pubkey,
    user_token_account: &Pubkey,
    launchpad_token_account: &Pubkey,
    amount: u64, )-> Instruction{
    let accounts = starship::accounts::RedeemBySolContext {
        launchpad: *launchpad_address,
        launchpad_signer: *launchpad_signer_address,
        user: *user_address,
        global_profile: *global_profile,
        local_profile: *local_profile,
        user_token_account: *user_token_account,
        launchpad_token_account: *launchpad_token_account,
        fee_owner: Pubkey::from_str(FEE_OWNER).unwrap(),
        system_program:system_program::id(),
        token_program: TOKEN_PROGRAM_ID,
    }.to_account_metas(None);

    let data = starship::instruction::RedeemBySol {
        amount: amount,
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn redeem_by_token_data_instruction(
    user_address: &Pubkey,
    launchpad_address: &Pubkey,
    launchpad_purchase: &Pubkey,
    launchpad_signer_address: &Pubkey,
    global_profile: &Pubkey,
    local_profile: &Pubkey,
    user_token0_account: &Pubkey,
    user_token1_account: &Pubkey,
    launchpad_token0_account: &Pubkey,
    launchpad_token1_account: &Pubkey,
    fee_owner_token0_account: &Pubkey,
    amount: u64, )-> Instruction{
    let accounts = starship::accounts::RedeemByTokenContext {
        launchpad: *launchpad_address,
        launchpad_purchase:*launchpad_purchase,
        launchpad_signer: *launchpad_signer_address,
        user: *user_address,
        global_profile: *global_profile,
        local_profile: *local_profile,
        user_token0_account: *user_token0_account,
        user_token1_account: *user_token1_account,
        launchpad_token0_account: *launchpad_token0_account,
        launchpad_token1_account: *launchpad_token1_account,
        fee_owner_token0_account: *fee_owner_token0_account,
        token_program: TOKEN_PROGRAM_ID,
    }.to_account_metas(None);

    let data = starship::instruction::RedeemByToken {
        amount: amount,
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_blacklist_data_instruction(
    owner: &Pubkey,
    user_address: &Pubkey,
    global_profile: &Pubkey,
    is_blacklisted: bool,
    )-> Instruction{
    let accounts = starship::accounts::SetBlacklistContext {
        root: *owner,
        global_profile: *global_profile,
    }.to_account_metas(None);

    let data = starship::instruction::SetBlacklist {
        user: *user_address,
        is_blacklisted: is_blacklisted,
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}

pub fn withdraw_token_data_instruction(
    owner: &Pubkey,
    launchpad: &Pubkey,
    launchpad_signer: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    amount: u64
    )-> Instruction{
    let accounts = starship::accounts::WithdrawTokenContext {
        root: *owner,
        launchpad: *launchpad,
        launchpad_signer: *launchpad_signer,
        from: *from,
        to: *to,
        token_program: TOKEN_PROGRAM_ID
    }.to_account_metas(None);

    let data = starship::instruction::WithdrawToken {
        amount: amount
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}
pub fn withdraw_sol_data_instruction(
    owner: &Pubkey,
    launchpad: &Pubkey,
    launchpad_signer: &Pubkey,
    amount: u64
    )-> Instruction{
    let accounts = starship::accounts::WithdrawSolContext {
        root: *owner,
        launchpad: *launchpad,
        launchpad_signer: *launchpad_signer,
        system_program:system_program::id(),
    }.to_account_metas(None);

    let data = starship::instruction::WithdrawSol {
        amount: amount
    }.data();

    let instruction = Instruction {
        program_id: starship::id(),
        data,
        accounts
    };

    instruction
}
