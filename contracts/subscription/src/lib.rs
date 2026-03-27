#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};
use soroban_sdk::token::Client as TokenClient;

#[contracttype]
pub enum DataKey {
    Plan(u32),
    Sub(u32, Address),
}

#[contracttype]
pub struct Plan {
    pub provider: Address,
    pub amount: i128,
    pub interval: u64,
}

#[contracttype]
pub struct Subscription {
    pub token: Address,
    pub balance: i128,
    pub next_payment_time: u64,
}

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {
    pub fn create_plan(env: Env, provider: Address, plan_id: u32, amount: i128, interval: u64) {
        provider.require_auth();

        let key = DataKey::Plan(plan_id);
        if env.storage().instance().has(&key) {
            panic!("Plan already exists");
        }

        let plan = Plan {
            provider,
            amount,
            interval,
        };
        env.storage().instance().set(&key, &plan);
    }

    pub fn subscribe(env: Env, user: Address, plan_id: u32, token: Address, prefund_amount: i128) {
        user.require_auth();

        let plan_key = DataKey::Plan(plan_id);
        if !env.storage().instance().has(&plan_key) {
            panic!("Plan does not exist");
        }

        let sub_key = DataKey::Sub(plan_id, user.clone());
        if env.storage().instance().has(&sub_key) {
            panic!("Already subscribed, you can top-up later (not implemented)");
        }

        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&user, &env.current_contract_address(), &prefund_amount);

        let sub = Subscription {
            token,
            balance: prefund_amount,
            next_payment_time: env.ledger().timestamp(), // First payment can be collected immediately
        };
        env.storage().instance().set(&sub_key, &sub);
    }

    pub fn collect(env: Env, provider: Address, user: Address, plan_id: u32) {
        provider.require_auth();

        let plan_key = DataKey::Plan(plan_id);
        let plan: Plan = env.storage().instance().get(&plan_key).unwrap_or_else(|| panic!("Plan not found"));
        
        if plan.provider != provider {
            panic!("Not authorized to collect");
        }

        let sub_key = DataKey::Sub(plan_id, user.clone());
        let mut sub: Subscription = env.storage().instance().get(&sub_key).unwrap_or_else(|| panic!("Subscription not found"));

        if env.ledger().timestamp() < sub.next_payment_time {
            panic!("Wait for the next billing cycle");
        }

        if sub.balance < plan.amount {
            panic!("Insufficient prefunded balance");
        }

        // Deduct balance and update next payment time
        sub.balance -= plan.amount;
        sub.next_payment_time = env.ledger().timestamp() + plan.interval;
        env.storage().instance().set(&sub_key, &sub);

        // Send payment to provider
        let token_client = TokenClient::new(&env, &sub.token);
        token_client.transfer(&env.current_contract_address(), &provider, &plan.amount);
    }

    pub fn cancel(env: Env, user: Address, plan_id: u32) {
        user.require_auth();

        let sub_key = DataKey::Sub(plan_id, user.clone());
        let sub: Subscription = env.storage().instance().get(&sub_key).unwrap_or_else(|| panic!("Subscription not found"));

        // Refund remaining balance
        if sub.balance > 0 {
            let token_client = TokenClient::new(&env, &sub.token);
            token_client.transfer(&env.current_contract_address(), &user, &sub.balance);
        }

        // Remove subscription
        env.storage().instance().remove(&sub_key);
    }
}

mod test;
