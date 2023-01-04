use crate::TestOutcome;
use ufmt_stdio::*;
// pub fn exit() -> ! {
//     loop {
//         cortex_m::asm::bkpt()
//     }
// }

pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note = if should_error {
            "`#[should_error]` "
        } else {
            ""
        };

        println!("{}test failed with outcome: {}", note, outcome);
    }
}
