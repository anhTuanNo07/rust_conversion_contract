use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Conversion {
    // implementation here...
}

#[near_bindgen]
impl Conversion {
    // concert celcius degree to fahrenheit degree
    pub fn celcius_to_fahrenheit(degree: f64) -> f64 {
        (degree * 1.8) + 32.0
    }

    // convert fahrenheit degree to celcius degree
    pub fn fahrenheit_to_celcius(degree: f64) -> f64 {
        (degree - 32.0)/1.8
    }

    // conversion dollar to vnd
    // 1 USD approximately equal to 23,000 vnd
    pub fn dollar_to_vnd(dollar: f64) -> f64 {
        dollar * 23000.0
    }

    // conversion vnd to dollar
    pub fn vnd_to_dollar(vnd: f64) -> f64 {
        vnd / 23000.0
    }

    // conversion inch to centimeter
    pub fn inch_to_cm(inch: f64) -> f64 {
        inch * 2.54
    }

    // conversion centimeter to inch
    pub fn cm_to_inch(cm: f64) -> f64 {
        cm / 2.54
    }

    // conversion kilogram to pound
    pub fn kg_to_lib(kg: f64) -> f64 {
        kg * 2.205
    }

    // conversion pound to kilogram
    pub fn lib_to_kb(lib: f64) -> f64 {
        lib / 2.205
    }

    // convert km/h to m/s
    // kph is kilometer per hour
    // mps is meter per second
    pub fn kph_to_mps(kph: f64) -> f64 {
        kph * 3.6
    }

    // convert m/s to km/h
    pub fn mps_to_kph(mps: f64) -> f64 {
        mps / 3.6
    }

    // conversion joule to kal
    pub fn joule_to_cal(joule: f64) -> f64 {
        joule * 0.239
    }

    // conversion kal to joule 
    pub fn cal_to_joule(cal: f64) -> f64 {
        cal / 0.239
    }

    // conversion watts to hoursepower
    pub fn watts_to_hp(watts: f64) -> f64 {
        watts / 735.499
    }

    // conversion hoursepower to watts
    pub fn hp_to_watts(hp: f64) -> f64 {
        hp * 735.499
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn celcius_to_fahrenheit() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);

        let fahrenheit_result = Conversion::celcius_to_fahrenheit(0.0);
        println!("Value after conversion : {}", fahrenheit_result);
        assert_eq!(32.0, fahrenheit_result);
    }

    #[test]
    fn fahrenheit_to_celcius() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);

        let fahrenheit_result = Conversion::fahrenheit_to_celcius(32.0);
        println!("Value after conversion : {}", fahrenheit_result);
        assert_eq!(0.0, fahrenheit_result);
    }

    #[test]
    fn dollar_to_vnd() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);

        let fahrenheit_result = Conversion::dollar_to_vnd(1.0);
        println!("Value after conversion : {}", fahrenheit_result);
        assert_eq!(23000.0, fahrenheit_result);
    }

    #[test]
    fn vnd_to_dollar() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);

        let fahrenheit_result = Conversion::vnd_to_dollar(23000.0);
        println!("Value after conversion : {}", fahrenheit_result);
        assert_eq!(1.0, fahrenheit_result);
    }
}