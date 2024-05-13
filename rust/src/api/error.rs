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

pub enum JoinError {
    /// No data found
    Empty,

    /// Conflicting/variable file type/encodings/sizes
    ConflictingHeaders,

    /// Too many parts
    TooManyParts(usize, usize),

    /// Duplicate part with different content
    DuplicatePartWrongContent(usize),

    /// Part with no data
    PartWithNoData(usize),

    /// Missing part
    MissingPart(usize),

    /// Header parse error
    HeaderParseError(HeaderParseError),

    /// Decode error
    DecodeError(DecodeError),
}

pub enum HeaderParseError {
    /// No data found
    Empty,

    /// Invalid encoding
    InvalidEncoding(String),

    /// Invalid file type
    InvalidFileType(String),

    /// Invalid fixed header
    InvalidFixedHeader,

    /// Invalid header size, not long enough
    InvalidHeaderSize(usize),

    /// Invalid header parts, not enough parts
    InvalidHeaderParts(String),
}

pub enum DecodeError {
    /// Unable to decode hex part
    UnableToDecodeHex(usize, String),

    /// Unable to decode base32 part
    UnableToDecodeBase32(usize, String),

    /// Unable to decompress zlib data
    UnableToInflateZlib(String),
}

impl From<bbqr::error::HeaderParseError> for HeaderParseError {
    fn from(e: bbqr::error::HeaderParseError) -> Self {
        match e {
            bbqr::error::HeaderParseError::Empty => HeaderParseError::Empty,
            bbqr::error::HeaderParseError::InvalidEncoding(c) => {
                HeaderParseError::InvalidEncoding(c.to_string())
            }
            bbqr::error::HeaderParseError::InvalidFileType(c) => {
                HeaderParseError::InvalidFileType(c.to_string())
            }
            bbqr::error::HeaderParseError::InvalidFixedHeader => {
                HeaderParseError::InvalidFixedHeader
            }
            bbqr::error::HeaderParseError::InvalidHeaderSize(size) => {
                HeaderParseError::InvalidHeaderSize(size)
            }
            bbqr::error::HeaderParseError::InvalidHeaderParts(s) => {
                HeaderParseError::InvalidHeaderParts(s)
            }
        }
    }
}

impl From<bbqr::error::DecodeError> for DecodeError {
    fn from(e: bbqr::error::DecodeError) -> Self {
        match e {
            bbqr::error::DecodeError::UnableToDecodeHex(a, b) => {
                DecodeError::UnableToDecodeHex(a, b.to_string())
            }
            bbqr::error::DecodeError::UnableToDecodeBase32(a, b) => {
                DecodeError::UnableToDecodeBase32(a, b.to_string())
            }
            bbqr::error::DecodeError::UnableToInflateZlib(s) => DecodeError::UnableToInflateZlib(s),
        }
    }
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

impl From<bbqr::error::JoinError> for JoinError {
    fn from(e: bbqr::error::JoinError) -> Self {
        match e {
            bbqr::error::JoinError::Empty => JoinError::Empty,
            bbqr::error::JoinError::ConflictingHeaders => JoinError::ConflictingHeaders,
            bbqr::error::JoinError::TooManyParts(a, b) => JoinError::TooManyParts(a, b),
            bbqr::error::JoinError::DuplicatePartWrongContent(a) => {
                JoinError::DuplicatePartWrongContent(a)
            }
            bbqr::error::JoinError::PartWithNoData(a) => JoinError::PartWithNoData(a),
            bbqr::error::JoinError::MissingPart(a) => JoinError::MissingPart(a),
            bbqr::error::JoinError::HeaderParseError(e) => JoinError::HeaderParseError(e.into()),
            bbqr::error::JoinError::DecodeError(e) => JoinError::DecodeError(e.into()),
        }
    }
}
