pub mod utils;
use std::time::{SystemTime, UNIX_EPOCH};

use anchor_lang::solana_program::keccak::hashv;

use solana_sdk::signature::{Keypair, Signer};
use solana_program_test::*;
use utils::helpers::*;
use utils::instructions::*;

#[tokio::test]
async fn set_launchpad_data_success(){
    println!(">>>>>>>>>> SUCCESS: set launchpad data succcess <<<<<<<<<<<<<<<");
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

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
      2000u64, // protocol fee
      10u64 // sharing fee
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
        .as_secs() as i64;
    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 1, 10, 100, 1000, time + 10, time + 100, time + 1000, time + 2000, Some(merkle_root));

    process_transaction(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await.unwrap();

}
#[tokio::test]
async fn set_launchpad_time_fail(){
    println!(">>>>>>>>>> FAIL: set launchpad time fail <<<<<<<<<<<<<<<");
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

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
      2000u64,
      10u64
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

    let time = 0;
    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 1, 10, 100, 1000, time + 10, time + 100, time + 1000, time + 2000, Some(merkle_root));

    let result = process_transaction_with_error(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await;
    match result {
      Ok(_) => assert!(result.is_err()),
      Err(_) => {},
    };
}
#[tokio::test]
async fn set_launchpad_price_fail(){
    println!(">>>>>>>>>> FAIL: set launchpad price fail <<<<<<<<<<<<<<<");
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

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
      2000u64,
      10u64
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
        .as_secs() as i64;
    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 0, 10, 100, 1000, time + 10, time + 100, time + 1000, time + 2000, Some(merkle_root));
    let result = process_transaction_with_error(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await;
    match result {
      Ok(_) => assert!(result.is_err()),
      Err(_) => {},
    };

}
#[tokio::test]
async fn set_launchpad_purchase_success(){
    println!(">>>>>>>>>> SUCCESS: set launchpad purchasse success <<<<<<<<<<<<<<<");
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
    let cusd_mint = Keypair::new();

    create_mint(&mut context, &c98_mint, &payer_wallet.pubkey(), None).await.unwrap();
    create_mint(&mut context, &cusd_mint, &payer_wallet.pubkey(), None).await.unwrap();

    let user1_cusd_token_account = create_associated_token_account(&mut context, &user1.pubkey(), &cusd_mint.pubkey()).await.unwrap();

    mint_tokens(&mut context, &cusd_mint.pubkey(), &user1_cusd_token_account, 1_000_000_000_000, &payer_wallet.pubkey(), Some(&payer_wallet)).await.unwrap();

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
      2000u64,
      10u64
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

    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 1, 10, 100, 1000, time + 10, time + 1000, time + 2000, time + 3000, Some(merkle_root));

    process_transaction(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await.unwrap();

    let (launchpad_purchase_address, _) =  find_launchpad_purchase_address(launchpad_address, cusd_mint.pubkey());
    let create_launchpad_purchase_data = create_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_address, &cusd_mint.pubkey());
    let set_launchpad_purchase_data = set_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_purchase_address, 1000, 1000, 1, 10, 1000);

    process_transaction(&mut context, &Vec::from([create_launchpad_purchase_data, set_launchpad_purchase_data]), &Vec::from([&payer_wallet])).await.unwrap();

}
#[tokio::test]
async fn set_launchpad_purchase_fail(){
    println!(">>>>>>>>>> FAIL: set launchpad purchasse fail <<<<<<<<<<<<<<<");
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
    let cusd_mint = Keypair::new();

    create_mint(&mut context, &c98_mint, &payer_wallet.pubkey(), None).await.unwrap();
    create_mint(&mut context, &cusd_mint, &payer_wallet.pubkey(), None).await.unwrap();

    let user1_cusd_token_account = create_associated_token_account(&mut context, &user1.pubkey(), &cusd_mint.pubkey()).await.unwrap();

    mint_tokens(&mut context, &cusd_mint.pubkey(), &user1_cusd_token_account, 1_000_000_000_000, &payer_wallet.pubkey(), Some(&payer_wallet)).await.unwrap();

      //create vault
    let (launchpad_address,_) = find_launchpad_address(feed_path);

    let create_launchpad_data = create_launchpad_data_instruction(
      &payer_wallet.pubkey(),
      launchpad_path,
      &c98_mint.pubkey(),
      2000u64,
      10u64
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

    let set_launchpad_data =  set_launchpad_data_instruction(&payer_wallet.pubkey(), &launchpad_address, 1000, 1, 10, 100, 1000, time + 10, time + 1000, time + 2000, time + 3000, Some(merkle_root));

    process_transaction(&mut context, &Vec::from([create_launchpad_data, set_launchpad_data]), &Vec::from([&payer_wallet])).await.unwrap();

    let (launchpad_purchase_address, _) =  find_launchpad_purchase_address(launchpad_address, cusd_mint.pubkey());
    let create_launchpad_purchase_data = create_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_address, &cusd_mint.pubkey());
    let set_launchpad_purchase_data = set_launchpad_purchase_data_instruction(&payer_wallet.pubkey(), &launchpad_purchase_address, 1000, 1, 0, 10, 1000);

    let result = process_transaction_with_error(&mut context, &Vec::from([create_launchpad_purchase_data, set_launchpad_purchase_data]), &Vec::from([&payer_wallet])).await;
    match result {
      Ok(_) => assert!(result.is_err()),
      Err(_) => {},
    };

}
