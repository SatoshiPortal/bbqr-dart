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

pub enum ContinuousJoinError {
    HeaderParseError(HeaderParseError),
    JoinError(JoinError),
    DecodeError(DecodeError),
}

impl From<bbqr::header::HeaderParseError> for HeaderParseError {
    fn from(e: bbqr::header::HeaderParseError) -> Self {
        match e {
            bbqr::header::HeaderParseError::Empty => HeaderParseError::Empty,
            bbqr::header::HeaderParseError::InvalidEncoding(c) => {
                HeaderParseError::InvalidEncoding(c.to_string())
            }
            bbqr::header::HeaderParseError::InvalidFileType(c) => {
                HeaderParseError::InvalidFileType(c.to_string())
            }
            bbqr::header::HeaderParseError::InvalidFixedHeader => {
                HeaderParseError::InvalidFixedHeader
            }
            bbqr::header::HeaderParseError::InvalidHeaderSize(size) => {
                HeaderParseError::InvalidHeaderSize(size)
            }
            bbqr::header::HeaderParseError::InvalidHeaderParts(s) => {
                HeaderParseError::InvalidHeaderParts(s)
            }
        }
    }
}

impl From<bbqr::decode::DecodeError> for DecodeError {
    fn from(e: bbqr::decode::DecodeError) -> Self {
        match e {
            bbqr::decode::DecodeError::UnableToDecodeHex(a, b) => {
                DecodeError::UnableToDecodeHex(a, b.to_string())
            }
            bbqr::decode::DecodeError::UnableToDecodeBase32(a, b) => {
                DecodeError::UnableToDecodeBase32(a, b.to_string())
            }
            bbqr::decode::DecodeError::UnableToInflateZlib(s) => {
                DecodeError::UnableToInflateZlib(s)
            }
        }
    }
}

impl From<bbqr::split::SplitError> for SplitError {
    fn from(e: bbqr::split::SplitError) -> Self {
        match e {
            bbqr::split::SplitError::Empty => SplitError::Empty,
            bbqr::split::SplitError::CannotFit => SplitError::CannotFit,
            bbqr::split::SplitError::MaxSplitSizeTooLarge(size) => {
                SplitError::MaxSplitSizeTooLarge(size)
            }
            bbqr::split::SplitError::MinSplitTooSmall => SplitError::MinSplitTooSmall,
            bbqr::split::SplitError::InvalidSplitRange => SplitError::InvalidSplitRange,
            bbqr::split::SplitError::InvalidVersionRange => SplitError::InvalidVersionRange,
            bbqr::split::SplitError::EncodeError(e) => SplitError::EncodeError(e.into()),
        }
    }
}

impl From<bbqr::encode::EncodeError> for EncodeError {
    fn from(e: bbqr::encode::EncodeError) -> Self {
        match e {
            bbqr::encode::EncodeError::Empty => EncodeError::Empty,
            bbqr::encode::EncodeError::CompressionError(e) => EncodeError::CompressionError(e),
        }
    }
}

impl From<bbqr::join::JoinError> for JoinError {
    fn from(e: bbqr::join::JoinError) -> Self {
        match e {
            bbqr::join::JoinError::Empty => JoinError::Empty,
            bbqr::join::JoinError::ConflictingHeaders => JoinError::ConflictingHeaders,
            bbqr::join::JoinError::TooManyParts(a, b) => JoinError::TooManyParts(a, b),
            bbqr::join::JoinError::DuplicatePartWrongContent(a) => {
                JoinError::DuplicatePartWrongContent(a)
            }
            bbqr::join::JoinError::PartWithNoData(a) => JoinError::PartWithNoData(a),
            bbqr::join::JoinError::MissingPart(a) => JoinError::MissingPart(a),
            bbqr::join::JoinError::HeaderParseError(e) => JoinError::HeaderParseError(e.into()),
            bbqr::join::JoinError::DecodeError(e) => JoinError::DecodeError(e.into()),
        }
    }
}

impl From<bbqr::continuous_join::ContinuousJoinError> for ContinuousJoinError {
    fn from(e: bbqr::continuous_join::ContinuousJoinError) -> Self {
        match e {
            bbqr::continuous_join::ContinuousJoinError::HeaderParseError(e) => {
                ContinuousJoinError::HeaderParseError(e.into())
            }
            bbqr::continuous_join::ContinuousJoinError::JoinError(e) => {
                ContinuousJoinError::JoinError(e.into())
            }
            bbqr::continuous_join::ContinuousJoinError::DecodeError(e) => {
                ContinuousJoinError::DecodeError(e.into())
            }
        }
    }
}
