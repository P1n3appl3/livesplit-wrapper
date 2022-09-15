use std::mem::{self, MaybeUninit};
use std::slice;

pub use bytemuck::Pod;

use super::ffi;

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
pub struct Process(pub(crate) u64);

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
    /// Returns an `Error` on a failed read, and panics if no null is
    /// encountered after 255 bytes or the bytes read are invalid unicode.
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
