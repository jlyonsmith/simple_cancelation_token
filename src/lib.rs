use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// The simplest implementation of an inter-thread cancelation token possible.
///
/// This is modeled after [tokio_util::sync::CancelationToken](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancelationToken.html)
/// and uses cloning to pass to other threads vs. having multiple separate structs with different functions.
#[derive(Clone)]
pub struct CancelationToken {
    canceled: Arc<AtomicBool>,
}

impl CancelationToken {
    /// Create a new cancelation token.  Clone it to pass it to another thread
    pub fn new() -> CancelationToken {
        let canceled = Arc::new(AtomicBool::new(false));

        CancelationToken { canceled }
    }

    /// Flips the state of the token to canceled, if it isn't already
    #[inline]
    pub fn cancel(&self) {
        self.canceled.store(true, Ordering::Release);
    }

    /// Checks if the token has been canceled
    #[inline]
    pub fn is_canceled(&self) -> bool {
        self.canceled.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let token = CancelationToken::new();

        assert!(!token.is_canceled(), "Token is was created canceled");
        let other_token = token.clone();
        token.cancel();
        assert!(
            token.is_canceled() && other_token.is_canceled(),
            "Token and clone(s) should be canceled"
        );
    }
}
