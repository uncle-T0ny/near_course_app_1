use near_sdk::borsh;
use near_sdk::borsh::BorshDeserialize;
use near_sdk::borsh::BorshSerialize;
use near_sdk::env;
use near_sdk::ext_contract;
use near_sdk::log;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::PanicOnDefault;
use near_sdk::Promise;

pub const TGAS: u64 = 1_000_000_000_000;
pub const BASE_GAS: u64 = 25 * TGAS;

#[ext_contract(ext_staking_pool)]
pub trait ExtUser {
    fn send(&mut self, amount: u128);
}

#[derive(
    Debug, BorshDeserialize, BorshSerialize, Deserialize, Serialize, PanicOnDefault, Clone,
)]
#[serde(crate = "near_sdk::serde")]
pub struct DonationItem {
    amount: u128,
    donated: u128,
    receiver: String,
}

#[derive(
    Debug, BorshDeserialize, BorshSerialize, Deserialize, Serialize, PanicOnDefault, Clone,
)]
#[serde(crate = "near_sdk::serde")]
pub struct DonationItemView {
    amount: String,
    donated: String,
    receiver: String,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Donation {
    dontaions: Vec<DonationItem>,
}

#[near_bindgen]
impl Donation {
    #[init]
    pub fn new() -> Self {
        let mut this = Self { dontaions: vec![] };
        this
    }

    pub fn add_donation(&mut self, amount: u128, receiver: String) -> bool {
        self.dontaions.push(DonationItem {
            amount: amount * 10u128.pow(24),
            receiver,
            donated: 0,
        });
        true
    }

    #[payable]
    pub fn donate(&mut self, receiver: String) -> bool {
        log!(
            "User sent {} nears for {}",
            env::attached_deposit(),
            receiver
        );
        // self.dontaions.push(DonationItem { amount, receiver, donated: 0 });
        self.dontaions.iter_mut().for_each(|el| {
            if el.receiver == receiver {
                el.donated += env::attached_deposit();
            }

            if el.donated >= el.amount {
                Promise::new(AccountId::new_unchecked(el.receiver.clone())).transfer(el.donated);
            }
        });

        true
    }

    pub fn get_donations(&self) -> Vec<DonationItemView> {
        self.dontaions
            .iter()
            .map(|el| DonationItemView {
                amount: el.amount.to_string(),
                donated: el.donated.to_string(),
                receiver: el.receiver.clone(),
            })
            .collect()
    }
}
