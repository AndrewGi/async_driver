//! Instant module for keeping track of time. Different systems have different clock sources so
//! this module generalizes over it. By default, it uses the `std::time::Instant` but it could use
//! a crystal oscillator clock (for ARM) or some other source.
use core::ops::Add;
pub use core::time::Duration;
use std::ops::Sub;

#[cfg(feature = "std")]
mod std_instant {
    use super::InstantTrait;
    use core::ops::Add;
    use core::time::Duration;
    use std::ops::Sub;

    #[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
    pub struct StdInstant(std::time::Instant);
    impl Add<Duration> for StdInstant {
        type Output = StdInstant;

        fn add(self, rhs: Duration) -> Self::Output {
            StdInstant(self.0 + rhs)
        }
    }
    impl Sub<Duration> for StdInstant {
        type Output = StdInstant;

        fn sub(self, rhs: Duration) -> Self::Output {
            StdInstant(self.0 - rhs)
        }
    }
    impl InstantTrait for StdInstant {
        fn now() -> Self {
            Self(std::time::Instant::now())
        }

        fn checked_duration_until(&self, later: Self) -> Option<Duration> {
            later.0.checked_duration_since(self.0)
        }

        fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
            self.0.checked_duration_since(earlier.0)
        }
    }
}
#[cfg(not(feature = "std"))]
type InternalInstant = DummyInstant;
#[cfg(feature = "std")]
type InternalInstant = std_instant::StdInstant;
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Instant(InternalInstant);

pub trait InstantTrait:
    Sized + Sub<Duration, Output = Self> + Add<Duration, Output = Self> + Clone + Copy + Ord + Eq
{
    fn now() -> Self;
    fn with_delay(delay: core::time::Duration) -> Self {
        Self::now() + delay
    }
    /// Returns `Some(self - other)` or `None` if `self > other`.
    fn checked_duration_until(&self, later: Self) -> Option<Duration>;
    /// Returns `Some(other - self)` or `None` if `other > self`.
    fn checked_duration_since(&self, earlier: Self) -> Option<Duration>;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct DummyInstant(());
impl Add<Duration> for DummyInstant {
    type Output = Self;

    fn add(self, _rhs: Duration) -> Self::Output {
        unimplemented!("dummy instant")
    }
}
impl Sub<Duration> for DummyInstant {
    type Output = Self;

    fn sub(self, _rhs: Duration) -> Self::Output {
        unimplemented!("dummy instant")
    }
}
impl InstantTrait for DummyInstant {
    fn now() -> Self {
        unimplemented!("dummy instant")
    }

    fn checked_duration_until(&self, _later: Self) -> Option<Duration> {
        unimplemented!("dummy instant")
    }

    fn checked_duration_since(&self, _earlier: Self) -> Option<Duration> {
        unimplemented!("dummy instant")
    }
}
impl Add<core::time::Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}
impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs)
    }
}
impl InstantTrait for Instant {
    fn now() -> Self {
        Instant(InternalInstant::now())
    }

    fn with_delay(delay: Duration) -> Self {
        Instant(InternalInstant::with_delay(delay))
    }

    fn checked_duration_until(&self, later: Self) -> Option<Duration> {
        self.0.checked_duration_until(later.0)
    }

    fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.0.checked_duration_since(earlier.0)
    }
}
