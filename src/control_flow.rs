//! Control flow related macros

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
