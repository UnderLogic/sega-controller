#![no_std]

#[cfg(feature = "mega-drive")]
pub mod mega_drive;

pub type ControllerResult<T, E> = Result<T, Error<E>>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error<E> {
    Other(E),
    NotPresent,
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::Other(error)
    }
}
