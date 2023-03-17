use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId,Promise};
use serde::{Serialize,Deserialize};


// Receivers structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Receivers {
    pub account: AccountId,
    pub amount: u128,
    pub age: u8,
    pub timestamp: u64,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FaucetContract {
    records: Vec<Receivers>,
    counter: u32,
}

// Define the default, which automatically initializes the contract
impl Default for FaucetContract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl FaucetContract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            records: Vec::new(),
            counter: 0,
        }
    }

    pub fn send_near(&mut self,amount:u128,age:u8) {
        // for i in self.records.clone() {
        //     if i.account == env::signer_account_id() {
        //         panic!("You can't get paid more than once");
        //     }
        // }
        let receiver = Receivers {
            account: env::signer_account_id(),
            amount:amount,
            age: age,
            timestamp: env::block_timestamp_ms(),
        };
        self.records.push(receiver);
        self.counter += 1;
        Promise::new(env::signer_account_id()).transfer(amount*1000000000000000000000000);
    }

    pub fn get_records(&self) -> Vec<Receivers> {
        self.records.clone()
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }
}