//! Control flow related macros
//!
//! Many times one would like to return from a function or continue iterating if an `Option` or
//! `Result` was not valid. These are some simple macros to facilitate that.
//!
//! ```rust
//! use pino_utils::ok_or_continue;
//!
//! fn main() {
//!     
//!     for str in vec!["1", "one", "2", "two"] {
//! 	     let num = ok_or_continue!(str.parse::<u32>());
//!     }
//!
//! }
//! ```

/// Unwraps result type or returns
#[macro_export]
macro_rules! ok_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}

/// Unwraps result type or returns with error function
#[macro_export]
macro_rules! ok_or_return_msg {
    ( $e:expr, $f:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $f(e);
                return;
            },
        }
    };
}

/// Unwraps result type or continues
#[macro_export]
macro_rules! ok_or_continue {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => continue,
        }
    };
}

/// Unwraps result type or continues with error function
#[macro_export]
macro_rules! ok_or_continue_msg {
    ( $e:expr, $f:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $f(e);
                continue;
            },
        }
    };
}

/// Unwraps option type or returns
#[macro_export]
macro_rules! some_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

/// Unwraps option type or returns with error function
#[macro_export]
macro_rules! some_or_return_msg {
    ( $e:expr, $f:expr ) => {
        match $e {
            Some(x) => x,
            None => {
                $f();
                return;
            },
        }
    };
}

/// Unwraps option type or continues
#[macro_export]
macro_rules! some_or_continue {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => continue,
        }
    };
}

/// Unwraps option type or continues with error function
#[macro_export]
macro_rules! some_or_continue_msg {
    ( $e:expr, $f:expr ) => {
        match $e {
            Some(x) => x,
            None => {
                $f();
                continue;
            },
        }
    };
}
