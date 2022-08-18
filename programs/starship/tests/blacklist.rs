pub mod utils;
use std::time::{SystemTime, UNIX_EPOCH};

use anchor_lang::{solana_program::keccak::hashv};

use solana_sdk::clock::Clock;
use solana_sdk::signature::{Keypair, Signer};
use solana_program_test::*;
use utils::helpers::*;
use utils::instructions::*;

#[tokio::test]
async fn user_unban_blacklist_register_claim(){
    println!(">>>>>>>>>> Success: user unban from blacklist redeem <<<<<<<<<<<<<<<");
    let mut context = c98_starship_program_test().start_with_context().await;

    let payer_wallet = Keypair::from_bytes(&[71,26,80,250,238,134,95,254,8,150,193,132,34,14,180,32,84,46,14,119,150,214,118,184,137,163,83,244,236,197,16,15,50,30,210,179,166,74,56,169,144,205,219,137,241,5,133,57,235,192,67,165,11,113,84,123,27,201,254,0,128,223,9,85]).unwrap();
    airdrop(&mut context, &payer_wallet.pubkey(), 10_000_000_000).await.unwrap();

    let feed_path: Vec<u8> = (0..10).map(|_| { rand::random::<u8>() }).collect();
    let launchpad_path =   feed_path.clone();
        // create user to claim
    let user1 = Keypair::new();
    airdrop(&mut context, &user1.pubkey(), 10_000_000_000).await.unwrap();
    let user2 = Keypair::new();
    airdrop(&mut context, &user2.pubkey(), 10_000_000_000).await.unwrap();

    let c98_mint = Keypair::new();
    create_mint(&mut context, &c98_mint, &payer_wallet.pubkey(), None).await.unwrap();
    let user1_c98_token_account = create_associated_token_account(&mut context, &user1.pubkey(), &c98_mint.pubkey()).await.unwrap();

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
    );

    let user1_data = starship::state::WhitelistParams{
      index: 0,
      address: user1.pubkey(),
    };

    let user2_data = starship::state::WhitelistParams{
      index: 1,
      address: user2.pubkey(),
    };
    let hash_user1 = hash_whitelist(user1_data);
    let hash_user2 = hash_whitelist(user2_data);
    let merkle_root: [u8; 32] = if hash_user1 < hash_user2 { hashv(&[&hash_user1, &hash_user2]).to_bytes() }
                                                      else { hashv(&[&hash_user2, &hash_user1]).to_bytes() };

    let mut user1_proofs: Vec<[u8; 32]> =  Vec::new();
    user1_proofs.push(hash_user2);

    let mut user2_proofs: Vec<[u8; 32]> =  Vec::new();
    user2_proofs.push(hash_user1);

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64 ;
    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 1, 10, 100, 1000, time + 10, time + 100, time + 1000, time + 2000, Some(merkle_root));

    process_transaction(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await.unwrap();

    let (launchpad_purchase_address, _) =  find_launchpad_purchase_address(launchpad_address, c98_mint.pubkey());
    let create_launchpad_purchase_data = create_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_address, &c98_mint.pubkey());
    let set_launchpad_purchase_data = set_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_purchase_address, 1000, 1000, 1, 10, 1000);

    process_transaction(&mut context, &Vec::from([create_launchpad_purchase_data, set_launchpad_purchase_data]), &Vec::from([&payer_wallet])).await.unwrap();

    let set_launchpad_status_data_instruction = set_launchpad_status_data_instruction(&payer_wallet.pubkey(), &launchpad_address, true);
    process_transaction(&mut context, &Vec::from([set_launchpad_status_data_instruction]), &Vec::from([&payer_wallet])).await.unwrap();


    let(user1_global_profile,_) = find_global_profile_address(user1.pubkey());
    let(user2_global_profile,_) = find_global_profile_address(user2.pubkey());

    let(user1_local_profile,_) = find_local_profile_address(launchpad_address, user1.pubkey());
    let(user2_local_profile,_) = find_local_profile_address(launchpad_address, user2.pubkey());

    // create user 1 profile
    let create_user1_global_data = create_global_profile_data_instruction(&user1.pubkey());
    process_transaction(&mut context, &Vec::from([create_user1_global_data]), &Vec::from([&user1])).await.unwrap();

    let create_user1_local_data = create_local_profile_data_instruction(&user1.pubkey(), &launchpad_address);
    process_transaction(&mut context, &Vec::from([create_user1_local_data]), &Vec::from([&user1])).await.unwrap();

    // create user 2 profile
    let create_user2_profile_data = create_global_profile_data_instruction(&user2.pubkey());
    process_transaction(&mut context, &Vec::from([create_user2_profile_data]), &Vec::from([&user2])).await.unwrap();

    let create_user2_local_data = create_local_profile_data_instruction(&user2.pubkey(), &launchpad_address);
    process_transaction(&mut context, &Vec::from([create_user2_local_data]), &Vec::from([&user2])).await.unwrap();

    context.set_sysvar(&Clock{
        epoch: 1,
        epoch_start_timestamp: 0,
        slot: 1,
        leader_schedule_epoch: 1,
        unix_timestamp: time+20
    });
    let user1_register_data = register_data_instruction(&launchpad_address, &user1.pubkey(), &user1_global_profile, &user1_local_profile, 0, user1_proofs);
    process_transaction(&mut context, &Vec::from([user1_register_data]), &Vec::from([&user1])).await.unwrap();
    let user2_register_data = register_data_instruction(&launchpad_address, &user2.pubkey(), &user2_global_profile, &user2_local_profile, 1, user2_proofs);
    process_transaction(&mut context, &Vec::from([user2_register_data]), &Vec::from([&user2])).await.unwrap();

    // mint token to launchpad
    let (launchpad_signer,_) = find_launchpad_signer_address(launchpad_address);
    let launchpad_c98_token_account = create_associated_token_account(&mut context, &launchpad_signer, &c98_mint.pubkey()).await.unwrap();
    mint_tokens(&mut context, &c98_mint.pubkey(), &launchpad_c98_token_account, 1_000_000_000_000, &payer_wallet.pubkey(), Some(&payer_wallet)).await.unwrap();

    let set_blacklist_data = set_blacklist_data_instruction(&payer_wallet.pubkey(), &user1.pubkey(), &user1_global_profile, true);
    process_transaction(&mut context, &Vec::from([set_blacklist_data]), &Vec::from([&payer_wallet])).await.unwrap();
    airdrop(&mut context, &launchpad_signer, 10_000_000_000).await.unwrap();

    context.set_sysvar(&Clock{
        epoch: 11,
        epoch_start_timestamp: 10,
        slot: 2,
        leader_schedule_epoch: 10,
        unix_timestamp: time+1001
    });
    let set_blacklist_data1 = set_blacklist_data_instruction(&payer_wallet.pubkey(), &user1.pubkey(), &user1_global_profile, false);
    process_transaction(&mut context, &Vec::from([set_blacklist_data1]), &Vec::from([&payer_wallet])).await.unwrap();

    let user1_redeem_token_by_sol =  redeem_by_sol_data_instruction(&user1.pubkey(), &launchpad_address, &launchpad_signer, &user1_global_profile, &user1_local_profile, &user1_c98_token_account, &launchpad_c98_token_account, 20);
  process_transaction(&mut context, &Vec::from([user1_redeem_token_by_sol]), &Vec::from([&user1])).await.unwrap();
}

#[tokio::test]
async fn user_blacklist_register_claim(){
    println!(">>>>>>>>>> FAIL: user in blacklist redeem <<<<<<<<<<<<<<<");
    let mut context = c98_starship_program_test().start_with_context().await;

    let payer_wallet = Keypair::from_bytes(&[71,26,80,250,238,134,95,254,8,150,193,132,34,14,180,32,84,46,14,119,150,214,118,184,137,163,83,244,236,197,16,15,50,30,210,179,166,74,56,169,144,205,219,137,241,5,133,57,235,192,67,165,11,113,84,123,27,201,254,0,128,223,9,85]).unwrap();
    airdrop(&mut context, &payer_wallet.pubkey(), 10_000_000_000).await.unwrap();

    let feed_path: Vec<u8> = (0..10).map(|_| { rand::random::<u8>() }).collect();
    let launchpad_path =   feed_path.clone();
        // create user to claim
    let user1 = Keypair::new();
    airdrop(&mut context, &user1.pubkey(), 10_000_000_000).await.unwrap();
    let user2 = Keypair::new();
    airdrop(&mut context, &user2.pubkey(), 10_000_000_000).await.unwrap();

    let c98_mint = Keypair::new();
    create_mint(&mut context, &c98_mint, &payer_wallet.pubkey(), None).await.unwrap();
    let user1_c98_token_account = create_associated_token_account(&mut context, &user1.pubkey(), &c98_mint.pubkey()).await.unwrap();

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
    );

    let user1_data = starship::state::WhitelistParams{
      index: 0,
      address: user1.pubkey(),
    };

    let user2_data = starship::state::WhitelistParams{
      index: 1,
      address: user2.pubkey(),
    };
    let hash_user1 = hash_whitelist(user1_data);
    let hash_user2 = hash_whitelist(user2_data);
    let merkle_root: [u8; 32] = if hash_user1 < hash_user2 { hashv(&[&hash_user1, &hash_user2]).to_bytes() }
                                                      else { hashv(&[&hash_user2, &hash_user1]).to_bytes() };

    let mut user1_proofs: Vec<[u8; 32]> =  Vec::new();
    user1_proofs.push(hash_user2);

    let mut user2_proofs: Vec<[u8; 32]> =  Vec::new();
    user2_proofs.push(hash_user1);

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64 ;
    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 1, 10, 100, 1000, time + 10, time + 100, time + 1000, time + 2000, Some(merkle_root));

    process_transaction(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await.unwrap();

    let (launchpad_purchase_address, _) =  find_launchpad_purchase_address(launchpad_address, c98_mint.pubkey());
    let create_launchpad_purchase_data = create_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_address, &c98_mint.pubkey());
    let set_launchpad_purchase_data = set_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_purchase_address, 1000, 1000, 1, 10, 1000);

    process_transaction(&mut context, &Vec::from([create_launchpad_purchase_data, set_launchpad_purchase_data]), &Vec::from([&payer_wallet])).await.unwrap();

    let set_launchpad_status_data_instruction = set_launchpad_status_data_instruction(&payer_wallet.pubkey(), &launchpad_address, true);
    process_transaction(&mut context, &Vec::from([set_launchpad_status_data_instruction]), &Vec::from([&payer_wallet])).await.unwrap();


    let(user1_global_profile,_) = find_global_profile_address(user1.pubkey());
    let(user2_global_profile,_) = find_global_profile_address(user2.pubkey());

    let(user1_local_profile,_) = find_local_profile_address(launchpad_address, user1.pubkey());
    let(user2_local_profile,_) = find_local_profile_address(launchpad_address, user2.pubkey());

    // create user 1 profile
    let create_user1_global_data = create_global_profile_data_instruction(&user1.pubkey());
    process_transaction(&mut context, &Vec::from([create_user1_global_data]), &Vec::from([&user1])).await.unwrap();

    let create_user1_local_data = create_local_profile_data_instruction(&user1.pubkey(), &launchpad_address);
    process_transaction(&mut context, &Vec::from([create_user1_local_data]), &Vec::from([&user1])).await.unwrap();

    // create user 2 profile
    let create_user2_profile_data = create_global_profile_data_instruction(&user2.pubkey());
    process_transaction(&mut context, &Vec::from([create_user2_profile_data]), &Vec::from([&user2])).await.unwrap();

    let create_user2_local_data = create_local_profile_data_instruction(&user2.pubkey(), &launchpad_address);
    process_transaction(&mut context, &Vec::from([create_user2_local_data]), &Vec::from([&user2])).await.unwrap();

    context.set_sysvar(&Clock{
        epoch: 1,
        epoch_start_timestamp: 0,
        slot: 1,
        leader_schedule_epoch: 1,
        unix_timestamp: time+20
    });
    let user1_register_data = register_data_instruction(&launchpad_address, &user1.pubkey(), &user1_global_profile, &user1_local_profile, 0, user1_proofs);
    process_transaction(&mut context, &Vec::from([user1_register_data]), &Vec::from([&user1])).await.unwrap();
    let user2_register_data = register_data_instruction(&launchpad_address, &user2.pubkey(), &user2_global_profile, &user2_local_profile, 1, user2_proofs);
    process_transaction(&mut context, &Vec::from([user2_register_data]), &Vec::from([&user2])).await.unwrap();

    // mint token to launchpad
    let (launchpad_signer,_) = find_launchpad_signer_address(launchpad_address);
    let launchpad_c98_token_account = create_associated_token_account(&mut context, &launchpad_signer, &c98_mint.pubkey()).await.unwrap();
    mint_tokens(&mut context, &c98_mint.pubkey(), &launchpad_c98_token_account, 1_000_000_000_000, &payer_wallet.pubkey(), Some(&payer_wallet)).await.unwrap();

    let set_blacklist_data = set_blacklist_data_instruction(&payer_wallet.pubkey(), &user1.pubkey(), &user1_global_profile, true);
    process_transaction(&mut context, &Vec::from([set_blacklist_data]), &Vec::from([&payer_wallet])).await.unwrap();
    airdrop(&mut context, &launchpad_signer, 10_000_000_000).await.unwrap();
    let user1_redeem_token_by_sol =  redeem_by_sol_data_instruction(&user1.pubkey(), &launchpad_address, &launchpad_signer, &user1_global_profile, &user1_local_profile, &user1_c98_token_account, &launchpad_c98_token_account, 20);
    let result = process_transaction_with_error(&mut context, &Vec::from([user1_redeem_token_by_sol]), &Vec::from([&user1])).await;
    match result {
      Ok(_) => assert!(result.is_err()),
      Err(_) => {},
    };
}

