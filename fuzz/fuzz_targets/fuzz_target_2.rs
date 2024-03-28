#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::testutils::arbitrary::{arbitrary, Arbitrary};
use std::sync::Once;

fuzz_target!(|input: Input| {
    run(input);
});

#[derive(Arbitrary, Debug)]
struct Input {
    which_pc: u8,
}

extern "C" {
    fn __sanitizer_cov_8bit_counters_init(start: *const u8, stop: *const u8);
    fn __sanitizer_cov_pcs_init(pcs_beg: *const usize, pcs_end: *const usize);
}

static INIT: Once = Once::new();

static mut COUNTERS: [u8; 256] = [0; 256];
static mut PCS: [(usize, usize); 256] = [(0, 0); 256];

fn run(input: Input) {
    INIT.call_once(|| {
        unsafe {
            for i in 0..256 {
                let fn_entry_flag = i % 2;
                PCS[i].0 = i + 256;
                PCS[i].1 = fn_entry_flag;
            }

            __sanitizer_cov_8bit_counters_init(
                &COUNTERS[0],
                (&COUNTERS[255] as *const u8).offset(1),
            );

            __sanitizer_cov_pcs_init(
                &PCS[0] as *const (usize, usize) as *const usize,
                (&PCS[255] as *const (usize, usize)).offset(1) as *const usize,
            );
        }
    });

    unsafe {
        COUNTERS[input.which_pc as usize] += 1;
    }
}
