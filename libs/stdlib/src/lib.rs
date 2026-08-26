// Definitions of the core standard function modules for IEC61131-3

// A panic in these functions aborts the whole runtime process (most are `extern "C"`),
// so all panicking constructs are banned outside of tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable,
        clippy::indexing_slicing,
        clippy::string_slice,
        clippy::exit
    )
)]

pub mod arithmetic_functions;
pub mod bistable_functionblocks;
pub mod bit_num_conversion;
pub mod bit_shift_functions;
pub mod counters;
pub mod date_time_conversion;
pub mod date_time_extra_functions;
pub mod date_time_numeric_functions;
pub mod endianness_conversion_functions;
pub mod extra_functions;
pub mod flanks;
pub mod num_conversion;
pub mod string_conversion;
pub mod string_functions;
pub mod timers;
pub mod types;
pub mod utils;
pub mod validation_functions;
