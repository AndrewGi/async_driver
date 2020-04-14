pub mod asyncs;
pub mod bytes;
pub mod error;
extern crate alloc;

/// Basic `ConversionError` for when primitives can't be converted to/from bytes because of invalid
/// states. Most modules use their own errors for when there is more information to report.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ConversionError(pub ());