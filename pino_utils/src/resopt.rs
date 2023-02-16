//! Result + Option = Resopt
//!
//! Experimental enum for nullable results

// TODO support ? operator (via std::ops::Try)

pub enum Resopt<T, E> {
    Some(T),
    None,
    Err(E),
}

impl<T, E> Resopt<T, E> {
    pub fn expect(self, msg: &str) -> T {
        unimplemented!()
    }
    pub fn unwrap(self) -> T {
        match self {
            Resopt::Some(v) => v,
            Resopt::None => panic!(""),
            Resopt::Err(_) => panic!(""),
        }
    }
    pub fn unwrap_or(self) {
        unimplemented!()
    }
    pub fn unwrap_default(self) {
        unimplemented!()
    }
    pub fn unwrap_else(self) {
        unimplemented!()
    }

    pub fn as_mut() {
        unimplemented!()
    }
    pub fn as_deref() {
        unimplemented!()
    }
    pub fn as_deref_mut() {
        unimplemented!()
    }

    pub fn is_some(&self) -> bool {
        if let Resopt::Some(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_none(&self) -> bool {
        if let Resopt::None = self {
            true
        } else {
            false
        }
    }
    pub fn is_err(&self) -> bool {
        if let Resopt::Err(_) = self {
            true
        } else {
            false
        }
    }

    pub fn some(self) -> Option<T> {
        match self {
            Resopt::Some(v) => Some(v),
            Resopt::None => None,
            Resopt::Err(_) => None,
        }
    }

    pub fn err(self) -> Option<E> {
        match self {
            Resopt::Some(_) => None,
            Resopt::None => None,
            Resopt::Err(e) => Some(e),
        }
    }
}

impl<T, E> From<Option<T>> for Resopt<T, E> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Resopt::Some(v),
            None => Resopt::None,
        }
    }
}

impl<T, E> From<Result<T, E>> for Resopt<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Resopt::Some(v),
            Err(e) => Resopt::Err(e),
        }
    }
}

impl<T, E> From<Result<Option<T>, E>> for Resopt<T, E> {
    fn from(value: Result<Option<T>, E>) -> Self {
        match value {
            Ok(opt) => Resopt::from(opt),
            Err(e) => Resopt::Err(e),
        }
    }
}

impl<T, E> std::fmt::Debug for Resopt<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resopt::Some(v) => write!(f, "Some({:?})", v),
            Resopt::None => write!(f, "None"),
            Resopt::Err(e) => write!(f, "Err({:?})", e),
        }
    }
}
