#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address, vec, Env};

#[test]
fn should_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ReclaimContract);
    let client = ReclaimContractClient::new(&env, &contract_id);
    let user = <soroban_sdk::Address as Address>::generate(&env);

    assert_eq!(client.instantiate(&user), ());
}

#[test]
fn should_add_epoch() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ReclaimContract);
    let client = ReclaimContractClient::new(&env, &contract_id);

    let user = <soroban_sdk::Address as Address>::generate(&env);
    assert_eq!(client.instantiate(&user), ());

    let mut witnesses = Vec::new(&env);

    let bytes = "244897572368eadf65bfbc5aec98d8e5443a9072".as_bytes();

    let items = &bytes[0..20]
        .try_into()
        .expect("slice with incorrect length");

    let witness = Witness {
        address: BytesN::<20>::from_array(&env, items),
        host: String::from_str(&env, "http"),
    };
    witnesses.push_back(witness);

    assert_eq!(client.add_epoch(&witnesses, &1_u128), ());
}
