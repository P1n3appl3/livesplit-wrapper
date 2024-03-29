#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://github.com/LiveSplit.png")]

mod process;
use std::time::Duration;

pub use once_cell::sync::OnceCell;
pub use process::{Address, Error, Pod, Process, Result};

use log::{Level, Metadata, Record};

/// This logger gets initialized automatically when you register an autosplitter
/// and emits logs to LiveSplit's autosplitter runtime.
pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // TODO: fixed size formatter to avoid alloc?
            let s = match record.level() {
                Level::Info => format!("{}", record.args()),
                Level::Warn => format!("⚠️ {}", record.args()),
                Level::Error => format!("⛔ {}", record.args()),
                _ => unimplemented!(),
            };
            unsafe { ffi::runtime_print_message(s.as_ptr(), s.len()) }
        }
    }

    fn flush(&self) {}
}

/// Wires up the necessary c interface for a type that implements [`Splitter`].
///
/// If you defined `struct MySplitter {...}` and `impl Splitter for MySplitter
/// {...}` then you can write `register_autosplitter!(MySplitter);` and you'll
/// be good to go.
#[macro_export]
macro_rules! register_autosplitter {
    ($struct:ident) => {
        use std::panic;
        // TODO: make sure mutex is a nop in wasm
        use std::sync::Mutex;

        use $crate::{Logger, OnceCell};

        const LOGGER: Logger = Logger;
        static SINGLETON: OnceCell<Mutex<$struct>> = OnceCell::new();

        #[no_mangle]
        pub extern "C" fn update() {
            SINGLETON
                .get_or_init(|| {
                    log::set_logger(&LOGGER)
                        .map(|()| log::set_max_level(log::LevelFilter::Info))
                        .ok();
                    panic::set_hook(Box::new(|panic_info| {
                        if let Some(location) = panic_info.location() {
                            log::error!(
                                "panic occurred in file '{}' at line {}",
                                location.file(),
                                location.line(),
                            );
                        } else {
                            log::error!("panic occurred but can't get location information...");
                        }
                    }));
                    Mutex::new($struct::new())
                })
                .lock()
                .unwrap()
                .update();
        }
    };
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
    /// screen or end level screen for games that use in-game time rather
    /// than real time. It may be a good idea to call `set_game_time()`
    /// immediately after pausing so that LiveSplit's game time counter
    /// shows the exact current time.
    fn pause(&self) {
        unsafe { ffi::timer_pause_game_time() }
    }

    /// Resume the game time counter.
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
    fn set_game_time(&self, time: Duration) {
        unsafe { ffi::timer_set_game_time(time.as_secs() as i64, time.subsec_nanos() as i32) }
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
        pub(crate) fn timer_set_game_time(seconds: i64, nanos: i32);
        pub(crate) fn timer_pause_game_time();
        pub(crate) fn timer_resume_game_time();
        pub(crate) fn timer_get_state() -> u32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default, Clone, Copy)]
    struct Unit;

    impl Splitter for Unit {
        fn new() -> Self {
            todo!()
        }

        fn update(&mut self) {
            todo!()
        }
    }

    register_autosplitter!(Unit);
}
