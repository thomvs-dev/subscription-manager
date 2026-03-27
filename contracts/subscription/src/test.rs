#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, testutils::Ledger};
use soroban_sdk::token::Client as TokenClient;

fn setup_test() -> (Env, SubscriptionContractClient<'static>, TokenClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    
    // Explicitly set ledger to bypass 0 timestamp edge cases
    env.ledger().with_mut(|li| {
        li.timestamp = 1000;
        li.sequence_number = 1;
    });

    let admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let token = TokenClient::new(&env, &token_address);
    let token_admin = soroban_sdk::token::StellarAssetClient::new(&env, &token_address);

    let contract_id = env.register(SubscriptionContract, ());
    let contract = SubscriptionContractClient::new(&env, &contract_id);

    let provider = Address::generate(&env);
    let user = Address::generate(&env);

    token_admin.mint(&user, &1000);

    (env, contract, token, provider, user)
}

#[test]
fn test_create_plan_and_subscribe() {
    let (_env, contract, token, provider, user) = setup_test();

    // 1. Provider creates a plan (ID 1: 50 XLM per cycle, 60 seconds interval)
    contract.create_plan(&provider, &1, &50, &60);

    // 2. User subscribes pre-funding with 200 XLM
    contract.subscribe(&user, &1, &token.address, &200);

    // Verifications
    assert_eq!(token.balance(&user), 800);
    assert_eq!(token.balance(&contract.address), 200);
}

#[test]
fn test_collect_payment() {
    let (env, contract, token, provider, user) = setup_test();

    contract.create_plan(&provider, &1, &50, &60);
    contract.subscribe(&user, &1, &token.address, &200);

    // Fast forward 60 seconds
    env.ledger().with_mut(|li| {
        li.timestamp += 61;
    });

    // Provider collects payment
    contract.collect(&provider, &user, &1);

    // Provider should have 50 XLM
    assert_eq!(token.balance(&provider), 50);
    // Contract should have 150 XLM left
    assert_eq!(token.balance(&contract.address), 150);
}

#[test]
#[should_panic(expected = "Wait for the next billing cycle")]
fn test_collect_too_early_fails() {
    let (env, contract, token, provider, user) = setup_test();

    contract.create_plan(&provider, &1, &50, &60);
    contract.subscribe(&user, &1, &token.address, &200);

    // Collect first payment (allowed immediately)
    contract.collect(&provider, &user, &1);

    // Only 30 seconds passed since first collect
    env.ledger().with_mut(|li| {
        li.timestamp += 30;
    });

    // Should panic because it's too early for the SECOND collect
    contract.collect(&provider, &user, &1);
}

#[test]
fn test_cancel_and_refund() {
    let (_env, contract, token, provider, user) = setup_test();

    contract.create_plan(&provider, &1, &50, &60);
    contract.subscribe(&user, &1, &token.address, &200);

    // User cancels the subscription immediately
    contract.cancel(&user, &1);

    // Balance completely refunded to user
    assert_eq!(token.balance(&user), 1000);
    assert_eq!(token.balance(&contract.address), 0);
}
