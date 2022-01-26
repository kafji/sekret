#![no_std]

use core::fmt;

/// Secret container.
///
/// A simple container that will redact its content from being printed.
///
/// ```rust
/// let token = sekret::Secret("secret_token");
/// println!("Secret token is: `{token}`.");
/// ```
#[derive(PartialEq, Clone)]
pub struct Secret<T>(pub T);

impl<T> Copy for Secret<T> where T: Copy {}

impl<T> From<T> for Secret<T> {
    fn from(s: T) -> Self {
        Self(s)
    }
}

impl<T> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Secret").field(&"█████").finish()
    }
}

impl<T> fmt::Display for Secret<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "█████")
    }
}

impl<T> Secret<T> {
    /// Convert `&Secret<T>` to `Secret<&T>`.
    pub fn as_ref(&self) -> Secret<&T> {
        Secret(&self.0)
    }

    /// Map the contained value.
    pub fn map<U, F>(self, f: F) -> Secret<U>
    where
        F: FnOnce(T) -> U,
    {
        let v = f(self.0);
        Secret(v)
    }

    /// Unwrap `Secret` returning the contained value.
    pub fn unwrap(self) -> T {
        self.0
    }
}
