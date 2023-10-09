#![allow(unused_imports)]
//! This module contains macros for logging to stdout a trace of wall-clock time required
//! to execute annotated code. One can use this code as follows:
//! ```
//! use ark_std::{start_timer, end_timer};
//! let start = start_timer!(|| "Addition of two integers");
//! let c = 5 + 7;
//! end_timer!(start);
//! ```
//! The foregoing code should log the following to stdout.
//! ```text
//! Start: Addition of two integers
//! End: Addition of two integers... 1ns
//! ```
//!
//! These macros can be arbitrarily nested, and the nested nature is made apparent
//! in the output. For example, the following snippet:
//! ```
//! use ark_std::{start_timer, end_timer};
//! let start = start_timer!(|| "Addition of two integers");
//! let start2 = start_timer!(|| "Inner");
//! let c = 5 + 7;
//! end_timer!(start2);
//! end_timer!(start);
//! ```
//! should print out the following:
//! ```text
//! Start: Addition of two integers
//!     Start: Inner
//!     End: Inner               ... 1ns
//! End: Addition of two integers... 1ns
//! ```
//!
//! Additionally, one can use the `add_to_trace` macro to log additional context
//! in the output.
pub use self::inner::*;

#[macro_use]
#[cfg(feature = "print-trace")]
pub mod inner {
    pub struct TimerInfo;

    #[macro_export]
    macro_rules! start_timer {
        ($msg:expr) => {
            $crate::perf_trace::TimerInfo
        };
    }
    #[macro_export]
    macro_rules! add_to_trace {
        ($title:expr, $msg:expr) => {
            let _ = $msg;
        };
    }

    #[macro_export]
    macro_rules! end_timer {
        ($time:expr, $msg:expr) => {
            let _ = $msg;
            let _ = $time;
        };
        ($time:expr) => {
            let _ = $time;
        };
    }
}

#[macro_use]
#[cfg(not(feature = "print-trace"))]
mod inner {
    pub struct TimerInfo;

    #[macro_export]
    macro_rules! start_timer {
        ($msg:expr) => {
            $crate::perf_trace::TimerInfo
        };
    }
    #[macro_export]
    macro_rules! add_to_trace {
        ($title:expr, $msg:expr) => {
            let _ = $msg;
        };
    }

    #[macro_export]
    macro_rules! end_timer {
        ($time:expr, $msg:expr) => {
            let _ = $msg;
            let _ = $time;
        };
        ($time:expr) => {
            let _ = $time;
        };
    }
}

mod tests {
    use super::*;

    #[test]
    fn print_start_end() {
        let start = start_timer!(|| "Hello");
        end_timer!(start);
    }

    #[test]
    fn print_add() {
        let start = start_timer!(|| "Hello");
        add_to_trace!(|| "HelloMsg", || "Hello, I\nAm\nA\nMessage");
        end_timer!(start);
    }
}
