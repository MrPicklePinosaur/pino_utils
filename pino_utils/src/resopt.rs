//! Result + Option = Resopt
//!
//! Experimental enum for nullable results

pub struct Resopt<T, E>(pub Result<Option<T>, E>);

impl<T, E> Resopt<T, E>
where
    E: std::fmt::Debug,
{
    pub fn unwrap(self) -> T {
        self.0.unwrap().unwrap()
    }

    pub fn is_some(&self) -> bool {
        self.0.is_ok()
    }
    pub fn is_none(&self) -> bool {
        self.0.is_ok()
    }
    pub fn is_err(&self) -> bool {
        self.0.is_err()
    }

    // pub fn result(self) -> Result<T, E> {

    // }
}

impl<T, E> std::ops::Deref for Resopt<T, E> {
    type Target = Result<Option<T>, E>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, E> std::ops::DerefMut for Resopt<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
