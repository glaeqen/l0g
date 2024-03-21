//! Logging facade crate that is supposed enable conditional logging macro
//! expansion depending on the chosen feature.
//!
//! - `defmt` feature will make the macros to expand to `defmt::<log_level>`
//! - `log` feature will make the macros to expand to `log::<log_level>`
//! - no feature is a noop
//!
//! Feature should be chosen *only* in the top level application. All
//! intermediate libraries should use this crate in a featureless fashion.

#![no_std]
#![warn(missing_docs)]

#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("defmt and log are mutually exclusive");

#[doc(hidden)]
pub mod __internal {
    #[cfg(feature = "defmt")]
    pub use defmt;

    #[cfg(feature = "log")]
    pub use log;
}

pub use l0g_macros::format;

/// Logs data at _error_ level.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::__internal::log::error!($s $(, $x)*)
        }
    }
}

/// Logs data at _error_ level.
#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            use $crate::__internal::defmt;
            $crate::__internal::defmt::error!($s $(, $x)*)
        }
    }
}

/// Logs data at _error_ level.
#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
macro_rules! error {
    ($s:literal $(, $x:expr)* $(,)?) => {};
}

/// Logs data at _warn_ level.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::__internal::log::warn!($s $(, $x)*)
        }
    }
}

/// Logs data at _warn_ level.
#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            use $crate::__internal::defmt;
            $crate::__internal::defmt::warn!($s $(, $x)*)
        }
    }
}

/// Logs data at _warn_ level.
#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
macro_rules! warn {
    ($s:literal $(, $x:expr)* $(,)?) => {};
}

/// Logs data at _info_ level.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::__internal::log::info!($s $(, $x)*)
        }
    }
}

/// Logs data at _info_ level.
#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            use $crate::__internal::defmt;
            $crate::__internal::defmt::info!($s $(, $x)*)
        }
    }
}

/// Logs data at _info_ level.
#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
macro_rules! info {
    ($s:literal $(, $x:expr)* $(,)?) => {};
}

/// Logs data at _debug_ level.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::__internal::log::debug!($s $(, $x)*)
        }
    }
}

/// Logs data at _debug_ level.
#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            use $crate::__internal::defmt;
            $crate::__internal::defmt::debug!($s $(, $x)*)
        }
    }
}

/// Logs data at _debug_ level.
#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
macro_rules! debug {
    ($s:literal $(, $x:expr)* $(,)?) => {};
}

/// Logs data at _trace_ level.
#[cfg(feature = "log")]
#[macro_export]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            $crate::__internal::log::trace!($s $(, $x)*)
        }
    }
}

/// Logs data at _trace_ level.
#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {
        {
            use $crate::__internal::defmt;
            $crate::__internal::defmt::trace!($s $(, $x)*)
        }
    }
}

/// Logs data at _trace_ level.
#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
macro_rules! trace {
    ($s:literal $(, $x:expr)* $(,)?) => {};
}
