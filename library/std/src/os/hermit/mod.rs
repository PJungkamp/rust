#![stable(feature = "rust1", since = "1.0.0")]

pub mod ffi;
pub mod io;

#[stable(feature = "rust1", since = "1.0.0")]
pub use hermit_abi as abi;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::io::{FromAbi,AsAbi,IntoAbi};
}
