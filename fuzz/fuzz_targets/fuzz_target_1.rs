#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::*;
use soroban_sdk::testutils::arbitrary::arbitrary::Arbitrary;

fuzz_target!(|input: Input| {
    run(input);
});

mod example_token {
    soroban_sdk::contract_import!(file = "../../example_token.wasm");
}

#[derive(Arbitrary, Default)]
struct Input {
    amount: i128,
}

fn run(input: Input) {
    let env = Env::default();

    env.register_contract_wasm(None, example_token::WASM);
}
