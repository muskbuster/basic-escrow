use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Escrow {
    deposits: LookupMap<AccountId, Balance>,
}

impl Default for Escrow {
    fn default() -> Self {
        env::panic_str("Escrow should be initialized before usage")
    }
}

#[near_bindgen]
impl Escrow {
    #[init]
    pub fn new() -> Self {
        if env::state_exists() {
            env::panic_str("ERR_ALREADY_INITIALIZED");
        }

        Self {
            deposits: LookupMap::new(b"r".to_vec()),
        }
    }

    pub fn deposits_of(&self, payee: &AccountId) -> Balance {
        return match self.deposits.get(payee) {
            Some(deposit) => deposit,
            None => 0,
        };
    }

    #[payable]
    pub fn deposit(&mut self) {
        if env::current_account_id() == env::signer_account_id() {
            env::panic_str("ERR_OWNER_SHOULD_NOT_DEPOSIT");
        }

        let amount = env::attached_deposit();
        let payee = env::signer_account_id();
        let current_balance = self.deposits_of(&payee);
        let new_balance = &(&current_balance + &amount);

        self.deposits.insert(&payee, new_balance);

        log!(
            "{} deposited {} NEAR tokens. New balance {}",
            &payee,
            amount,
            new_balance
        );
        // @TODO emit deposit event
    }

    #[payable]
    pub fn withdraw(&mut self) {
        let payee = env::signer_account_id();
        let payment = self.deposits_of(&payee);

        Promise::new(payee.clone()).transfer(payment);
        self.deposits.insert(&payee, &0);

        log!(
            "{} withdrawn {} NEAR tokens. New balance {}",
            &payee,
            payment,
            self.deposits_of(&payee)
        );
        // @TODO emit withdraw event
    }
}