use crate::attributes::*;
use crate::error::*;

use anyhow::Result;
use subtle::ConstantTimeEq;

// check_size returns ErrAttrSizeInvalid if got is not equal to expected.
pub fn check_size(_at: AttrType, got: usize, expected: usize) -> Result<()> {
    if got == expected {
        Ok(())
    } else {
        Err(Error::ErrAttributeSizeInvalid.into())
    }
}

// is_attr_size_invalid returns true if error means that attribute size is invalid.
pub fn is_attr_size_invalid(err: &anyhow::Error) -> bool {
    Error::ErrAttributeSizeInvalid.equal(err)
}

pub(crate) fn check_hmac(got: &[u8], expected: &[u8]) -> Result<()> {
    if got.ct_eq(expected).unwrap_u8() != 1 {
        Err(Error::ErrIntegrityMismatch.into())
    } else {
        Ok(())
    }
}

pub(crate) fn check_fingerprint(got: u32, expected: u32) -> Result<()> {
    if got == expected {
        Ok(())
    } else {
        Err(Error::ErrFingerprintMismatch.into())
    }
}

// check_overflow returns ErrAttributeSizeOverflow if got is bigger that max.
pub fn check_overflow(_at: AttrType, got: usize, max: usize) -> Result<()> {
    if got <= max {
        Ok(())
    } else {
        Err(Error::ErrAttributeSizeOverflow.into())
    }
}

// is_attr_size_overflow returns true if error means that attribute size is too big.
pub fn is_attr_size_overflow(err: &anyhow::Error) -> bool {
    Error::ErrAttributeSizeOverflow.equal(err)
}
