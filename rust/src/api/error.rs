pub enum SplitError {
    /// No data found
    Empty,

    /// Cannot make the data fit
    CannotFit,

    /// Max split size is too large
    MaxSplitSizeTooLarge(usize),

    /// Min split size is too small
    MinSplitTooSmall,

    /// Invalid split min and max range, min is larger than max
    InvalidSplitRange,

    /// Invalid version min and max range, min is larger than max
    InvalidVersionRange,

    /// Error while encoding
    EncodeError(EncodeError),
}

pub enum EncodeError {
    /// No data to encode
    Empty,

    /// Error while compressing data
    CompressionError(String),
}

impl From<bbqr::error::SplitError> for SplitError {
    fn from(e: bbqr::error::SplitError) -> Self {
        match e {
            bbqr::error::SplitError::Empty => SplitError::Empty,
            bbqr::error::SplitError::CannotFit => SplitError::CannotFit,
            bbqr::error::SplitError::MaxSplitSizeTooLarge(size) => {
                SplitError::MaxSplitSizeTooLarge(size)
            }
            bbqr::error::SplitError::MinSplitTooSmall => SplitError::MinSplitTooSmall,
            bbqr::error::SplitError::InvalidSplitRange => SplitError::InvalidSplitRange,
            bbqr::error::SplitError::InvalidVersionRange => SplitError::InvalidVersionRange,
            bbqr::error::SplitError::EncodeError(e) => SplitError::EncodeError(e.into()),
        }
    }
}

impl From<bbqr::error::EncodeError> for EncodeError {
    fn from(e: bbqr::error::EncodeError) -> Self {
        match e {
            bbqr::error::EncodeError::Empty => EncodeError::Empty,
            bbqr::error::EncodeError::CompressionError(e) => EncodeError::CompressionError(e),
        }
    }
}
