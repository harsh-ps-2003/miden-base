#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use miden_objects::transaction::TransactionInputs;

mod executor;
pub use executor::{DataStore, TransactionExecutor, TransactionMastStore};

pub mod host;
pub use host::{TransactionHost, TransactionProgress};

mod prover;
pub use prover::{ProvingOptions, TransactionProver};

mod verifier;
pub use verifier::TransactionVerifier;

mod error;
pub use error::{
    AuthenticationError, DataStoreError, TransactionCompilerError, TransactionExecutorError,
    TransactionProverError, TransactionVerifierError, KERNEL_ERRORS,
};

pub mod auth;

#[cfg(feature = "testing")]
pub mod testing;

#[cfg(test)]
mod tests;

// RE-EXPORTS
// ================================================================================================
pub use miden_objects::utils;
