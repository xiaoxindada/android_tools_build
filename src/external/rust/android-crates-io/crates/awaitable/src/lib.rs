#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use cfg_if::cfg_if;
use std::{mem, task::Waker};

cfg_if! {
    if #[cfg(feature = "parking_lot")] {
        use parking_lot::{const_mutex, Mutex};
    } else {
        use std::sync::{Mutex as StdMutex, MutexGuard};

        #[derive(Debug)]
        #[repr(transparent)]
        struct Mutex<T>(StdMutex<T>);

        impl<T> Mutex<T> {
            fn new(val: T) -> Self {
                Self(StdMutex::new(val))
            }

            #[track_caller]
            fn lock(&self) -> MutexGuard<'_, T> {
                self.0.lock().unwrap()
            }
        }
    }
}

pub use awaitable_error::Error;

#[derive(Debug)]
enum InnerState<Input, Output> {
    Uninitialized,

    Ongoing(Option<Input>, Option<Waker>),

    /// The awaitable is done
    Done(Output),

    Consumed,
}

/// Awaitable guarantees that there is no spurious wakeup
#[derive(Debug)]
pub struct Awaitable<Input, Output>(Mutex<InnerState<Input, Output>>);

impl<Input, Output> Default for Awaitable<Input, Output> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Input, Output> Awaitable<Input, Output> {
    /// Create an uninitialized `Awaitable`.
    ///
    /// Must be `reset` before it can be used.
    pub fn new() -> Self {
        Self(Mutex::new(InnerState::Uninitialized))
    }

    /// Create an uninitialized `Awaitable`.
    ///
    /// Must be `reset` before it can be used.
    #[cfg(feature = "parking_lot")]
    pub const fn const_new() -> Self {
        Self(const_mutex(InnerState::Uninitialized))
    }
}

impl<Input, Output> Awaitable<Input, Output> {
    /// Reset `Awaitable` to its initial state.
    ///
    /// After this call, `install_waker`, `take_input` and `done`
    /// can be called.
    pub fn reset(&self, input: Option<Input>) {
        *self.0.lock() = InnerState::Ongoing(input, None);
    }

    /// Return true if the task is already done.
    ///
    /// **
    /// `install_waker` must not be called after `take_output` is called.
    /// **
    pub fn install_waker(&self, waker: Waker) -> Result<bool, Error> {
        use InnerState::*;

        let mut guard = self.0.lock();

        match &mut *guard {
            Uninitialized => Err(Error::Uninitialized),

            Ongoing(_input, stored_waker) => {
                *stored_waker = Some(waker);
                Ok(false)
            }
            Done(_) => Ok(true),
            Consumed => Err(Error::AlreadyConsumed),
        }
    }

    /// **`take_input` must not be called after `take_output` is called.
    pub fn take_input(&self) -> Result<Option<Input>, Error> {
        use InnerState::*;

        let mut guard = self.0.lock();

        match &mut *guard {
            Uninitialized => Err(Error::Uninitialized),

            Ongoing(input, _stored_waker) => Ok(input.take()),
            Done(_) => Ok(None),
            Consumed => Err(Error::AlreadyConsumed),
        }
    }

    /// `done` must be only called once on one instance of `Awaitable`.
    ///
    /// **`done` must not be called after `take_output` is called.**
    pub fn done(&self, value: Output) -> Result<(), Error> {
        use InnerState::*;

        let prev_state = mem::replace(&mut *self.0.lock(), Done(value));

        match prev_state {
            Uninitialized => Err(Error::Uninitialized),

            Done(_) => Err(Error::AlreadyDone),
            Ongoing(_input, stored_waker) => {
                if let Some(waker) = stored_waker {
                    waker.wake();
                }

                Ok(())
            }
            Consumed => Err(Error::AlreadyConsumed),
        }
    }

    /// Return `Some(output)` if the awaitable is done.
    pub fn take_output(&self) -> Option<Output> {
        use InnerState::*;

        let prev_state = mem::replace(&mut *self.0.lock(), Consumed);

        match prev_state {
            Done(value) => Some(value),
            _ => None,
        }
    }

    /// Return true if current state is `Done`.
    pub fn is_done(&self) -> bool {
        matches!(&*self.0.lock(), InnerState::Done(_))
    }

    /// Return true if current state is `Consumed`.
    pub fn is_consumed(&self) -> bool {
        matches!(&*self.0.lock(), InnerState::Consumed)
    }
}
