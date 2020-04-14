# driver_async
Async backend/runtime for Rust Drivers. Mostly just a wrapper for async mpsc/locks/etc so tokio-io or async-std can be targeted. 

WIP and so far just a move from the `btle` crate async internals into its own crate (this repo). Mostly for personal use.
