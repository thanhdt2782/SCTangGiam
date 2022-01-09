use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    pub fn tang(&mut self) {
        self.val += 1;
        let log_message = format!("Da tang so len {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }
    pub fn giam(&mut self) {
        self.val -= 1;
        let log_message = format!("Da giam so xuong {}", self.val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        env::log(b"Reset counter lai thanh 0");
    }
}

fn after_counter_change() {
    env::log("Da thay doi thanh cong (luu y range tu -128 -> 127).".as_bytes());
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::serde::export::TryFrom;

    // simple helper function to take a string literal and return a ValidAccountId
    fn to_valid_account(account: &str) -> ValidAccountId {
        ValidAccountId::try_from(account.to_string()).expect("Invalid account")
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(to_valid_account("foo.near"));
        testing_env!(context.build());
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn giam() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { val: 0 };
        contract.giam();
        println!("Value after decrement: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        // confirm that we received -1 when calling get_num
        assert_eq!(0, contract.get_num());
    }
}
