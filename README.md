# Faucet Contract

```rust
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
```

<br />

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

<br />

## 1. Build and Deploy the Contract
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
./deploy.sh
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```

<br />

## 2. Get methods

`get_records` is a read-only method (aka `view` method).

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the greeting
near view <dev-account> get_records
```

<br />

## 3. Store a New Greeting
`send_near` changes the contract's state, for which it is a `change` method.

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction.

```bash
# Use near-cli to set a new greeting
near call <dev-account> send_near '{"amount":1,"age":23}' --accountId <dev-account>
```

**Tip:** If you would like to call `send_near` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.
