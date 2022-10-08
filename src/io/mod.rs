//! Access to I/O ports
#![allow(dead_code)]


#[unstable(feature = "std3_io", issue = "none")]
#[allow(missing_docs)]
pub mod prelude;

#[doc(hidden)] mod __uart_16550;
#[doc(hidden)] mod __x86_64;



/// Different types of I/O errors
#[unstable(feature = "std3_io", issue = "none")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IOError {
    /// Failed to read from port.
    ReadFail,
    /// Failed to write to port.
    WriteFail,
}

impl crate::fmt::Display for IOError {
    fn fmt(&self, f: &mut crate::fmt::Formatter) -> crate::fmt::Result {
        match self {
            IOError::ReadFail => write!(f, "IOERROR: Failed to read from port."),
            IOError::WriteFail => write!(f, "IOERROR: Failed to write to port."),
        }
    }
}


/// Result of some operation.
#[unstable(feature = "std3_io", issue = "none")]
pub type Result<T> = crate::result::Result<T,IOError>;

#[unstable(feature = "std3_io", issue = "none")]
/// Reads from a port.
pub unsafe fn read_from_port(port: u16) -> Result<u8> {
    #[cfg(target_arch = "x86_64")]
    return __x86_64::read(port).get_data(IOError::ReadFail);

    #[cfg(not(target_arch = "x86_64"))]
    return __uart_16550::read(port).get_data(IOError::ReadFail);
}

#[unstable(feature = "std3_io", issue = "none")]
/// Writes to a port
pub unsafe fn write_to_port(port: u16, data: u8) -> Result<()> {
    #[cfg(target_arch = "x86_64")]
    return __x86_64::write(port, data).get_data(IOError::WriteFail);

    #[cfg(not(target_arch = "x86_64"))]
    return __uart_16550::write(port, data).get_data(IOError::WriteFail);
}


#[doc(hidden)]
trait GetData {
    type Target;
    fn get_data(&self, error: IOError) -> Result<Self::Target>;
}
#[doc(hidden)]
impl GetData for (u8,bool) {
    type Target = u8;
    fn get_data(&self, error: IOError) -> Result<Self::Target> {
        match self.1 {
            true => Ok(self.0),
            false => Err(error)
        }
    }
}
#[doc(hidden)]
impl GetData for bool {
    type Target = ();
    fn get_data(&self, error: IOError) -> Result<Self::Target> {
        match self {
            true => Ok(()),
            false => Err(error)
        }
    }
}