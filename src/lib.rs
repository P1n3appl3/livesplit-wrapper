#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://github.com/LiveSplit.png")]

use std::mem::{self, MaybeUninit};
use std::slice;

pub use bytemuck::Pod;

/// Wires up the necessary c interface for a type that implements [`Splitter`].
///
/// If you defined `struct MySplitter {...}` and `impl Splitter for MySplitter
/// {...}` then you can write `register_autosplitter!(MySplitter);` and you'll
/// be good to go.
#[macro_export]
macro_rules! register_autosplitter {
    ($struct:ident) => {
        use std::cell::{Cell, RefCell};
        thread_local! {
            static SINGLETON: RefCell<$struct> = RefCell::default();
            static INITIALIZED: Cell<bool> = Cell::new(false);
        }
        #[no_mangle]
        pub extern "C" fn update() {
            if INITIALIZED.with(|id| id.get()) {
                SINGLETON.with(|s| s.borrow_mut().update());
            } else {
                SINGLETON.with(|s| s.replace($struct::new()));
            }
        }
    };
}

/// Currently the only possible error is a failed memory read on the attached
/// process.
#[derive(Debug)]
pub enum Error {
    /// A memory read on the attached process failed
    FailedRead,
}

/// The result of an attempt to read process memory.
pub type Result<T> = std::result::Result<T, Error>;

/// An address in the attached processes memory.
///
/// Autosplitters can attach to 32-bit processes, they'll just get an error if
/// they try to read outside it's address space.
pub type Address = u64;

/// A handle representing an attached process that can be used to read its
/// memory.
#[derive(Debug)]
pub struct Process(u64);

impl Process {
    /// Reads a single value from the attached processes memory space. To be
    /// able to use this with your own types, they need to implement [`Pod`]
    /// (it's implemented for the numeric types and fixed size arrays by
    /// default).
    pub fn read<T: Pod>(&self, addr: Address) -> Result<T> {
        unsafe {
            let mut buf = MaybeUninit::uninit();
            self.read_into_buf(
                addr,
                slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, mem::size_of::<T>()),
            )?;
            Ok(buf.assume_init())
        }
    }

    /// Search for a module (aka dynamic library) loaded by the attached process
    /// by name and return its base address.
    pub fn module(&self, name: &str) -> Option<Address> {
        unsafe {
            match ffi::process_get_module_address(self.0, name.as_ptr() as u32, name.len() as u32) {
                0 => None,
                n => Some(n),
            }
        }
    }

    /// Read bytes from the attached processes memory space starting at `addr`
    /// into `buf`.
    pub fn read_into_buf(&self, addr: Address, buf: &mut [u8]) -> Result<()> {
        unsafe {
            (ffi::process_read(self.0, addr, buf.as_mut_ptr() as u32, buf.len() as u32) != 0)
                .then_some(())
                .ok_or(Error::FailedRead)
        }
    }

    /// Reads a null terminated string starting at the given base address.
    /// Returns an `Error` if no null is encountered after 255 bytes or the
    /// bytes read are invalid unicode.
    pub fn read_cstr(&self, base: u64) -> Result<String> {
        const MAX_STR_LEN: usize = 256;
        let mut buf = vec![0u8; MAX_STR_LEN];
        unsafe {
            (ffi::process_read(
                self.0,
                base,
                buf.as_mut_ptr() as u32,
                MAX_STR_LEN as u32 - 1,
            ) != 0)
                .then_some(())
                .ok_or(Error::FailedRead)?;
        }
        buf.truncate(buf.iter().position(|&x| x == 0).expect("string too long") + 1);
        let cstr = std::ffi::CString::from_vec_with_nul(buf).expect("invalid unicode");
        Ok(cstr.to_string_lossy().to_string())
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            ffi::process_detach(self.0);
        }
    }
}

/// The main autosplitter trait.
///
/// This trait is the entry point for the autosplitter's functionality. The
/// `new` and `update` functions are hooks that will be called by LiveSplit. To
/// interact with LiveSplit's timer, use the functions defined in
/// [`HostFunctions`].
///
/// [`HostFunctions`] is automatically implemented on `Splitter`s, so in your
/// `update` function you can call methods like
/// [`self.split()`](HostFunctions::split) and
/// [`self.set_game_time()`](HostFunctions::set_game_time).
///
/// ## REMEMBER!
///
/// Make sure you use the [`register_autosplitter!`] macro on your splitter!
/// Without it, your wasm library won't expose the proper functions and it'll
/// fail to load.
pub trait Splitter {
    /// Called when the LiveSplit runtime instantiates your splitter. It's a
    /// good time to attach to a game process, set initial variables, or
    /// change your tick rate. Note that this _won't_ be called every time
    /// the run is reset.
    fn new() -> Self;

    /// Called periodically by the LiveSplit runtime. To change the rate that
    /// it's called, use [`set_tick_rate`](HostFunctions::set_tick_rate)
    fn update(&mut self);
}

/// The autosplitter's interface for interacting with the LiveSpilit timer.
pub trait HostFunctions {
    /// Output a message. This can be used for debugging and/or sending error
    /// messages to the player through whichever LiveSplit frontend they're
    /// using. Note that because autosplitters run in WASM, they don't have
    /// access to STDOUT or files, so typical solutions like `println!` and
    /// logging will not work (this could chagne in the future as LiveSplit
    /// plans to support WASI).
    fn print(&self, str: &str) {
        unsafe { ffi::runtime_print_message(str.as_ptr(), str.len()) }
    }

    /// Attach to a process running on the same machine as the autosplitter.
    fn attach(&self, name: &str) -> Option<Process> {
        unsafe {
            match ffi::process_attach(name.as_ptr() as u32, name.len() as u32) {
                0 => None,
                n => Some(Process(n)),
            }
        }
    }

    /// Start the timer for a run. Note that this will silently do nothing on
    /// subsequent calls. To start a new run, call `reset()` and _then_
    /// `start()`.
    fn start(&self) {
        unsafe { ffi::timer_start() }
    }

    /// Pause the game time counter. This is often used when entering a loading
    /// screen or end level screen for games that use in game time rather
    /// than real time. It may be a good idea to call `set_game_time()`
    /// immediately after pausing so that LiveSplit's game time counter
    /// shows the exact current time.
    fn pause(&self) {
        unsafe { ffi::timer_pause_game_time() }
    }

    /// Resume the game time counter. Note that
    fn unpause(&self) {
        unsafe { ffi::timer_resume_game_time() }
    }

    /// Mark the current split as finished and move to the next one.
    fn split(&self) {
        unsafe { ffi::timer_split() }
    }

    /// Reset the run. Don't do this automatically when a run has finished, and
    /// in general be conservative about resetting runs from the
    /// autosplitter. Common practice is to do so only if there's an
    /// unambiguous signal that the player is done with this run.
    fn reset(&self) {
        unsafe { ffi::timer_reset() }
    }

    /// Set the game time. Note that if the timer is not paused, the time shown
    /// will keep incrementing immediately after it is set to the given
    /// value.
    fn set_game_time(&self, time: f64) {
        unsafe { ffi::timer_set_game_time(time) }
    }

    /// Set the rate at which the [`update`](Splitter::update) function will be
    /// called (in Hz).
    fn set_tick_rate(&self, rate: f64) {
        unsafe { ffi::runtime_set_tick_rate(rate) }
    }

    /// Get the current state of the timer. This is how the autosplitter can
    /// detect if the player manually paused or reset a run.
    fn state(&self) -> TimerState {
        unsafe { std::mem::transmute(ffi::timer_get_state() as u8) }
    }

    /// Set a variable which can be displayed by LiveSplit. This is commonly
    /// used for features like death counters.
    fn set_variable(&self, key: &str, value: &str) {
        unsafe {
            ffi::timer_set_variable(
                key.as_ptr() as u32,
                key.len() as u32,
                value.as_ptr() as u32,
                value.len() as u32,
            );
        }
    }
}

impl<T: Splitter> HostFunctions for T {}

/// The possible states of the timer.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TimerState {
    /// The timer has yet to be started.
    NotRunning = 0,
    /// The timer is currently running.
    Running = 1,
    /// The timer is paused.
    Paused = 2,
    /// The timer is stopped because a run was completed.
    Ended = 3,
}

mod ffi {
    extern "C" {
        pub(crate) fn runtime_print_message(ptr: *const u8, len: usize);
        pub(crate) fn runtime_set_tick_rate(rate: f64);
        pub(crate) fn process_attach(ptr: u32, len: u32) -> u64;
        pub(crate) fn process_detach(handle: u64);
        pub(crate) fn process_get_module_address(handle: u64, ptr: u32, len: u32) -> u64;
        pub(crate) fn process_read(handle: u64, address: u64, buf: u32, buf_len: u32) -> u32;
        pub(crate) fn timer_start();
        pub(crate) fn timer_split();
        pub(crate) fn timer_reset();
        pub(crate) fn timer_set_variable(key: u32, key_len: u32, value: u32, value_len: u32);
        pub(crate) fn timer_set_game_time(time: f64);
        pub(crate) fn timer_pause_game_time();
        pub(crate) fn timer_resume_game_time();
        pub(crate) fn timer_get_state() -> u32;
    }
}
