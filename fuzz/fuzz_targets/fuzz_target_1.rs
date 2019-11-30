#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate berk;

fuzz_target!(|data: &[u8]| {
    berk::eat(data)
});
