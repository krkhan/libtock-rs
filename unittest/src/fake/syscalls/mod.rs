mod command_impl;
mod raw_syscalls_impl;
mod yield_impl;

/// `fake::Syscalls` implements `libtock_platform::Syscalls` by forwarding the
/// system calls to the thread's `fake::Kernel` instance. It is used by unit
/// tests to provide the code under test access to Tock's system calls.
pub struct Syscalls;

#[cfg(test)]
mod command_impl_tests;
#[cfg(test)]
mod yield_impl_tests;

// Miri does not always check that values are valid (see `doc/MiriTips.md` in
// the root of this repository). This function uses a hack to verify a value is
// valid. If the value is invalid, Miri will detect undefined behavior when it
// executes this. It is used by submodules of fake::syscalls.
fn assert_valid<T: core::fmt::Debug>(_value: T) {
    #[cfg(miri)]
    format!("{:?}", _value);
}
