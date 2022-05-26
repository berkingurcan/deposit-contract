use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};
use near_sdk::payable;
use near_sdk::{AccountId, Balance};
use near_sdk::{Promise, PromiseResult};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    let deposit_map: UnorderedMap::new(b"a".to_vec());
}

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn deposit() {
        let account_id = env::signer_account_id();
        let current_value = deposit_map.get(&account_id).unwrap_or(0);
        deposit_map.insert(&account_id, &(current_value + env::attached_deposit()));
    }

    #[payable]
    pub fn withdraw(amount: U128) -> Promise {
        let account_id = env::signer_account_id();
        let current_value = deposit_map.get(&account_id).unwrap_or(0);
        let amount_to_withdraw: Balance = amount.into();
        
        assert!(
            (amount_to_withdraw <= current_value) && (amount_to_withdraw > 0),
            "Amount of withdrawal is invalid",
        );
        
        Promise::new(env::signer_account_id()).transfer(amount_to_withdraw)
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
