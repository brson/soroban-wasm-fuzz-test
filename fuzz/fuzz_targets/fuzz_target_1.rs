#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::*;
use soroban_sdk::testutils::arbitrary::Arbitrary;
use soroban_sdk::testutils::arbitrary::arbitrary;
use soroban_sdk::testutils::Address as _;

fuzz_target!(|input: Input| {
    run(input);
});

mod example_token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
}

#[derive(Arbitrary, Debug)]
struct Input {
    amount: i128,
}

fn run(input: Input) {
    let env = Env::default();

    env.mock_all_auths();
    env.budget().reset_unlimited();

    let admin_addr = Address::generate(&env);
    let token_addr = env.register_contract_wasm(None, example_token::WASM);

    let token_client = example_token::Client::new(&env, &token_addr);

    token_client.initialize(
        &admin_addr,
        &10,
        &String::from_str(&env, "Token"),
        &String::from_str(&env, "TKN"),
    );

    let _ = token_client.try_mint(
        &admin_addr,
        &input.amount,
    );
}
