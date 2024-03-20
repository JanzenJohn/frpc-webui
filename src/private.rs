use std::fmt::Debug;

#[derive(Clone)]
pub struct Private<T> {
    inner: T,
}

impl Debug for Private<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<SECRETS_SHOW=1 TO UNBLUR>")
    }
}
pub trait Privatable<T> {
    fn make_private(self) -> Private<T>;
}

pub trait UnPrivatable<T> {
    fn make_unprivate(self) -> T;
}

impl<T> UnPrivatable<T> for Private<T> {
    fn make_unprivate(self) -> T {
        self.inner
    }
}

impl<T> Privatable<T> for T {
    fn make_private(self) -> Private<T> {
        Private { inner: self }
    }
}
