//! Flag definitions

use bitflags::bitflags;
use core::convert::TryFrom;

//=============================================================================
// Interrupt flags
//=============================================================================
const DRE: u8 = 0x01;
const TXC: u8 = 0x02;
const RXC: u8 = 0x04;
const RXS: u8 = 0x08;

/// Interrupt flags available for RX transactions
pub const RX_FLAG_MASK: u8 = RXC | RXS;
/// Interrupt flags available for TX transactions
pub const TX_FLAG_MASK: u8 = DRE | TXC;
/// Interrupt flags available for Duplex transactions
pub const DUPLEX_FLAG_MASK: u8 = RX_FLAG_MASK | TX_FLAG_MASK;

bitflags! {
    /// Interrupt bit flags for UART Rx transactions
    ///
    /// The available interrupt flags are `DRE`, `TXC`, `RXC`, `RXS`, `CTSIC`, `RXBRK` and
    /// `ERROR`. The binary format of the underlying bits exactly matches the
    /// INTFLAG bits.
    pub struct Flags: u8 {
        const DRE = DRE;
        const TXC = TXC;
        const RXC = RXC;
        const RXS = RXS ;
    }
}

//=============================================================================
// Status flags
//=============================================================================

const PERR: u16 = 0x01;
const FERR: u16 = 0x02;
const BUFOVF: u16 = 0x04;
const SYNCBUSY: u16 = 0x8000;

/// Status flags available for RX transactions
pub const RX_STATUS_MASK: u16 = PERR | FERR | BUFOVF;
/// Status flags available for Duplex transactions
pub const DUPLEX_STATUS_MASK: u16 = RX_STATUS_MASK;

bitflags! {
    /// Status flags for UART Rx transactions
    ///
    /// The available status flags are `PERR`, `FERR`, `BUFOVF`,
    /// `CTS`, `ISF` and `COLL`.
    /// The binary format of the underlying bits exactly matches
    /// the STATUS bits.
    pub struct Status: u16 {
        const PERR = PERR;
        const FERR = FERR;
        const BUFOVF = BUFOVF;
        const SYNCBUSY = SYNCBUSY;
    }
}

//=============================================================================
// Error
//=============================================================================

/// Errors available for UART transactions
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Detected a parity error
    ParityError,
    /// Detected a frame error
    FrameError,
    /// Detected a buffer overflow
    Overflow,
}

impl TryFrom<Status> for () {
    type Error = Error;

    #[inline]
    fn try_from(errors: Status) -> Result<(), Error> {
        use Error::*;
        if errors.contains(Status::PERR) {
            Err(ParityError)
        } else if errors.contains(Status::FERR) {
            Err(FrameError)
        } else if errors.contains(Status::BUFOVF) {
            Err(Overflow)
        } else {
            Ok(())
        }
    }
}

impl From<Error> for Status {
    #[inline]
    fn from(err: Error) -> Self {
        use Error::*;
        match err {
            ParityError => Status::PERR,
            FrameError => Status::FERR,
            Overflow => Status::BUFOVF,
        }
    }
}
