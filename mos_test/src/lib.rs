#![no_std]

pub use mos_test_macros::tests;

/// Private implementation details used by the proc macro.
#[doc(hidden)]
pub mod export;

use ufmt_stdio::ufmt::uDisplay;

pub trait TestOutcome: uDisplay {
    fn is_success(&self) -> bool;
}

pub struct OutcomeWrapper<T>(pub T);

impl uDisplay for OutcomeWrapper<()> {
    fn fmt<W>(&self, fmt: &mut ufmt_stdio::ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_stdio::uWrite + ?Sized,
    {
        fmt.write_str("()")
    }
}
impl<T: uDisplay, E: uDisplay> uDisplay for OutcomeWrapper<Result<T, E>> {
    fn fmt<W>(&self, fmt: &mut ufmt_stdio::ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_stdio::uWrite + ?Sized,
    {
        match &self.0 {
            Ok(v) => v.fmt(fmt),
            Err(v) => v.fmt(fmt),
        }
    }
}

impl<T: uDisplay, E: uDisplay> TestOutcome for OutcomeWrapper<Result<T, E>> {
    fn is_success(&self) -> bool {
        self.0.is_ok()
    }
}

impl TestOutcome for OutcomeWrapper<()> {
    fn is_success(&self) -> bool {
        true
    }
}
