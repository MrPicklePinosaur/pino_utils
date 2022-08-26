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

/// Unwraps result type or returns with error function
#[macro_export]
macro_rules! ok_or_return_msg {
    ( $e:expr, $f:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                $f(e);
                return
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
                continue
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
                return
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
                continue
            },
        }
    };
}
