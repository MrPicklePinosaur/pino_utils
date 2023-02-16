//! Result + Option = Resopt
//!
//! Experimental enum for nullable results

pub enum Resopt<T, E> {
    Ok(T),
    None,
    Err(E),
}

impl<T, E> Resopt<T, E>
where
    E: std::fmt::Debug,
{
    pub fn unwrap(self) -> T {
        match self {
            Resopt::Ok(v) => v,
            Resopt::None => panic!(""),
            Resopt::Err(e) => panic!(""),
        }
    }

    pub fn is_ok(&self) -> bool {
        if let Resopt::Ok(_) = self {
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

    pub fn option(self) -> Option<T> {
        match self {
            Resopt::Ok(v) => Some(v),
            Resopt::None => None,
            Resopt::Err(_) => None,
        }
    }

    // pub fn result(self) -> Result<T, E> {

    // }
}
