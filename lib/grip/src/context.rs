//! Unit-testing utilities for Stylus contracts.
use std::sync::{Mutex, MutexGuard};

use crate::storage::reset_storage;

/// A global static mutex.
///
/// We use this for scenarios where concurrent mutation of storage is wanted.
/// For example, when a test harness is running, this ensures each test
/// accesses storage in an non-overlapping manner.
///
/// See [`with_context`].
pub(crate) static STORAGE_MUTEX: Mutex<()> = Mutex::new(());

/// Acquires access to storage.
pub(crate) fn acquire_storage() -> MutexGuard<'static, ()> {
    STORAGE_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

/// The default `msg::sender` - `0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF`.
///
/// `msg::sender` is reset to this value after each `#[grip::test]` finishes.
pub const DEFAULT_MSG_SENDER: [u8; 20] = [
    222, 173, 190, 239, 222, 173, 190, 239, 222, 173, 190, 239, 222, 173, 190,
    239, 222, 173, 190, 239,
];

/// A global static mutex that holds the `msg::sender`.
///
/// We use this for scenarios where overriding `msg::sender` is needed.
pub(crate) static MSG_SENDER_MUTEX: Mutex<[u8; 20]> =
    Mutex::new(DEFAULT_MSG_SENDER);

/// Retrieves the current `msg::sender`.
pub(crate) fn get_msg_sender() -> [u8; 20] {
    *MSG_SENDER_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

/// Resets the value of `msg::sender` to [`DEFAULT_MSG_SENDER`].
pub(crate) fn reset_msg_sender() {
    set_msg_sender(&DEFAULT_MSG_SENDER);
}

/// Sets `msg::sender` to `value`.
///
/// Note that after calling this function, all further calls to `msg::sender`
/// will return the value passed until this function is called again. Most
/// notably, in tests, this value is reset after each `#[grip::test]` finishes
/// to `0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF`.
pub fn set_msg_sender(value: &[u8; 20]) {
    let mut guard = MSG_SENDER_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    guard.copy_from_slice(value.as_ref());
}

/// Decorates a closure by running it with exclusive access to storage.
pub fn with_context<C: Default>(closure: impl FnOnce(&mut C)) {
    let _lock = acquire_storage();
    let mut contract = C::default();
    closure(&mut contract);
    reset_msg_sender();
    reset_storage();
}
