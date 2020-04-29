pub mod asyncs;
pub mod time;
extern crate alloc;

/// Basic `ConversionError` for when primitives can't be converted to/from bytes because of invalid
/// states. Most modules use their own errors for when there is more information to report.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct ConversionError(pub ());
