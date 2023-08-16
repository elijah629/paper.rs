use std::process;

pub trait UnwrapAbort<T> {
    fn unwrap_abort(self) -> T;
}

impl<T> UnwrapAbort<T> for Option<T> {
    #[inline]
    fn unwrap_abort(self) -> T {
        match self {
            Some(t) => t,
            None => process::abort(),
        }
    }
}

impl<T, E> UnwrapAbort<T> for Result<T, E> {
    #[inline]
    fn unwrap_abort(self) -> T {
        match self {
            Ok(t) => t,
            Err(_) => process::abort(),
        }
    }
}
